//! 四柱排盤。

use crate::astro::{
    apparent_solar_longitude, delta_t, from_jd, jie_jd_ut, to_jd, DateTime, JIE,
};
use crate::ganzhi::{GanZhi, Zhi};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn name(self) -> &'static str {
        match self {
            Gender::Male => "乾造",
            Gender::Female => "坤造",
        }
    }
}

/// 排盤選項。
#[derive(Debug, Clone, Copy)]
pub struct Options {
    /// 時區偏移，單位小時。台灣、中國大陸為 8.0。
    pub tz: f64,
    /// 23 時起算次日子時。傳統多數派為 true。
    pub late_zi_next_day: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { tz: 8.0, late_zi_next_day: true }
    }
}

/// 四柱與相關時刻。
#[derive(Debug, Clone)]
pub struct Chart {
    pub input: DateTime,
    pub gender: Gender,
    pub options: Options,
    /// 出生瞬間的世界時儒略日。
    pub jd_ut: f64,
    pub year: GanZhi,
    pub month: GanZhi,
    pub day: GanZhi,
    pub hour: GanZhi,
    /// 年柱所屬之年（立春換年）。
    pub solar_year: i32,
    /// 月支序，0 為寅月。
    pub month_index: usize,
    /// 本月節之名，及其本地時刻。
    pub jie_name: &'static str,
    pub jie_time: DateTime,
    /// 下一個節之名與本地時刻。
    pub next_jie_name: &'static str,
    pub next_jie_time: DateTime,
    /// 生時距本月節之日數，含小數。
    pub days_into_jie: f64,
}

impl Chart {
    pub fn new(input: DateTime, gender: Gender, options: Options) -> Chart {
        let jd_ut = to_jd(&input) - options.tz / 24.0;

        let (solar_year, month_index) = locate_solar_month(jd_ut, input.year);
        let bounds = month_bounds(solar_year);

        let year_gz = GanZhi::from_index(solar_year as i64 - 4);

        let month_zhi = ((month_index + 2) % 12) as u8;
        let month_gan = ((year_gz.gan().0 % 5) * 2 + 2 + month_index as u8) % 10;
        let month_gz = GanZhi::from_gan_zhi(month_gan, month_zhi)
            .expect("月柱干支必成對");

        let day_gz = day_pillar(&input, options.late_zi_next_day);

        let zi_index = ((input.hour + 1) / 2 % 12) as u8;
        let hour_gan = ((day_gz.gan().0 % 5) * 2 + zi_index) % 10;
        let hour_gz = GanZhi::from_gan_zhi(hour_gan, zi_index).expect("時柱干支必成對");

        let to_local = |jd: f64| from_jd(jd + options.tz / 24.0);

        Chart {
            input,
            gender,
            options,
            jd_ut,
            year: year_gz,
            month: month_gz,
            day: day_gz,
            hour: hour_gz,
            solar_year,
            month_index,
            jie_name: JIE[month_index].0,
            jie_time: to_local(bounds[month_index]),
            next_jie_name: JIE[(month_index + 1) % 12].0,
            next_jie_time: to_local(bounds[month_index + 1]),
            days_into_jie: jd_ut - bounds[month_index],
        }
    }

    /// 日干，即命主。
    pub fn day_master(&self) -> crate::ganzhi::Gan {
        self.day.gan()
    }

    pub fn pillars(&self) -> [GanZhi; 4] {
        [self.year, self.month, self.day, self.hour]
    }

    /// 出生當下的太陽視黃經，度。
    pub fn solar_longitude(&self) -> f64 {
        apparent_solar_longitude(self.jd_ut + delta_t(self.input.year as f64) / 86400.0)
    }
}

/// 年柱之年的十三個節界：索引 0..11 為本年寅月至丑月之節，索引 12 為次年立春。
pub fn month_bounds(solar_year: i32) -> [f64; 13] {
    let mut b = [0.0f64; 13];
    for i in 0..12 {
        b[i] = jie_jd_ut(solar_year, i);
    }
    b[12] = jie_jd_ut(solar_year + 1, 0);
    b
}

/// 某時刻所屬之干支年，立春為界。`tz` 為該時刻所用之時區偏移，單位小時。
///
/// 年柱、大運交運、流年三處同用此界，故收於一處。原先大運與流年各寫一份，
/// 大運那份只在首步查表、其後加十推算——立春在二月三日至五日之間游移，
/// 交運時刻卻固定，二線交錯，加十必錯。同一件事寫兩遍正是漏改的溫床。
pub fn solar_year_of(dt: &DateTime, tz: f64) -> i32 {
    let t = to_jd(dt) - tz / 24.0;
    if t < jie_jd_ut(dt.year, 0) {
        dt.year - 1
    } else {
        dt.year
    }
}

/// 由世界時儒略日定出年柱之年與月支序。
fn locate_solar_month(jd_ut: f64, civil_year: i32) -> (i32, usize) {
    for cand in [civil_year, civil_year - 1, civil_year + 1] {
        let b = month_bounds(cand);
        if jd_ut >= b[0] && jd_ut < b[12] {
            let idx = (0..12).rev().find(|&i| jd_ut >= b[i]).unwrap();
            return (cand, idx);
        }
    }
    unreachable!("節氣區間必涵蓋任一時刻");
}

/// 日柱。以本地日期取儒略日序，1900-01-01 為甲戌日。
fn day_pillar(local: &DateTime, late_zi_next_day: bool) -> GanZhi {
    let noon = DateTime::new(local.year, local.month, local.day, 12, 0, 0);
    let mut jdn = to_jd(&noon) as i64;
    if late_zi_next_day && local.hour >= 23 {
        jdn += 1;
    }
    GanZhi::from_index(jdn + 49)
}

/// 地支對應的時辰範圍描述，供輸出使用。
pub fn hour_range(zhi: Zhi) -> String {
    let start = (zhi.0 as i32 * 2 + 23) % 24;
    let end = (start + 2) % 24;
    format!("{start:02}:00–{end:02}:00")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chart(y: i32, m: u32, d: u32, h: u32, mi: u32) -> Chart {
        Chart::new(
            DateTime::new(y, m, d, h, mi, 0),
            Gender::Male,
            Options::default(),
        )
    }

    #[test]
    fn day_pillar_anchors() {
        // 1900-01-01 甲戌、2000-01-01 戊午
        assert_eq!(day_pillar(&DateTime::new(1900, 1, 1, 12, 0, 0), true).name(), "甲戌");
        assert_eq!(day_pillar(&DateTime::new(2000, 1, 1, 12, 0, 0), true).name(), "戊午");
    }

    #[test]
    fn late_zi_rolls_to_next_day() {
        let a = day_pillar(&DateTime::new(2000, 1, 1, 23, 30, 0), true);
        let b = day_pillar(&DateTime::new(2000, 1, 2, 1, 0, 0), true);
        assert_eq!(a.name(), b.name());
        assert_eq!(day_pillar(&DateTime::new(2000, 1, 1, 23, 30, 0), false).name(), "戊午");
    }

    #[test]
    fn year_changes_at_lichun_not_new_year() {
        // 2024 立春在 2 月 4 日。1 月 20 日仍屬癸卯年。
        assert_eq!(chart(2024, 1, 20, 10, 0).year.name(), "癸卯");
        assert_eq!(chart(2024, 2, 10, 10, 0).year.name(), "甲辰");
        assert_eq!(chart(1984, 3, 1, 10, 0).year.name(), "甲子");
    }

    #[test]
    fn month_pillar_follows_wuhudun() {
        // 甲年寅月起丙寅
        let c = chart(1984, 2, 20, 10, 0);
        assert_eq!(c.year.name(), "甲子");
        assert_eq!(c.month.name(), "丙寅");
        assert_eq!(c.month_index, 0);
    }

    #[test]
    fn hour_pillar_follows_wushudun() {
        // 日干甲，子時起甲子
        let c = chart(2000, 1, 1, 10, 0); // 戊午日
        assert_eq!(c.day.name(), "戊午");
        // 戊日巳時：戊癸起壬子，巳為第 5 位，壬+5 = 丁
        assert_eq!(c.hour.name(), "丁巳");
    }

    #[test]
    fn month_index_covers_every_pillar() {
        // 逐月抽樣，月支應依寅卯辰…順行
        for (i, (m, d)) in [
            (2, 20), (3, 20), (4, 20), (5, 20), (6, 20), (7, 20),
            (8, 20), (9, 20), (10, 20), (11, 20), (12, 20), (1, 20),
        ]
        .iter()
        .enumerate()
        {
            let y = if i == 11 { 2001 } else { 2000 };
            let c = chart(y, *m, *d, 12, 0);
            assert_eq!(c.month_index, i, "{y}-{m}-{d} 月支序不符");
            assert_eq!(c.month.zhi().0, ((i + 2) % 12) as u8);
        }
    }
}
