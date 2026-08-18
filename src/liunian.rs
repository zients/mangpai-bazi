//! 流年，及某時刻所行之運。
//!
//! 流年干支與年柱同法，立春換年。西元年與干支年不重合：
//! 二〇二五年一月十五日仍屬甲辰，須待二月三日立春方入乙巳。
//!
//! 大運亦然。交運非在元旦，而在生日推算所得之確切時刻，
//! 故交運當年前後分屬兩運。以曆年近似會答錯，此處逐一比對日期。

use crate::astro::{from_jd, jie_jd_ut, to_jd, DateTime};
use crate::canggan;
use crate::chart::Chart;
use crate::ganzhi::GanZhi;
use crate::luck::{DaYun, LuckCycle, XiaoYun};

/// 一個流年。
#[derive(Debug, Clone)]
pub struct LiuNian {
    /// 干支年所屬之西元年，以立春為界。
    pub year: i32,
    pub pillar: GanZhi,
    /// 流年天干對日主之十神。
    pub gan_ten_god: &'static str,
    /// 流年地支本氣對日主之十神。
    pub zhi_ten_god: &'static str,
    /// 本年立春，本地時。
    pub start: DateTime,
    /// 次年立春，本地時。本流年至此為止。
    pub end: DateTime,
    /// 虛歲。生年即一歲，每逢立春進一歲。
    /// 生年之前無虛歲可言，為 `None`——虛歲自一起算，倒推過原點只是把
    /// 計數器跑到負值，命理上無所指。
    pub age: Option<u32>,
}

/// 所行之運。起運前為小運，其後為大運。
#[derive(Debug, Clone)]
pub enum Fortune {
    Da(DaYun),
    Xiao(XiaoYun),
}

impl Fortune {
    pub fn pillar(&self) -> GanZhi {
        match self {
            Fortune::Da(d) => d.pillar,
            Fortune::Xiao(x) => x.pillar,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Fortune::Da(d) => format!("第{}運", d.ordinal),
            Fortune::Xiao(x) => format!("小運{}歲", x.age),
        }
    }
}

/// 某流年之全貌。
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub liu_nian: LiuNian,
    /// 立春當下所行之運。生年之前為 `None`。
    pub fortune: Option<Fortune>,
    /// 若本流年之內遇交運，記其後所行之運與交運時刻。
    pub hand_over: Option<(Fortune, DateTime)>,
}

/// 求某干支年之流年。
pub fn liu_nian(chart: &Chart, year: i32) -> LiuNian {
    let tz = chart.options.tz;
    let dm = chart.day_master();
    let pillar = GanZhi::from_index(year as i64 - 4);
    LiuNian {
        year,
        pillar,
        gan_ten_god: pillar.gan().ten_god(dm),
        zhi_ten_god: canggan::ben_qi(pillar.zhi()).ten_god(dm),
        start: from_jd(jie_jd_ut(year, 0) + tz / 24.0),
        end: from_jd(jie_jd_ut(year + 1, 0) + tz / 24.0),
        age: (year >= chart.solar_year).then(|| (year - chart.solar_year) as u32 + 1),
    }
}

/// 求某時刻所行之運。起運之前走小運，其後依交運時刻逐步遞進。
///
/// 生年之前回傳 `None`。以干支年為界而非出生瞬間：生年立春至生時之間
/// 仍屬虛歲一，與 [`liu_nian`] 之虛歲同界，二者不可歧出。
///
/// 時刻距立春不足一秒者，干支年之判定承 [`liu_nian_year_of`] 之界，
/// 該處以四捨五入至秒之值相比，本即該精度所限。已知干支年者
/// 應走 [`snapshot`]，不必由時刻反推。
pub fn fortune_at(chart: &Chart, luck: &LuckCycle, at: &DateTime) -> Option<Fortune> {
    fortune_in_solar_year(chart, luck, liu_nian_year_of(chart, at), to_jd(at))
}

/// 已知干支年時求所行之運。`solar_year` 為該時刻所屬之干支年，`t` 為其儒略日。
///
/// 干支年由呼叫端給定，不由 `t` 反推：`t` 若是 `from_jd` 還原之立春時刻，
/// 已四捨五入至秒，可能落在真正立春前不足一秒處，反推會退成前一年。
///
/// 小運與大運二者恆可算得：小運自虛歲一終身不斷，大運則起運後方有。
/// 取用之序為「有大運則取大運，否則取小運」，故大運未交時必落在小運上，
/// 而小運為公式所得，無取不到之虞。
fn fortune_in_solar_year(
    chart: &Chart,
    luck: &LuckCycle,
    solar_year: i32,
    t: f64,
) -> Option<Fortune> {
    if solar_year < chart.solar_year {
        return None;
    }
    // 自後往前找第一個已交之運
    if let Some(d) = luck
        .da_yun
        .iter()
        .rev()
        .find(|d| t >= to_jd(&d.start_time))
    {
        return Some(Fortune::Da(d.clone()));
    }
    // 未交運，走小運。虛歲以立春為界，逐歲皆有柱。
    let age = (solar_year - chart.solar_year) as u32 + 1;
    Some(Fortune::Xiao(crate::luck::xiao_yun_at(
        chart,
        luck.direction,
        age,
    )))
}

/// 某時刻所屬之干支年，立春為界。界之實作見 [`crate::chart::solar_year_of`]。
pub fn liu_nian_year_of(chart: &Chart, at: &DateTime) -> i32 {
    crate::chart::solar_year_of(at, chart.options.tz)
}

/// 綜觀某流年：流年干支、立春當下所行之運，及年內若有交運則併記。
pub fn snapshot(chart: &Chart, luck: &LuckCycle, year: i32) -> Snapshot {
    let ln = liu_nian(chart, year);
    // 干支年直接給定，不由 ln.start 反推——該值已捨入至秒
    let fortune = fortune_in_solar_year(chart, luck, year, to_jd(&ln.start));

    // 年內是否遇交運
    let (s, e) = (to_jd(&ln.start), to_jd(&ln.end));
    let hand_over = luck
        .da_yun
        .iter()
        .find(|d| {
            let t = to_jd(&d.start_time);
            t > s && t < e
        })
        .map(|d| (Fortune::Da(d.clone()), d.start_time));

    Snapshot { liu_nian: ln, fortune, hand_over }
}

/// 自生年至末運末年的每一個流年。
///
/// 流年逐年自足：干支、虛歲、十神、立春區間、所行之運皆備，
/// 取用端不必自行接合大運與流年——那正是最容易錯一年的地方。
pub fn all_years(chart: &Chart, luck: &LuckCycle) -> Vec<Snapshot> {
    let last = match luck.da_yun.last() {
        Some(d) => d.year_range.1,
        None => chart.solar_year + luck.xiao_yun.len() as i32 - 1,
    };
    (chart.solar_year..=last).map(|y| snapshot(chart, luck, y)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::{Gender, Options};
    use crate::luck::compute;

    fn setup(y: i32, m: u32, d: u32, h: u32, g: Gender) -> (Chart, LuckCycle) {
        let c = Chart::new(DateTime::new(y, m, d, h, 0, 0), g, Options::default());
        let l = compute(&c, 10);
        (c, l)
    }

    #[test]
    fn liu_nian_matches_year_pillar_formula() {
        let (c, _) = setup(2003, 3, 18, 14, Gender::Female);
        assert_eq!(liu_nian(&c, 2025).pillar.name(), "乙巳");
        assert_eq!(liu_nian(&c, 2024).pillar.name(), "甲辰");
        assert_eq!(liu_nian(&c, 1984).pillar.name(), "甲子");
        // 本命年：流年等於年柱
        assert_eq!(liu_nian(&c, c.solar_year).pillar.name(), c.year.name());
    }

    #[test]
    fn liu_nian_spans_lichun_to_lichun() {
        let (c, _) = setup(2003, 3, 18, 14, Gender::Female);
        let ln = liu_nian(&c, 2025);
        assert_eq!((ln.start.year, ln.start.month), (2025, 2));
        assert_eq!((ln.end.year, ln.end.month), (2026, 2));
    }

    #[test]
    fn year_boundary_is_lichun_not_new_year() {
        let (c, _) = setup(2003, 3, 18, 14, Gender::Female);
        // 2025 立春在 2 月 3 日
        assert_eq!(liu_nian_year_of(&c, &DateTime::new(2025, 1, 15, 12, 0, 0)), 2024);
        assert_eq!(liu_nian_year_of(&c, &DateTime::new(2025, 3, 15, 12, 0, 0)), 2025);
    }

    #[test]
    fn xu_sui_counts_birth_year_as_one() {
        let (c, _) = setup(2003, 3, 18, 14, Gender::Female);
        assert_eq!(liu_nian(&c, 2003).age, Some(1));
        assert_eq!(liu_nian(&c, 2025).age, Some(23));
    }

    #[test]
    fn fortune_walks_from_xiao_to_da() {
        // 2003 坤造順行，交運 2009-03-14
        let (c, l) = setup(2003, 3, 18, 14, Gender::Female);
        let first = &l.da_yun[0];
        assert_eq!(first.start_time.year, 2009);

        // 交運前一日仍走小運
        let before = DateTime::new(2009, 3, 13, 0, 0, 0);
        assert!(matches!(fortune_at(&c, &l, &before), Some(Fortune::Xiao(_))));

        // 交運後入第一大運
        let after = DateTime::new(2009, 3, 15, 0, 0, 0);
        match fortune_at(&c, &l, &after) {
            Some(Fortune::Da(d)) => assert_eq!(d.ordinal, 1),
            other => panic!("交運後應走大運，實得 {other:?}"),
        }
    }

    #[test]
    fn fortune_at_2025_is_the_second_da_yun() {
        let (c, l) = setup(2003, 3, 18, 14, Gender::Female);
        let snap = snapshot(&c, &l, 2025);
        assert_eq!(snap.liu_nian.pillar.name(), "乙巳");
        assert_eq!(snap.liu_nian.age, Some(23));
        match &snap.fortune {
            Some(Fortune::Da(d)) => {
                assert_eq!(d.ordinal, 2);
                assert_eq!(d.pillar.name(), "丁巳");
            }
            other => panic!("2025 應在第二大運，實得 {other:?}"),
        }
        // 2025 年內無交運
        assert!(snap.hand_over.is_none());
    }

    #[test]
    fn hand_over_year_reports_both_fortunes() {
        // 交運 2019-03-14，落在 2019 流年之內
        let (c, l) = setup(2003, 3, 18, 14, Gender::Female);
        let snap = snapshot(&c, &l, 2019);
        let (after, when) = snap.hand_over.expect("2019 應有交運");
        assert_eq!(when.year, 2019);
        assert_eq!(after.pillar().name(), "丁巳");
        // 立春當下仍在前一運
        assert_eq!(snap.fortune.expect("2019 立春當下應有運").pillar().name(), "丙辰");
    }

    /// 生年之前無虛歲、無行運。此前回傳 `4294967284` 之類的下溢值，
    /// 行運則被 `.max(0)` 夾成生年小運——後者尤險，看似正常而全錯。
    #[test]
    fn pre_birth_years_have_neither_age_nor_fortune() {
        let (c, l) = setup(2003, 3, 18, 14, Gender::Female);
        for y in [1990, 2001, 2002] {
            let snap = snapshot(&c, &l, y);
            assert_eq!(snap.liu_nian.age, None, "{y} 不應有虛歲");
            assert!(snap.fortune.is_none(), "{y} 不應有行運");
            assert!(snap.hand_over.is_none(), "{y} 不應有交運");
        }
    }

    /// 生前流年只砍虛歲與行運兩項，干支、立春區間、十神照舊——
    /// 對盤要看的正是這些，與出生與否無涉。
    #[test]
    fn pre_birth_keeps_pillar_terms_and_ten_gods() {
        let (c, _) = setup(2003, 3, 18, 14, Gender::Female);
        // 1990 與 2050 同為庚午年，一在生前一在生後，除虛歲外應全同
        let before = liu_nian(&c, 1990);
        let after = liu_nian(&c, 2050);
        assert_eq!(before.pillar.name(), "庚午");
        assert_eq!(before.pillar.name(), after.pillar.name());
        assert_eq!(before.gan_ten_god, after.gan_ten_god);
        assert_eq!(before.zhi_ten_god, after.zhi_ten_god);
        // 立春區間仍為真實節氣時刻
        assert_eq!((before.start.year, before.start.month), (1990, 2));
        assert_eq!((before.end.year, before.end.month), (1991, 2));
        assert_eq!(before.age, None);
        assert_eq!(after.age, Some(48));
    }

    /// 生年本身為虛歲一，且有運可行——界在干支年，非出生瞬間。
    /// 生年立春至生時之間仍算虛歲一，不可因未及生時而判為生前。
    #[test]
    fn birth_year_is_age_one_and_has_a_fortune() {
        let (c, l) = setup(2003, 3, 18, 14, Gender::Female);
        let snap = snapshot(&c, &l, 2003);
        assert_eq!(snap.liu_nian.age, Some(1));
        assert!(snap.fortune.is_some(), "生年立春當下應有小運");
        // 立春在三月生日之前，仍屬虛歲一
        assert!(snap.liu_nian.start.month == 2);
        let before_birthday = DateTime::new(2003, 2, 10, 12, 0, 0);
        assert!(fortune_at(&c, &l, &before_birthday).is_some(), "生年立春後應有運");
        // 退到生年立春之前一日，即入前一干支年，無運
        let prev = DateTime::new(2003, 2, 1, 12, 0, 0);
        assert_eq!(liu_nian_year_of(&c, &prev), 2002);
        assert!(fortune_at(&c, &l, &prev).is_none(), "生年立春前不應有運");
    }

    /// 虛歲與行運須同界：有其一必有其二，缺其一必俱缺。
    /// 二者各自判斷生前與否，歧出則一欄印「未生」另一欄印運，自相矛盾。
    ///
    /// 時區一併輪替——`liu_nian_year_of` 以時區換算世界時後方比立春，
    /// 界的判定與時區有涉，不可只驗東八區。
    #[test]
    fn age_and_fortune_share_one_birth_boundary() {
        for (by, bm, bd) in [(2003, 3, 18), (1984, 1, 20), (1990, 12, 31), (2000, 2, 4)] {
            for g in [Gender::Male, Gender::Female] {
                for tz in [8.0, 0.0, -5.0, 5.5, -11.0] {
                    let opts = Options { tz, ..Default::default() };
                    let c = Chart::new(DateTime::new(by, bm, bd, 14, 0, 0), g, opts);
                    let l = compute(&c, 10);
                    for y in (by - 20)..(by + 40) {
                        let snap = snapshot(&c, &l, y);
                        assert_eq!(
                            snap.liu_nian.age.is_some(),
                            snap.fortune.is_some(),
                            "{by}-{bm}-{bd} tz{tz} 生人查 {y}：虛歲與行運不同界"
                        );
                        // 有無之界恰為干支生年
                        assert_eq!(
                            snap.liu_nian.age.is_some(),
                            y >= c.solar_year,
                            "{by}-{bm}-{bd} tz{tz} 生人查 {y}：界不在干支生年"
                        );
                    }
                }
            }
        }
    }

    /// 大運步數為零時，逐年仍須答出正確之小運。
    ///
    /// 小運為公式所得，不受大運表存否影響。昔者由小運表查得，
    /// 查不到便取末位頂替，此時逐年皆會答出同一柱。
    #[test]
    fn xiao_yun_holds_without_any_da_yun() {
        let c = Chart::new(DateTime::new(1984, 5, 1, 10, 0, 0), Gender::Male, Options::default());
        let l = compute(&c, 0);
        assert!(l.da_yun.is_empty());
        for age in 1..=40u32 {
            let year = c.solar_year + age as i32 - 1;
            match snapshot(&c, &l, year).fortune {
                Some(Fortune::Xiao(x)) => {
                    assert_eq!(x.age, age, "{year} 年小運虛歲不符");
                    assert_eq!(
                        x.pillar.name(),
                        c.hour.step(l.direction.sign() * age as i64).name(),
                        "{year} 年小運柱不符公式"
                    );
                }
                other => panic!("{year} 年應走小運，實得 {other:?}"),
            }
        }
    }

    /// 大運表所標之年區間，須與 `fortune_at` 逐年相符。
    ///
    /// 二者分屬不同路徑：表由交運時刻投影到立春格子，`fortune_at` 逐一比對時刻。
    /// 歧出則使用者看表得一答案、下 `--at` 得另一答案——正是首步查表、
    /// 其後加十所造成的病。
    ///
    /// 起運之前亦驗：小運既為公式所得，逐年皆須答出與虛歲相符之柱。
    /// 昔者小運只排至起運之前一歲，交運當年無柱可取，遂頂出錯一步者。
    #[test]
    fn da_yun_year_ranges_agree_with_fortune_at() {
        for (by, bm, bd, bh) in [
            (1915, 1, 18, 21),
            (1990, 8, 8, 6),
            (2003, 3, 18, 14),
            (1984, 2, 4, 12),
            (1963, 11, 5, 3),
        ] {
            for g in [Gender::Male, Gender::Female] {
                let c = Chart::new(DateTime::new(by, bm, bd, bh, 0, 0), g, Options::default());
                let l = compute(&c, 12);
                let last = l.da_yun.last().unwrap().year_range.1;
                for year in c.solar_year..=last {
                    // 走 snapshot：干支年直接傳入，不由立春時刻反推。
                    // 反推會受該時刻捨入至秒之影響，退成前一年。
                    let snap = snapshot(&c, &l, year);
                    let listed = l
                        .da_yun
                        .iter()
                        .find(|d| year >= d.year_range.0 && year <= d.year_range.1)
                        .map(|d| d.ordinal);
                    match (listed, snap.fortune) {
                        // 大運表列有此年者，fortune_at 須判同一步
                        (Some(n), Some(Fortune::Da(d))) => assert_eq!(
                            n, d.ordinal,
                            "{by}-{bm}-{bd} {g:?} 之 {year} 年：表標第{n}運，判第{}運",
                            d.ordinal
                        ),
                        // 大運表未列者即起運之前，須為小運，且虛歲相符
                        (None, Some(Fortune::Xiao(x))) => {
                            assert_eq!(
                                x.age as i32,
                                year - c.solar_year + 1,
                                "{by}-{bm}-{bd} {g:?} 之 {year} 年：小運虛歲不符"
                            );
                            assert_eq!(x.year, year, "小運年份不符");
                        }
                        (a, b) => panic!(
                            "{by}-{bm}-{bd} {g:?} 之 {year} 年：表 {a:?} 與 fortune_at {:?} 不符",
                            b.map(|f| f.label())
                        ),
                    }
                }
            }
        }
    }

    #[test]
    fn every_year_resolves_to_some_fortune() {
        for g in [Gender::Male, Gender::Female] {
            let (c, l) = setup(1990, 5, 15, 14, g);
            for y in 1990..2085 {
                let snap = snapshot(&c, &l, y);
                assert_eq!(snap.liu_nian.age, Some((y - 1990) as u32 + 1));
                assert!(!snap.fortune.expect("生年之後必有運").label().is_empty());
            }
        }
    }
}
