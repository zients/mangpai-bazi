//! 地支藏干。
//!
//! 一支之中所蘊之天干，分本氣、中氣、餘氣三位。與時間無涉，
//! 凡此支現於盤上即帶此三干。
//!
//! 四正氣專，四生藏三合局之長生，四庫藏所庫之氣而以前一正支之本氣為餘氣。
//! 此結構可由三合局反推，測試即據此驗表。
//!
//! 人元司令分野本亦在此，後已移除——那是子平月令取格之法，盲派不用。

use crate::ganzhi::{Gan, Zhi};

/// 藏干在本支中的分位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// 本氣，一支之正，五行與本支同。
    BenQi,
    /// 中氣，三合局之長生或墓庫所藏。
    ZhongQi,
    /// 餘氣，前一支遞來之氣。
    YuQi,
}

impl Role {
    pub fn name(self) -> &'static str {
        match self {
            Role::BenQi => "本氣",
            Role::ZhongQi => "中氣",
            Role::YuQi => "餘氣",
        }
    }
}

/// 一位藏干。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hidden {
    pub gan: Gan,
    pub role: Role,
}

impl Hidden {
    const fn new(gan: u8, role: Role) -> Hidden {
        Hidden { gan: Gan(gan), role }
    }
}

use Role::{BenQi as B, YuQi as Y, ZhongQi as Z};

/// 地支藏干表，本氣在前。
///
/// 四正（子午卯酉）氣專，午另藏己為中氣。
/// 四生（寅申巳亥）藏本氣、三合長生、土之餘氣。
/// 四庫（辰戌丑未）藏土之本氣、所庫之中氣、前一正支遞來之餘氣。
///
/// 亥是否兼藏戊、午是否兼藏丙，各家不一。此表取多數之說：亥不列戊，午不列丙。
/// 該二干仍見於司令表，蓋司令本含前月遞來之氣。
const HIDDEN: [&[Hidden]; 12] = [
    // 子 癸
    &[Hidden::new(9, B)],
    // 丑 己癸辛：己本、癸餘（子月遞來）、辛中（金庫）
    &[Hidden::new(5, B), Hidden::new(9, Y), Hidden::new(7, Z)],
    // 寅 甲丙戊：甲本、丙中（火長生）、戊餘
    &[Hidden::new(0, B), Hidden::new(2, Z), Hidden::new(4, Y)],
    // 卯 乙
    &[Hidden::new(1, B)],
    // 辰 戊乙癸：戊本、乙餘（卯月遞來）、癸中（水庫）
    &[Hidden::new(4, B), Hidden::new(1, Y), Hidden::new(9, Z)],
    // 巳 丙庚戊：丙本、庚中（金長生）、戊餘
    &[Hidden::new(2, B), Hidden::new(6, Z), Hidden::new(4, Y)],
    // 午 丁己：丁本、己中
    &[Hidden::new(3, B), Hidden::new(5, Z)],
    // 未 己丁乙：己本、丁餘（午月遞來）、乙中（木庫）
    &[Hidden::new(5, B), Hidden::new(3, Y), Hidden::new(1, Z)],
    // 申 庚壬戊：庚本、壬中（水長生）、戊餘
    &[Hidden::new(6, B), Hidden::new(8, Z), Hidden::new(4, Y)],
    // 酉 辛
    &[Hidden::new(7, B)],
    // 戌 戊辛丁：戊本、辛餘（酉月遞來）、丁中（火庫）
    &[Hidden::new(4, B), Hidden::new(7, Y), Hidden::new(3, Z)],
    // 亥 壬甲：壬本、甲中（木長生）
    &[Hidden::new(8, B), Hidden::new(0, Z)],
];

/// 取一支所藏諸干，本氣在前。
pub fn hidden(zhi: Zhi) -> &'static [Hidden] {
    HIDDEN[zhi.0 as usize]
}

/// 取一支之本氣。
pub fn ben_qi(zhi: Zhi) -> Gan {
    hidden(zhi)[0].gan
}

/// 取一支之中氣，四正之子卯酉無之。
pub fn zhong_qi(zhi: Zhi) -> Option<Gan> {
    hidden(zhi).iter().find(|h| h.role == Role::ZhongQi).map(|h| h.gan)
}

/// 取一支之餘氣。
pub fn yu_qi(zhi: Zhi) -> Option<Gan> {
    hidden(zhi).iter().find(|h| h.role == Role::YuQi).map(|h| h.gan)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ganzhi::WuXing;

    /// 三合局：(長生, 帝旺, 墓庫, 局之五行)
    const SAN_HE: [(u8, u8, u8, WuXing); 4] = [
        (8, 0, 4, WuXing::Shui),  // 申子辰 水局
        (11, 3, 7, WuXing::Mu),   // 亥卯未 木局
        (2, 6, 10, WuXing::Huo),  // 寅午戌 火局
        (5, 9, 1, WuXing::Jin),   // 巳酉丑 金局
    ];

    /// 某五行之陽干。
    fn yang_gan(wx: WuXing) -> Gan {
        Gan(match wx {
            WuXing::Mu => 0,
            WuXing::Huo => 2,
            WuXing::Tu => 4,
            WuXing::Jin => 6,
            WuXing::Shui => 8,
        })
    }

    fn yin_gan(wx: WuXing) -> Gan {
        Gan(yang_gan(wx).0 + 1)
    }

    /// 藏干非硬記，可由三合局推得：
    /// 四生之中氣取本局五行之陽干，四庫之中氣取本局五行之陰干。
    /// 以局推表，驗硬編之表無誤。
    #[test]
    fn zhong_qi_derives_from_san_he_ju() {
        for (sheng, wang, ku, wx) in SAN_HE {
            assert_eq!(
                zhong_qi(Zhi(sheng)),
                Some(yang_gan(wx)),
                "{} 為{}局長生，中氣應取陽干",
                Zhi(sheng).name(),
                wx.name()
            );
            assert_eq!(
                zhong_qi(Zhi(ku)),
                Some(yin_gan(wx)),
                "{} 為{}局墓庫，中氣應取陰干",
                Zhi(ku).name(),
                wx.name()
            );
            // 帝旺者即四正，氣專，其本氣五行必同本局
            assert_eq!(ben_qi(Zhi(wang)).wuxing(), wx, "{} 帝旺五行不符本局", Zhi(wang).name());
        }
    }

    /// 四庫之餘氣，即前一正支之本氣。
    #[test]
    fn si_ku_yu_qi_is_previous_branch_ben_qi() {
        for ku in [1u8, 4, 7, 10] {
            let prev = Zhi((ku + 11) % 12);
            assert_eq!(
                yu_qi(Zhi(ku)),
                Some(ben_qi(prev)),
                "{} 之餘氣應為前支 {} 之本氣",
                Zhi(ku).name(),
                prev.name()
            );
        }
    }

    /// 四生之餘氣皆戊土。亥不列戊，屬流派取捨，見表首註。
    #[test]
    fn si_sheng_yu_qi_is_wu_earth() {
        for sheng in [2u8, 5, 8] {
            assert_eq!(yu_qi(Zhi(sheng)).map(|g| g.name()), Some("戊"));
        }
        assert_eq!(yu_qi(Zhi(11)), None); // 亥
    }

    /// 本氣陰陽從本支，惟子午巳亥四支相反，此為子平舊例。
    #[test]
    fn ben_qi_polarity_inverts_only_for_zi_wu_si_hai() {
        const INVERTED: [u8; 4] = [0, 5, 6, 11]; // 子巳午亥
        for i in 0..12u8 {
            let zhi = Zhi(i);
            let same = ben_qi(zhi).is_yang() == zhi.is_yang();
            if INVERTED.contains(&i) {
                assert!(!same, "{} 本氣陰陽應與本支相反", zhi.name());
            } else {
                assert!(same, "{} 本氣陰陽應與本支相同", zhi.name());
            }
        }
    }

    #[test]
    fn every_branch_has_exactly_one_ben_qi() {
        for i in 0..12 {
            let zhi = Zhi(i);
            let n = hidden(zhi).iter().filter(|h| h.role == Role::BenQi).count();
            assert_eq!(n, 1, "{} 本氣不為一", zhi.name());
            // 本氣必列首位
            assert_eq!(hidden(zhi)[0].role, Role::BenQi);
        }
    }

    #[test]
    fn ben_qi_shares_wuxing_with_branch() {
        for i in 0..12 {
            let zhi = Zhi(i);
            assert_eq!(
                ben_qi(zhi).wuxing(),
                zhi.wuxing(),
                "{} 本氣五行不符本支",
                zhi.name()
            );
        }
    }

    #[test]
    fn si_zheng_have_no_zhong_qi_except_wu() {
        assert_eq!(zhong_qi(Zhi(0)), None); // 子
        assert_eq!(zhong_qi(Zhi(3)), None); // 卯
        assert_eq!(zhong_qi(Zhi(9)), None); // 酉
        assert_eq!(zhong_qi(Zhi(6)).map(|g| g.name()), Some("己")); // 午藏己
    }

    #[test]
    fn si_ku_zhong_qi_is_the_stored_element() {
        // 辰水庫藏癸、戌火庫藏丁、丑金庫藏辛、未木庫藏乙
        assert_eq!(zhong_qi(Zhi(4)).map(|g| g.name()), Some("癸"));
        assert_eq!(zhong_qi(Zhi(10)).map(|g| g.name()), Some("丁"));
        assert_eq!(zhong_qi(Zhi(1)).map(|g| g.name()), Some("辛"));
        assert_eq!(zhong_qi(Zhi(7)).map(|g| g.name()), Some("乙"));
    }

}
