//! 曆法換算與太陽視黃經，用以求節氣時刻。
//!
//! 太陽位置採 Meeus《Astronomical Algorithms》第 25 章低精度式，
//! 誤差約 0.01 度，換算為時間約 15 分鐘上限；近代通常優於此值。
//! 生時若正落在節氣交界前後數分鐘，須另查權威萬年曆核對。

use std::f64::consts::PI;

const DEG: f64 = PI / 180.0;

/// 民用日期時間（本地時），供輸入與輸出使用。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl DateTime {
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Self {
        DateTime { year, month, day, hour, minute, second }
    }

    /// 一日之中已過的日數比例。
    pub fn day_fraction(&self) -> f64 {
        (self.hour as f64 + self.minute as f64 / 60.0 + self.second as f64 / 3600.0) / 24.0
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

/// 曆日轉儒略日。1582-10-15 以後視為格里曆。
pub fn to_jd(dt: &DateTime) -> f64 {
    let mut y = dt.year as f64;
    let mut m = dt.month as f64;
    if m <= 2.0 {
        y -= 1.0;
        m += 12.0;
    }
    let day = dt.day as f64 + dt.day_fraction();
    let gregorian = (dt.year, dt.month, dt.day) >= (1582, 10, 15);
    let b = if gregorian {
        let a = (y / 100.0).floor();
        2.0 - a + (a / 4.0).floor()
    } else {
        0.0
    };
    (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + day + b - 1524.5
}

/// 儒略日轉曆日。秒數四捨五入，必要時進位到下一分鐘。
pub fn from_jd(jd: f64) -> DateTime {
    let z_f = (jd + 0.5).floor();
    let f = jd + 0.5 - z_f;
    let z = z_f as i64;
    let a = if z < 2299161 {
        z
    } else {
        let alpha = ((z as f64 - 1867216.25) / 36524.25).floor() as i64;
        z + 1 + alpha - alpha.div_euclid(4)
    };
    let b = a + 1524;
    let c = ((b as f64 - 122.1) / 365.25).floor() as i64;
    let d = (365.25 * c as f64).floor() as i64;
    let e = ((b - d) as f64 / 30.6001).floor() as i64;

    let day = b - d - (30.6001 * e as f64).floor() as i64;
    let month = if e < 14 { e - 1 } else { e - 13 };
    let year = if month > 2 { c - 4716 } else { c - 4715 };

    // 由日的小數部分還原時分秒，並處理四捨五入的進位。
    let secs = (f * 86400.0).round() as i64;
    let (year, month, day, total_seconds) = if secs >= 86400 {
        let (y2, m2, d2) = next_day(year as i32, month as u32, day as u32);
        (y2 as i64, m2 as i64, d2 as i64, secs - 86400)
    } else {
        (year, month, day, secs)
    };

    DateTime {
        year: year as i32,
        month: month as u32,
        day: day as u32,
        hour: (total_seconds / 3600) as u32,
        minute: (total_seconds % 3600 / 60) as u32,
        second: (total_seconds % 60) as u32,
    }
}

fn is_leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap(y) { 29 } else { 28 },
        _ => 30,
    }
}

fn next_day(y: i32, m: u32, d: u32) -> (i32, u32, u32) {
    if d < days_in_month(y, m) {
        (y, m, d + 1)
    } else if m < 12 {
        (y, m + 1, 1)
    } else {
        (y + 1, 1, 1)
    }
}

/// 力學時與世界時之差 ΔT，單位秒。採 Espenak & Meeus 多項式。
pub fn delta_t(year: f64) -> f64 {
    let y = year;
    if y < 1600.0 {
        let u = (y - 1820.0) / 100.0;
        -20.0 + 32.0 * u * u
    } else if y < 1700.0 {
        let t = y - 1600.0;
        120.0 - 0.9808 * t - 0.01532 * t * t + t.powi(3) / 7129.0
    } else if y < 1800.0 {
        let t = y - 1700.0;
        8.83 + 0.1603 * t - 0.0059285 * t * t + 0.00013336 * t.powi(3) - t.powi(4) / 1_174_000.0
    } else if y < 1860.0 {
        let t = y - 1800.0;
        13.72 - 0.332447 * t + 0.0068612 * t.powi(2) + 0.0041116 * t.powi(3)
            - 0.00037436 * t.powi(4)
            + 0.0000121272 * t.powi(5)
            - 0.0000001699 * t.powi(6)
            + 0.000000000875 * t.powi(7)
    } else if y < 1900.0 {
        let t = y - 1860.0;
        7.62 + 0.5737 * t - 0.251754 * t.powi(2) + 0.01680668 * t.powi(3)
            - 0.0004473624 * t.powi(4)
            + t.powi(5) / 233_174.0
    } else if y < 1920.0 {
        let t = y - 1900.0;
        -2.79 + 1.494119 * t - 0.0598939 * t.powi(2) + 0.0061966 * t.powi(3)
            - 0.000197 * t.powi(4)
    } else if y < 1941.0 {
        let t = y - 1920.0;
        21.20 + 0.84493 * t - 0.076100 * t.powi(2) + 0.0020936 * t.powi(3)
    } else if y < 1961.0 {
        let t = y - 1950.0;
        29.07 + 0.407 * t - t.powi(2) / 233.0 + t.powi(3) / 2547.0
    } else if y < 1986.0 {
        let t = y - 1975.0;
        45.45 + 1.067 * t - t.powi(2) / 260.0 - t.powi(3) / 718.0
    } else if y < 2005.0 {
        let t = y - 2000.0;
        63.86 + 0.3345 * t - 0.060374 * t.powi(2)
            + 0.0017275 * t.powi(3)
            + 0.000651814 * t.powi(4)
            + 0.00002373599 * t.powi(5)
    } else if y < 2050.0 {
        let t = y - 2000.0;
        62.92 + 0.32217 * t + 0.005589 * t * t
    } else if y < 2150.0 {
        let u = (y - 1820.0) / 100.0;
        -20.0 + 32.0 * u * u - 0.5628 * (2150.0 - y)
    } else {
        let u = (y - 1820.0) / 100.0;
        -20.0 + 32.0 * u * u
    }
}

/// 太陽視黃經，度，輸入為力學時儒略日。
///
/// 採 VSOP87D 截斷級數求地球日心黃經，加 180 度得太陽幾何黃經，
/// 再作 FK5 改正、章動與光行差改正。精度約 1 角秒，折合時間約 25 秒。
pub fn apparent_solar_longitude(jde: f64) -> f64 {
    let t = (jde - 2451545.0) / 36525.0;
    let tau = t / 10.0;

    // 地球日心黃經加 180 度即太陽幾何黃經
    let sun = crate::vsop::earth_longitude(tau) / DEG + 180.0;

    // FK5 座標系改正。黃緯改正於黃經無涉，故略。
    let fk5 = sun - 0.09033 / 3600.0;

    // 光行差，隨日地距離變動
    let r = crate::vsop::earth_radius(tau);
    let aberration = -20.4898 / (3600.0 * r);

    (fk5 + nutation_longitude(t) + aberration).rem_euclid(360.0)
}

/// 黃經章動 Δψ，度。取主要四項，精度約 0.5 角秒。
fn nutation_longitude(t: f64) -> f64 {
    let omega = (125.04452 - 1934.136261 * t) * DEG;
    let l_sun = (280.4665 + 36000.7698 * t) * DEG;
    let l_moon = (218.3165 + 481267.8813 * t) * DEG;
    let arcsec = -17.20 * omega.sin() - 1.32 * (2.0 * l_sun).sin()
        - 0.23 * (2.0 * l_moon).sin()
        + 0.21 * (2.0 * omega).sin();
    arcsec / 3600.0
}

/// 十二節的目標視黃經（度）與粗略播種日期（月, 日），
/// 依序為立春、驚蟄、清明、立夏、芒種、小暑、立秋、白露、寒露、立冬、大雪、小寒。
/// 索引即月支序：0 為寅月，11 為丑月。小寒落在次年一月。
pub const JIE: [(&str, f64, u32, u32); 12] = [
    ("立春", 315.0, 2, 4),
    ("驚蟄", 345.0, 3, 6),
    ("清明", 15.0, 4, 5),
    ("立夏", 45.0, 5, 6),
    ("芒種", 75.0, 6, 6),
    ("小暑", 105.0, 7, 7),
    ("立秋", 135.0, 8, 8),
    ("白露", 165.0, 9, 8),
    ("寒露", 195.0, 10, 8),
    ("立冬", 225.0, 11, 7),
    ("大雪", 255.0, 12, 7),
    ("小寒", 285.0, 1, 6),
];

/// 求節氣時刻，回傳世界時儒略日。
///
/// `solar_year` 為年柱所屬之年，`month_index` 為月支序（0 = 寅月）。
/// 小寒（索引 11）落在 `solar_year + 1` 的一月。
pub fn jie_jd_ut(solar_year: i32, month_index: usize) -> f64 {
    let (_, target, seed_month, seed_day) = JIE[month_index];
    let seed_year = if month_index == 11 { solar_year + 1 } else { solar_year };
    let mut jde = to_jd(&DateTime::new(seed_year, seed_month, seed_day, 12, 0, 0));

    // 太陽日行約 0.98565 度，以牛頓法迭代收斂。
    for _ in 0..30 {
        let lambda = apparent_solar_longitude(jde);
        let mut diff = target - lambda;
        diff = (diff + 180.0).rem_euclid(360.0) - 180.0;
        let step = diff / 0.98565;
        jde += step;
        if step.abs() < 1e-9 {
            break;
        }
    }
    let year_f = seed_year as f64 + seed_month as f64 / 12.0;
    jde - delta_t(year_f) / 86400.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn julian_roundtrip() {
        // Meeus 例 7.a
        assert!((to_jd(&DateTime::new(1957, 10, 4, 19, 26, 24)) - 2436116.31).abs() < 1e-2);
        assert!((to_jd(&DateTime::new(2000, 1, 1, 12, 0, 0)) - 2451545.0).abs() < 1e-9);
        let dt = DateTime::new(1990, 5, 15, 14, 30, 0);
        let back = from_jd(to_jd(&dt));
        assert_eq!(back, dt);
    }

    #[test]
    fn lichun_lands_early_february() {
        for y in [1950, 1984, 2000, 2024] {
            let jd = jie_jd_ut(y, 0);
            let dt = from_jd(jd + 8.0 / 24.0); // 東八區
            assert_eq!(dt.year, y);
            assert_eq!(dt.month, 2);
            assert!((3..=5).contains(&dt.day), "{y} 立春落在 {dt}");
        }
    }

    #[test]
    fn xiaohan_falls_in_following_january() {
        let dt = from_jd(jie_jd_ut(2023, 11) + 8.0 / 24.0);
        assert_eq!((dt.year, dt.month), (2024, 1));
    }

    /// 對照權威萬年曆之 2024 年十二節（北京時間）。
    /// 容差兩分鐘，足以攔下級數表損毀，又不因末位截斷而誤報。
    #[test]
    fn terms_match_published_almanac() {
        let expected = [
            ("立春", 2024, 2, 4, 16, 26),
            ("驚蟄", 2024, 3, 5, 10, 22),
            ("清明", 2024, 4, 4, 15, 2),
            ("立夏", 2024, 5, 5, 8, 9),
            ("芒種", 2024, 6, 5, 12, 9),
            ("小暑", 2024, 7, 6, 22, 19),
            ("立秋", 2024, 8, 7, 8, 9),
            ("白露", 2024, 9, 7, 11, 11),
            ("寒露", 2024, 10, 8, 2, 59),
            ("立冬", 2024, 11, 7, 6, 20),
            ("大雪", 2024, 12, 6, 23, 17),
            ("小寒", 2025, 1, 5, 10, 33),
        ];
        for (i, (name, y, mo, d, h, mi)) in expected.iter().enumerate() {
            let got = jie_jd_ut(2024, i) + 8.0 / 24.0;
            let want = to_jd(&DateTime::new(*y, *mo, *d, *h, *mi, 0));
            let diff_min = (got - want).abs() * 1440.0;
            assert!(diff_min < 2.0, "{name} 差 {diff_min:.2} 分鐘，算得 {}", from_jd(got));
        }
    }

    #[test]
    fn solar_longitude_at_terms() {
        // 求得的時刻代回，視黃經應等於目標值
        for (i, (_, target, _, _)) in JIE.iter().enumerate() {
            let jd_ut = jie_jd_ut(2020, i);
            let jde = jd_ut + delta_t(2020.0) / 86400.0;
            let lambda = apparent_solar_longitude(jde);
            let diff = ((lambda - target + 180.0).rem_euclid(360.0) - 180.0).abs();
            assert!(diff < 1e-4, "節氣 {i} 黃經誤差 {diff}");
        }
    }
}
