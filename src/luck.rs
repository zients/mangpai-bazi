//! 大運與小運推算。

use crate::astro::{from_jd, to_jd, DateTime};
use crate::chart::{month_bounds, solar_year_of, Chart, Gender};
use crate::ganzhi::GanZhi;

/// 行運方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// 順行：陽年男、陰年女。
    Forward,
    /// 逆行：陰年男、陽年女。
    Backward,
}

impl Direction {
    pub fn name(self) -> &'static str {
        match self {
            Direction::Forward => "順行",
            Direction::Backward => "逆行",
        }
    }

    pub fn sign(self) -> i64 {
        match self {
            Direction::Forward => 1,
            Direction::Backward => -1,
        }
    }
}

/// 一步大運。
#[derive(Debug, Clone)]
pub struct DaYun {
    /// 第幾步，自 1 起算。
    pub ordinal: u32,
    pub pillar: GanZhi,
    /// 交運時的虛歲。
    pub start_age: u32,
    /// 交運時刻（本地時）。
    pub start_time: DateTime,
    /// 本步所轄之首個干支年，即 `year_range.0`。
    pub start_year: i32,
    /// 本步所轄之干支年區間，含頭含尾。
    ///
    /// 大運為整整十年，自交運至交運一秒不差；但交運不落在立春上，
    /// 兩端各切去數小時，投影到立春格子後可得九年、十年或十一年。
    /// 此區間即該投影：自交運後第一個立春起，至次步交運前最後一個立春止，
    /// 逐步相接，不重不漏，與 [`crate::liunian::fortune_at`] 所判者一致。
    pub year_range: (i32, i32),
}

/// 一年小運。
#[derive(Debug, Clone)]
pub struct XiaoYun {
    /// 虛歲。
    pub age: u32,
    pub pillar: GanZhi,
    /// 本歲所屬之干支年。
    pub year: i32,
}

/// 某虛歲之小運。自時柱起，一年一位，方向同大運。
///
/// 小運終身不斷，起運後仍逐年在走，只是論命多以大運流年為主而不觀。
/// 故此處為公式，不為查表——查表必在表末生出缺口，該缺口曾使
/// 交運當年答出錯一步的柱。
pub fn xiao_yun_at(chart: &Chart, direction: Direction, age: u32) -> XiaoYun {
    XiaoYun {
        age,
        pillar: chart.hour.step(direction.sign() * age as i64),
        year: chart.solar_year + age as i32 - 1,
    }
}

/// 完整行運資訊。
#[derive(Debug, Clone)]
pub struct LuckCycle {
    pub direction: Direction,
    /// 起運距生時的實際日數。
    pub span_days: f64,
    /// 起運年、月、日。三日折一年，一日折四月，一時折五日。
    pub start_years: u32,
    pub start_months: u32,
    pub start_days: u32,
    /// 起運（交運）時刻，本地時。
    pub start_time: DateTime,
    pub da_yun: Vec<DaYun>,
    /// 小運，自虛歲一至起運之歲，逐歲一柱。
    ///
    /// 小運本身終身不斷，此處只列至起運為止——盲派論命不觀小運，
    /// 排滿百二十列徒然洗版。末列即交運所在之虛歲，只走到交運為止，
    /// 其後由 `da_yun` 之首步接手，兩表相接不留缺口。
    /// 要取任一虛歲之小運，用 [`xiao_yun_at`]。
    pub xiao_yun: Vec<XiaoYun>,
}

/// 推算行運。`count` 為大運步數。
pub fn compute(chart: &Chart, count: u32) -> LuckCycle {
    let year_yang = chart.year.gan().is_yang();
    let direction = match (year_yang, chart.gender) {
        (true, Gender::Male) | (false, Gender::Female) => Direction::Forward,
        _ => Direction::Backward,
    };

    let bounds = month_bounds(chart.solar_year);
    let span_days = match direction {
        Direction::Forward => bounds[chart.month_index + 1] - chart.jd_ut,
        Direction::Backward => chart.jd_ut - bounds[chart.month_index],
    };

    // 三日折一年：一日折 120 運日，一年為 360 運日。
    let luck_days = span_days * 120.0;
    let start_years = (luck_days / 360.0).floor() as u32;
    let rem = luck_days - start_years as f64 * 360.0;
    let start_months = (rem / 30.0).floor() as u32;
    let start_days = (rem - start_months as f64 * 30.0).round() as u32;

    let start_time = add_ymd(
        &chart.input,
        start_years as i32,
        start_months as i32,
        start_days as i32,
    );

    // 虛歲：生年即一歲，每逢立春進一歲。
    // 交運落在元旦與立春之間者，干支年仍屬前一年，不可逕取曆年。
    let birth_solar_year = chart.solar_year;
    let start_solar_year = solar_year_of(&start_time, chart.options.tz);
    let start_age = (start_solar_year - birth_solar_year).max(0) as u32 + 1;

    let sign = direction.sign();

    // 逐步交運時刻，恰隔十個曆年。多推一步，供末步之年區間收尾。
    let starts: Vec<DateTime> = (0..=count as i32)
        .map(|k| add_ymd(&start_time, k * 10, 0, 0))
        .collect();
    // 每步交運所屬之干支年，各自比立春求得，不由首步加十推。
    let handover: Vec<i32> = starts
        .iter()
        .map(|t| solar_year_of(t, chart.options.tz))
        .collect();

    let da_yun: Vec<DaYun> = (1..=count)
        .map(|n| {
            let i = (n - 1) as usize;
            // 交運落在某干支年之內時，該年之立春已過，該年仍歸前一步；
            // 本步自其後第一個立春起算，至次步交運所在之干支年為止。
            let from = handover[i] + 1;
            let to = handover[i + 1];
            DaYun {
                ordinal: n,
                pillar: chart.month.step(sign * n as i64),
                // 起運虛歲取慣例梯，逐步加十。萬年曆皆作五、十五、二十五，
                // 改為逐步查立春會得九與十一交替之梯，反不可讀。
                start_age: start_age + (n - 1) * 10,
                start_time: starts[i],
                start_year: from,
                year_range: (from, to),
            }
        })
        .collect();

    // 小運列至起運之歲為止。含 start_age 本身——交運在該虛歲之年中，
    // 該年前半仍走小運。少列這一歲，該年即無柱可答。
    let xiao_yun: Vec<XiaoYun> =
        (1..=start_age).map(|age| xiao_yun_at(chart, direction, age)).collect();

    LuckCycle {
        direction,
        span_days,
        start_years,
        start_months,
        start_days,
        start_time,
        da_yun,
        xiao_yun,
    }
}

/// 曆法加年月日，保留時分秒。日期溢出當月時取該月末日。
fn add_ymd(dt: &DateTime, years: i32, months: i32, days: i32) -> DateTime {
    let total = (dt.year * 12 + dt.month as i32 - 1) + years * 12 + months;
    let y = total.div_euclid(12);
    let m = total.rem_euclid(12) as u32 + 1;
    let d = dt.day.min(days_in_month(y, m));
    let base = DateTime::new(y, m, d, dt.hour, dt.minute, dt.second);
    from_jd(to_jd(&base) + days as f64)
}

fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::Options;

    fn make(y: i32, m: u32, d: u32, h: u32, g: Gender) -> (Chart, LuckCycle) {
        let c = Chart::new(DateTime::new(y, m, d, h, 0, 0), g, Options::default());
        let l = compute(&c, 10);
        (c, l)
    }

    #[test]
    fn direction_rules() {
        // 1984 甲子年，陽年。男順女逆。
        let (_, m) = make(1984, 5, 1, 10, Gender::Male);
        assert_eq!(m.direction, Direction::Forward);
        let (_, f) = make(1984, 5, 1, 10, Gender::Female);
        assert_eq!(f.direction, Direction::Backward);
        // 1985 乙丑年，陰年。男逆女順。
        let (_, m2) = make(1985, 5, 1, 10, Gender::Male);
        assert_eq!(m2.direction, Direction::Backward);
        let (_, f2) = make(1985, 5, 1, 10, Gender::Female);
        assert_eq!(f2.direction, Direction::Forward);
    }

    /// 同一生時，順逆兩造的距節日數相加，應等於本節整長。
    /// 順行數至下一節，逆行自上一節數來，二者互補。
    #[test]
    fn forward_and_backward_spans_fill_the_term() {
        for (y, m, d, h) in [
            (2003, 3, 18, 14),
            (1984, 5, 1, 10),
            (1990, 8, 8, 6),
            (2024, 12, 20, 23),
        ] {
            let dt = DateTime::new(y, m, d, h, 0, 0);
            let male = Chart::new(dt, Gender::Male, Options::default());
            let female = Chart::new(dt, Gender::Female, Options::default());
            let (lm, lf) = (compute(&male, 1), compute(&female, 1));
            assert_ne!(lm.direction, lf.direction, "{dt} 男女方向應相反");

            let bounds = month_bounds(male.solar_year);
            let term_len = bounds[male.month_index + 1] - bounds[male.month_index];
            let sum = lm.span_days + lf.span_days;
            assert!(
                (sum - term_len).abs() < 1e-9,
                "{dt} 順逆距節相加 {sum:.6} 不等於節長 {term_len:.6}"
            );
        }
    }

    #[test]
    fn da_yun_steps_from_month_pillar() {
        let (c, l) = make(1984, 5, 1, 10, Gender::Male);
        assert_eq!(l.da_yun.len(), 10);
        assert_eq!(l.da_yun[0].pillar.name(), c.month.step(1).name());
        assert_eq!(l.da_yun[9].pillar.name(), c.month.step(10).name());
        // 逆行者反向
        let (c2, l2) = make(1984, 5, 1, 10, Gender::Female);
        assert_eq!(l2.da_yun[0].pillar.name(), c2.month.step(-1).name());
    }

    #[test]
    fn start_age_within_ten_years() {
        for y in [1950, 1984, 1999, 2010] {
            for m in [1, 4, 7, 10] {
                for g in [Gender::Male, Gender::Female] {
                    let (_, l) = make(y, m, 15, 10, g);
                    assert!(l.start_years <= 10, "{y}-{m} 起運 {} 年", l.start_years);
                    assert!(l.span_days >= 0.0 && l.span_days <= 32.0);
                }
            }
        }
    }

    /// 交運落在元旦與立春之間者，干支年仍屬前一年。
    /// 逕取曆年會使虛歲多一歲，小運亦多產生一筆。
    #[test]
    fn start_age_uses_lichun_not_calendar_year() {
        let dt = DateTime::new(1950, 4, 3, 9, 0, 0);
        let c = Chart::new(dt, Gender::Male, Options::default());
        let l = compute(&c, 10);

        // 交運 1951-01-12，在 1951 立春之前
        assert_eq!((l.start_time.year, l.start_time.month), (1951, 1));
        // 干支年仍是 1950，故虛歲為一
        assert_eq!(l.da_yun[0].start_age, 1);
        // 交運在虛歲一之年中，該年前半仍走小運，故小運恰一筆
        assert_eq!(l.xiao_yun.len(), 1, "小運應為一筆，實得 {} 筆", l.xiao_yun.len());
        assert_eq!(l.xiao_yun[0].age, 1);
        assert_eq!(l.xiao_yun[0].year, c.solar_year);
    }

    /// 小運須恰好填滿起運之前，逐歲相接，不重不漏。
    #[test]
    fn xiao_yun_tiles_the_years_before_start() {
        for y in [1950, 1963, 1984, 1999, 2010, 2024] {
            for m in [1, 3, 6, 9, 11] {
                for g in [Gender::Male, Gender::Female] {
                    let c = Chart::new(
                        DateTime::new(y, m, 12, 9, 0, 0),
                        g,
                        Options::default(),
                    );
                    let l = compute(&c, 10);
                    let start_age = l.da_yun[0].start_age;

                    assert_eq!(
                        l.xiao_yun.len() as u32,
                        start_age,
                        "{y}-{m} 小運筆數不符起運虛歲"
                    );
                    for (i, x) in l.xiao_yun.iter().enumerate() {
                        assert_eq!(x.age as usize, i + 1, "小運虛歲不連續");
                        assert_eq!(
                            x.year,
                            c.solar_year + i as i32,
                            "小運年份不符虛歲"
                        );
                    }
                    // 末位小運即起運之歲；其年份 +1 恰為大運首年，兩表相接不留缺口
                    let last = l.xiao_yun.last().expect("小運至少一筆");
                    assert_eq!(last.age, start_age, "末位小運應為起運之歲");
                    assert_eq!(
                        last.year + 1,
                        l.da_yun[0].year_range.0,
                        "{y}-{m} 小運末年與大運首年之間有缺口"
                    );
                }
            }
        }
    }

    #[test]
    fn xiao_yun_fills_years_before_start() {
        let (c, l) = make(1984, 5, 1, 10, Gender::Male);
        assert_eq!(l.xiao_yun.len() as u32, l.da_yun[0].start_age);
        let first = l.xiao_yun.first().expect("小運至少一筆");
        assert_eq!(first.age, 1);
        assert_eq!(first.pillar.name(), c.hour.step(1).name());
    }

    /// 虛歲取慣例梯，逐步加十；交運時刻恰隔十個曆年。
    /// 年區間則不必為十——交運不落在立春上，投影到立春格子會有增減。
    #[test]
    fn da_yun_ordinals_advance_ten_years() {
        let (_, l) = make(1990, 8, 8, 6, Gender::Male);
        for w in l.da_yun.windows(2) {
            assert_eq!(w[1].start_age, w[0].start_age + 10, "虛歲應為慣例梯");
            let (a, b) = (&w[0].start_time, &w[1].start_time);
            assert_eq!(
                (b.year - a.year, b.month, b.day, b.hour, b.minute),
                (10, a.month, a.day, a.hour, a.minute),
                "交運應恰隔十個曆年"
            );
        }
    }

    /// 年區間逐步相接，不重不漏；首步起於首個交運之後的立春。
    #[test]
    fn da_yun_year_ranges_tile_without_gap_or_overlap() {
        for (y, m, d, h) in [(1915, 1, 18, 21), (1990, 8, 8, 6), (2003, 3, 18, 14), (1950, 4, 3, 9)] {
            for g in [Gender::Male, Gender::Female] {
                let c = Chart::new(DateTime::new(y, m, d, h, 0, 0), g, Options::default());
                let l = compute(&c, 12);
                for w in l.da_yun.windows(2) {
                    assert_eq!(
                        w[1].year_range.0,
                        w[0].year_range.1 + 1,
                        "{y}-{m}-{d} 第{}運與第{}運之年區間未相接",
                        w[0].ordinal,
                        w[1].ordinal
                    );
                }
                for d in &l.da_yun {
                    let span = d.year_range.1 - d.year_range.0 + 1;
                    assert!(
                        (9..=11).contains(&span),
                        "{y}-{m}-{d:?} 第{}運年區間 {span} 年，超出九至十一",
                        d.ordinal
                    );
                    assert_eq!(d.start_year, d.year_range.0, "start_year 應等於區間之首");
                }
            }
        }
    }

    /// 小運表須與公式一致——表若自行其是，表末必生缺口。
    #[test]
    fn xiao_yun_table_matches_the_formula() {
        for (y, m, d) in [(1915, 1, 18), (1950, 4, 3), (1984, 5, 1), (2003, 3, 18)] {
            for g in [Gender::Male, Gender::Female] {
                let c = Chart::new(DateTime::new(y, m, d, 9, 0, 0), g, Options::default());
                let l = compute(&c, 10);
                for x in &l.xiao_yun {
                    let f = xiao_yun_at(&c, l.direction, x.age);
                    assert_eq!((x.pillar.name(), x.year), (f.pillar.name(), f.year));
                }
                // 公式在起運之後仍給得出值——小運終身不斷
                let after = xiao_yun_at(&c, l.direction, 60);
                assert_eq!(after.year, c.solar_year + 59);
                assert_eq!(after.pillar.name(), c.hour.step(l.direction.sign() * 60).name());
            }
        }
    }

    /// 立春在二月三日至五日之間游移，交運時刻固定，二線交錯。
    /// 首步查表、其後加十者，於此盤第二、四、六步皆錯一年。
    #[test]
    fn da_yun_year_range_tracks_lichun_drift() {
        let (_, l) = make(1915, 1, 18, 21, Gender::Female);
        let got: Vec<(i32, i32)> = l.da_yun.iter().take(5).map(|d| d.year_range).collect();
        assert_eq!(
            got,
            vec![(1919, 1929), (1930, 1938), (1939, 1949), (1950, 1958), (1959, 1969)],
            "年區間未追隨立春游移"
        );
        // 交運時刻本身恆為 02-04 21:00，游移的是立春
        for d in l.da_yun.iter().take(5) {
            assert_eq!(
                (d.start_time.month, d.start_time.day, d.start_time.hour),
                (2, 4, 21)
            );
        }
    }
}
