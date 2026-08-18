//! 天干、地支、六十甲子與五行十神。

pub const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub const ZHI: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

/// 五行
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WuXing {
    Mu,
    Huo,
    Tu,
    Jin,
    Shui,
}

impl WuXing {
    pub fn name(self) -> &'static str {
        match self {
            WuXing::Mu => "木",
            WuXing::Huo => "火",
            WuXing::Tu => "土",
            WuXing::Jin => "金",
            WuXing::Shui => "水",
        }
    }

    /// 我生者
    pub fn generates(self) -> WuXing {
        match self {
            WuXing::Mu => WuXing::Huo,
            WuXing::Huo => WuXing::Tu,
            WuXing::Tu => WuXing::Jin,
            WuXing::Jin => WuXing::Shui,
            WuXing::Shui => WuXing::Mu,
        }
    }

    /// 我剋者
    pub fn controls(self) -> WuXing {
        match self {
            WuXing::Mu => WuXing::Tu,
            WuXing::Tu => WuXing::Shui,
            WuXing::Shui => WuXing::Huo,
            WuXing::Huo => WuXing::Jin,
            WuXing::Jin => WuXing::Mu,
        }
    }
}

/// 天干，0 = 甲 … 9 = 癸
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gan(pub u8);

impl Gan {
    pub fn name(self) -> &'static str {
        GAN[self.0 as usize]
    }

    /// 陽干為 true
    pub fn is_yang(self) -> bool {
        self.0 % 2 == 0
    }

    pub fn wuxing(self) -> WuXing {
        match self.0 / 2 {
            0 => WuXing::Mu,
            1 => WuXing::Huo,
            2 => WuXing::Tu,
            3 => WuXing::Jin,
            _ => WuXing::Shui,
        }
    }

    /// 以日干為我，取本干之十神
    pub fn ten_god(self, day_master: Gan) -> &'static str {
        let (me, other) = (day_master.wuxing(), self.wuxing());
        let same_polarity = day_master.is_yang() == self.is_yang();
        if me == other {
            if same_polarity { "比肩" } else { "劫財" }
        } else if me.generates() == other {
            if same_polarity { "食神" } else { "傷官" }
        } else if me.controls() == other {
            if same_polarity { "偏財" } else { "正財" }
        } else if other.controls() == me {
            if same_polarity { "七殺" } else { "正官" }
        } else {
            if same_polarity { "偏印" } else { "正印" }
        }
    }
}

/// 地支，0 = 子 … 11 = 亥
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zhi(pub u8);

impl Zhi {
    pub fn name(self) -> &'static str {
        ZHI[self.0 as usize]
    }

    pub fn is_yang(self) -> bool {
        self.0 % 2 == 0
    }

    pub fn wuxing(self) -> WuXing {
        match self.0 {
            2 | 3 => WuXing::Mu,
            5 | 6 => WuXing::Huo,
            8 | 9 => WuXing::Jin,
            11 | 0 => WuXing::Shui,
            _ => WuXing::Tu, // 辰戌丑未
        }
    }

    /// 生肖
    pub fn animal(self) -> &'static str {
        const A: [&str; 12] = [
            "鼠", "牛", "虎", "兔", "龍", "蛇", "馬", "羊", "猴", "雞", "狗", "豬",
        ];
        A[self.0 as usize]
    }
}

/// 干支柱，內部以六十甲子序 0..59 表示。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GanZhi(pub u8);

impl GanZhi {
    /// 由六十甲子序建立，自動取模。
    pub fn from_index(i: i64) -> GanZhi {
        GanZhi(i.rem_euclid(60) as u8)
    }

    /// 由干序與支序建立。干支奇偶必須一致，否則不成柱。
    pub fn from_gan_zhi(gan: u8, zhi: u8) -> Option<GanZhi> {
        (0..60)
            .find(|i| i % 10 == gan as i64 && i % 12 == zhi as i64)
            .map(GanZhi::from_index)
    }

    pub fn gan(self) -> Gan {
        Gan(self.0 % 10)
    }

    pub fn zhi(self) -> Zhi {
        Zhi(self.0 % 12)
    }

    pub fn name(self) -> String {
        format!("{}{}", self.gan().name(), self.zhi().name())
    }

    /// 順推 n 位（n 可為負，即逆推）。
    pub fn step(self, n: i64) -> GanZhi {
        GanZhi::from_index(self.0 as i64 + n)
    }
}

impl std::fmt::Display for GanZhi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// 六十甲子納音。
pub fn nayin(gz: GanZhi) -> &'static str {
    const N: [&str; 30] = [
        "海中金", "爐中火", "大林木", "路旁土", "劍鋒金", "山頭火",
        "澗下水", "城頭土", "白蠟金", "楊柳木", "泉中水", "屋上土",
        "霹靂火", "松柏木", "長流水", "沙中金", "山下火", "平地木",
        "壁上土", "金箔金", "覆燈火", "天河水", "大驛土", "釵釧金",
        "桑柘木", "大溪水", "沙中土", "天上火", "石榴木", "大海水",
    ];
    N[(gz.0 / 2) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ganzhi_cycle() {
        assert_eq!(GanZhi(0).name(), "甲子");
        assert_eq!(GanZhi(59).name(), "癸亥");
        assert_eq!(GanZhi(0).step(-1).name(), "癸亥");
        assert_eq!(GanZhi::from_gan_zhi(0, 10).unwrap().name(), "甲戌");
        assert_eq!(GanZhi::from_gan_zhi(0, 1), None); // 甲丑不成柱
    }

    #[test]
    fn ten_gods_from_jia() {
        let jia = Gan(0);
        assert_eq!(Gan(0).ten_god(jia), "比肩");
        assert_eq!(Gan(1).ten_god(jia), "劫財");
        assert_eq!(Gan(2).ten_god(jia), "食神");
        assert_eq!(Gan(3).ten_god(jia), "傷官");
        assert_eq!(Gan(4).ten_god(jia), "偏財");
        assert_eq!(Gan(5).ten_god(jia), "正財");
        assert_eq!(Gan(6).ten_god(jia), "七殺");
        assert_eq!(Gan(7).ten_god(jia), "正官");
        assert_eq!(Gan(8).ten_god(jia), "偏印");
        assert_eq!(Gan(9).ten_god(jia), "正印");
    }

    #[test]
    fn nayin_known_pillars() {
        assert_eq!(nayin(GanZhi::from_index(0)), "海中金"); // 甲子
        assert_eq!(nayin(GanZhi::from_index(1)), "海中金"); // 乙丑
        assert_eq!(nayin(GanZhi::from_index(2)), "爐中火"); // 丙寅
        assert_eq!(nayin(GanZhi::from_index(59)), "大海水"); // 癸亥
    }

    /// 納音之名末字即其五行。
    fn nayin_wuxing(gz: GanZhi) -> char {
        nayin(gz).chars().last().expect("納音名非空")
    }

    /// 納音五行週期為十五：某柱與其後三十位（干支序差三十）五行相同。
    /// 名雖三十異，五行僅十五別。
    #[test]
    fn nayin_wuxing_has_period_fifteen() {
        for i in 0..30i64 {
            let a = GanZhi::from_index(i * 2);
            let b = GanZhi::from_index(i * 2 + 30);
            assert_eq!(
                nayin_wuxing(a),
                nayin_wuxing(b),
                "{} 與 {} 納音五行應同",
                a.name(),
                b.name()
            );
        }
    }

    /// 六十甲子分三十納音，五行各得其六。
    #[test]
    fn nayin_wuxing_evenly_distributed() {
        let mut count = std::collections::BTreeMap::new();
        for i in 0..30i64 {
            *count.entry(nayin_wuxing(GanZhi::from_index(i * 2))).or_insert(0) += 1;
        }
        assert_eq!(count.len(), 5, "納音五行應為五類，實得 {:?}", count.keys());
        for (wx, n) in &count {
            assert_eq!(*n, 6, "納音五行 {wx} 應得六，實得 {n}");
        }
    }

    /// 同一納音必涵蓋相鄰兩柱，且該兩柱干支陰陽相對。
    #[test]
    fn nayin_pairs_adjacent_pillars() {
        for i in 0..30i64 {
            let a = GanZhi::from_index(i * 2);
            let b = GanZhi::from_index(i * 2 + 1);
            assert_eq!(nayin(a), nayin(b), "{} 與 {} 應同納音", a.name(), b.name());
            assert_ne!(a.gan().is_yang(), b.gan().is_yang(), "成對兩柱陰陽應相對");
        }
    }

    /// 納音名為查表所得，其名雖不可推，仍有數則定性可驗：
    /// 三十條互異、皆三字、末字即其五行。
    /// 前綴不必互異——沙中金與沙中土並存，本即如此。
    #[test]
    fn nayin_names_are_wellformed_and_distinct() {
        let all: Vec<&str> = (0..30i64).map(|i| nayin(GanZhi::from_index(i * 2))).collect();
        assert_eq!(all.len(), 30);

        let unique: std::collections::BTreeSet<_> = all.iter().collect();
        assert_eq!(unique.len(), 30, "納音名應三十條互異");

        for n in &all {
            assert_eq!(n.chars().count(), 3, "納音名 {n} 非三字");
            let last = n.chars().last().unwrap();
            assert!("金木水火土".contains(last), "納音名 {n} 末字非五行");
        }
    }
}
