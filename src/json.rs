//! 極簡 JSON 輸出，零相依。緊湊、無空白，給 agent 讀。
//!
//! 僅供本專案序列化之用，不作通用解析。

use crate::canggan;
use crate::chart::{Chart, Gender};
use crate::ganzhi::GanZhi;
use crate::liunian::{self, Fortune, Snapshot};
use crate::luck::{Direction, LuckCycle};
use crate::ganzhi::nayin;
use crate::astro::DateTime;

/// JSON 值。
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    /// 浮點數，`1` 為小數位數。
    Num(f64, usize),
    Str(String),
    Arr(Vec<Value>),
    Obj(Vec<(&'static str, Value)>),
}

impl Value {
    fn write(&self, out: &mut String) {
        match self {
            Value::Null => out.push_str("null"),
            Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Value::Int(n) => out.push_str(&n.to_string()),
            Value::Num(x, digits) => out.push_str(&format!("{x:.*}", digits)),
            Value::Str(s) => out.push_str(&escape(s)),
            Value::Arr(items) => {
                out.push('[');
                for (i, v) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    v.write(out);
                }
                out.push(']');
            }
            Value::Obj(pairs) => {
                out.push('{');
                for (i, (k, v)) in pairs.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push_str(&escape(k));
                    out.push(':');
                    v.write(out);
                }
                out.push('}');
            }
        }
    }

    pub fn to_json(&self) -> String {
        let mut s = String::new();
        self.write(&mut s);
        s
    }
}

/// 依 JSON 規範轉義字串，並加上引號。
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn s(v: impl Into<String>) -> Value {
    Value::Str(v.into())
}

/// 時區偏移，如 `+08:00`。
fn tz_offset(tz: f64) -> String {
    let sign = if tz < 0.0 { '-' } else { '+' };
    let total = (tz.abs() * 60.0).round() as i64;
    format!("{sign}{:02}:{:02}", total / 60, total % 60)
}

/// ISO 8601 帶時區。
fn iso(dt: &DateTime, tz: f64) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}",
        dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second,
        tz_offset(tz)
    )
}

/// 一柱之完整資訊。
fn pillar(gz: GanZhi, chart: &Chart) -> Value {
    let dm = chart.day_master();
    let hidden: Vec<Value> = canggan::hidden(gz.zhi())
        .iter()
        .map(|h| {
            Value::Obj(vec![
                ("gan", s(h.gan.name())),
                ("role", s(h.role.name())),
                ("wuxing", s(h.gan.wuxing().name())),
                ("ten_god", s(h.gan.ten_god(dm))),
            ])
        })
        .collect();

    Value::Obj(vec![
        ("ganzhi", s(gz.name())),
        ("gan", s(gz.gan().name())),
        ("gan_wuxing", s(gz.gan().wuxing().name())),
        ("gan_yang", Value::Bool(gz.gan().is_yang())),
        ("gan_ten_god", s(gz.gan().ten_god(dm))),
        ("zhi", s(gz.zhi().name())),
        ("zhi_wuxing", s(gz.zhi().wuxing().name())),
        ("zhi_animal", s(gz.zhi().animal())),
        ("nayin", s(nayin(gz))),
        ("hidden", Value::Arr(hidden)),
    ])
}

/// 一段行運。
fn fortune_value(f: &Fortune, chart: &Chart) -> Value {
    let dm = chart.day_master();
    let gz = f.pillar();
    let mut pairs = vec![
        ("kind", s(match f {
            Fortune::Da(_) => "da_yun",
            Fortune::Xiao(_) => "xiao_yun",
        })),
        ("label", s(f.label())),
        ("ganzhi", s(gz.name())),
        ("gan_ten_god", s(gz.gan().ten_god(dm))),
        ("zhi_ten_god", s(canggan::ben_qi(gz.zhi()).ten_god(dm))),
    ];
    if let Fortune::Da(d) = f {
        pairs.push(("ordinal", Value::Int(d.ordinal as i64)));
        pairs.push(("handover_age", Value::Int(d.start_age as i64)));
    }
    if let Fortune::Xiao(x) = f {
        pairs.push(("age", Value::Int(x.age as i64)));
    }
    Value::Obj(pairs)
}

/// 流年序列化。
pub fn liunian_value(chart: &Chart, snap: &Snapshot) -> Value {
    let tz = chart.options.tz;
    let ln = &snap.liu_nian;
    let mut pairs = vec![
        ("year", Value::Int(ln.year as i64)),
        ("ganzhi", s(ln.pillar.name())),
        ("gan", s(ln.pillar.gan().name())),
        ("zhi", s(ln.pillar.zhi().name())),
        ("gan_ten_god", s(ln.gan_ten_god)),
        ("zhi_ten_god", s(ln.zhi_ten_god)),
        // 生年之前無虛歲，作 null，不以 0 或負值充數
        ("age", ln.age.map_or(Value::Null, |a| Value::Int(a as i64))),
        ("start", s(iso(&ln.start, tz))),
        ("end", s(iso(&ln.end, tz))),
        (
            "fortune",
            snap.fortune
                .as_ref()
                .map_or(Value::Null, |f| fortune_value(f, chart)),
        ),
    ];
    if let Some((after, when)) = &snap.hand_over {
        pairs.push((
            "hand_over",
            Value::Obj(vec![
                ("time", s(iso(when, tz))),
                ("fortune", fortune_value(after, chart)),
            ]),
        ));
    }
    Value::Obj(pairs)
}

/// 將排盤、行運與全部流年序列化。
pub fn chart_to_json(chart: &Chart, luck: &LuckCycle) -> String {
    let tz = chart.options.tz;
    let dm = chart.day_master();

    let da_yun: Vec<Value> = luck
        .da_yun
        .iter()
        .map(|d| {
            Value::Obj(vec![
                ("ordinal", Value::Int(d.ordinal as i64)),
                ("ganzhi", s(d.pillar.name())),
                ("gan", s(d.pillar.gan().name())),
                ("zhi", s(d.pillar.zhi().name())),
                ("gan_ten_god", s(d.pillar.gan().ten_god(dm))),
                ("zhi_ten_god", s(canggan::ben_qi(d.pillar.zhi()).ten_god(dm))),
                ("nayin", s(nayin(d.pillar))),
                // 交運：時刻，及其當下之虛歲（慣例梯，逐步加十）
                ("handover_time", s(iso(&d.start_time, tz))),
                ("handover_age", Value::Int(d.start_age as i64)),
                // 所轄：干支年區間，及對應之虛歲區間。
                // 交運在虛歲 handover_age 之年中，故 age_from 通常大於 handover_age。
                ("year_from", Value::Int(d.year_range.0 as i64)),
                ("year_to", Value::Int(d.year_range.1 as i64)),
                (
                    "age_from",
                    Value::Int((d.year_range.0 - chart.solar_year + 1) as i64),
                ),
                (
                    "age_to",
                    Value::Int((d.year_range.1 - chart.solar_year + 1) as i64),
                ),
            ])
        })
        .collect();

    let xiao_yun: Vec<Value> = luck
        .xiao_yun
        .iter()
        .map(|x| {
            Value::Obj(vec![
                ("age", Value::Int(x.age as i64)),
                ("year", Value::Int(x.year as i64)),
                ("ganzhi", s(x.pillar.name())),
                ("gan_ten_god", s(x.pillar.gan().ten_god(dm))),
                ("zhi_ten_god", s(canggan::ben_qi(x.pillar.zhi()).ten_god(dm))),
            ])
        })
        .collect();

    let mut root_pairs = vec![
        (
            "input",
            Value::Obj(vec![
                ("datetime", s(iso(&chart.input, tz))),
                (
                    "gender",
                    s(match chart.gender {
                        Gender::Male => "male",
                        Gender::Female => "female",
                    }),
                ),
                ("gender_label", s(chart.gender.name())),
                ("timezone", s(tz_offset(tz))),
                ("late_zi_next_day", Value::Bool(chart.options.late_zi_next_day)),
            ]),
        ),
        (
            "pillars",
            Value::Obj(vec![
                ("year", pillar(chart.year, chart)),
                ("month", pillar(chart.month, chart)),
                ("day", pillar(chart.day, chart)),
                ("hour", pillar(chart.hour, chart)),
            ]),
        ),
        (
            "day_master",
            Value::Obj(vec![
                ("gan", s(dm.name())),
                ("wuxing", s(dm.wuxing().name())),
                ("yang", Value::Bool(dm.is_yang())),
            ]),
        ),
        (
            "term",
            Value::Obj(vec![
                ("solar_year", Value::Int(chart.solar_year as i64)),
                ("month_index", Value::Int(chart.month_index as i64)),
                ("current_name", s(chart.jie_name)),
                ("current_time", s(iso(&chart.jie_time, tz))),
                ("next_name", s(chart.next_jie_name)),
                ("next_time", s(iso(&chart.next_jie_time, tz))),
                ("days_into", Value::Num(chart.days_into_jie, 4)),
                ("solar_longitude", Value::Num(chart.solar_longitude(), 4)),
            ]),
        ),
        (
            "luck",
            Value::Obj(vec![
                (
                    "direction",
                    s(match luck.direction {
                        Direction::Forward => "forward",
                        Direction::Backward => "backward",
                    }),
                ),
                ("direction_label", s(luck.direction.name())),
                ("span_days", Value::Num(luck.span_days, 4)),
                (
                    "start_after",
                    Value::Obj(vec![
                        ("years", Value::Int(luck.start_years as i64)),
                        ("months", Value::Int(luck.start_months as i64)),
                        ("days", Value::Int(luck.start_days as i64)),
                    ]),
                ),
                ("handover_time", s(iso(&luck.start_time, tz))),
                (
                    "handover_age",
                    Value::Int(luck.da_yun.first().map(|d| d.start_age).unwrap_or(0) as i64),
                ),
                ("da_yun", Value::Arr(da_yun)),
                ("xiao_yun", Value::Arr(xiao_yun)),
            ]),
        ),
    ];

    // 自生年至末運末年，逐年自足——取用端不必自行接合大運與流年
    let years: Vec<Value> = liunian::all_years(chart, luck)
        .iter()
        .map(|sn| liunian_value(chart, sn))
        .collect();
    root_pairs.push(("years", Value::Arr(years)));

    Value::Obj(root_pairs).to_json()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::astro::DateTime;
    use crate::chart::Options;
    use crate::luck::compute;

    fn sample() -> String {
        let c = Chart::new(
            DateTime::new(2024, 9, 17, 18, 0, 0),
            Gender::Female,
            Options::default(),
        );
        let l = compute(&c, 10);
        chart_to_json(&c, &l)
    }

    #[test]
    fn escapes_control_and_quote() {
        assert_eq!(escape("a\"b"), "\"a\\\"b\"");
        assert_eq!(escape("a\\b"), "\"a\\\\b\"");
        assert_eq!(escape("a\nb"), "\"a\\nb\"");
        assert_eq!(escape("\u{1}"), "\"\\u0001\"");
        assert_eq!(escape("甲子"), "\"甲子\"");
    }

    #[test]
    fn timezone_formats() {
        assert_eq!(tz_offset(8.0), "+08:00");
        assert_eq!(tz_offset(-5.0), "-05:00");
        assert_eq!(tz_offset(5.5), "+05:30");
        assert_eq!(tz_offset(0.0), "+00:00");
    }

    /// 括號與字串須成對閉合。
    fn assert_balanced(j: &str) {
        let mut depth = 0i32;
        let mut in_str = false;
        let mut escaped = false;
        for c in j.chars() {
            if in_str {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '"' {
                    in_str = false;
                }
                continue;
            }
            match c {
                '"' => in_str = true,
                '{' | '[' => depth += 1,
                '}' | ']' => depth -= 1,
                _ => {}
            }
            assert!(depth >= 0, "括號提前閉合");
        }
        assert_eq!(depth, 0, "括號未閉合");
        assert!(!in_str, "字串未閉合");
    }

    #[test]
    fn braces_and_quotes_balance() {
        assert_balanced(&sample());
    }

    /// 給 agent 讀。字串外空白只燒 token，連結尾換行也不留。
    #[test]
    fn compact_no_whitespace_outside_strings() {
        let j = sample();
        let mut in_str = false;
        let mut escaped = false;
        for c in j.chars() {
            if in_str {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '"' {
                    in_str = false;
                }
                continue;
            }
            if c == '"' {
                in_str = true;
                continue;
            }
            assert!(
                !c.is_ascii_whitespace(),
                "字串外不應有空白 {c:?}"
            );
        }
        assert!(!j.is_empty());
    }

    fn liunian_json(year: i32) -> String {
        let c = Chart::new(
            DateTime::new(2003, 3, 18, 14, 0, 0),
            Gender::Female,
            Options::default(),
        );
        let l = compute(&c, 10);
        let snap = crate::liunian::snapshot(&c, &l, year);
        liunian_value(&c, &snap).to_json()
    }

    /// 生前之虛歲與行運作 `null`，不以 0、負值或下溢值充數。
    ///
    /// 命令列已無查生前流年之途（`years` 自生年起），但 `snapshot` 仍為
    /// 公開 API，函式庫使用者查得到，序列化不可因此漏接。
    #[test]
    fn pre_birth_age_and_fortune_serialise_as_null() {
        let j = liunian_json(1990);
        assert_balanced(&j);
        assert!(j.contains("\"age\":null"), "虛歲應為 null：{j}");
        assert!(j.contains("\"fortune\":null"), "行運應為 null：{j}");
        assert!(!j.contains("4294967"), "不應有下溢值：{j}");
        assert!(j.contains("\"ganzhi\":\"庚午\""), "應仍有流年干支：{j}");
        assert!(j.contains("\"1990-02-04"), "應仍有立春時刻：{j}");
    }

    /// 生後照舊為數值與物件，未被 Option 波及。
    #[test]
    fn post_birth_age_and_fortune_have_values() {
        let j = liunian_json(2025);
        assert_balanced(&j);
        assert!(j.contains("\"age\":23"), "{j}");
        assert!(!j.contains("\"fortune\":null"), "{j}");
        assert!(j.contains("\"kind\":\"da_yun\""), "{j}");
    }

    #[test]
    fn carries_the_four_pillars() {
        let j = sample();
        for expect in ["甲辰", "癸酉", "甲申", "白露", "寒露"] {
            assert!(j.contains(expect), "JSON 缺 {expect}");
        }
        assert!(j.contains("\"direction\":\"backward\""), "坤造甲年應逆行");
        assert!(j.contains("\"gan\":\"甲\""));
    }

    /// `handover_time` 只出現於大運，可用以計步數；
    /// `ordinal` 不行——流年的 fortune 物件也帶它。
    #[test]
    fn da_yun_count_matches() {
        let j = sample();
        assert_eq!(j.matches("\"handover_time\"").count(), 10 + 1); // 十步 + luck 頂層一處
    }

    /// 大運同時帶「交運當下之虛歲」與「所轄虛歲區間」，二者不可混為一談：
    /// 交運在虛歲 handover_age 之年中，其所轄自次一虛歲起。
    #[test]
    fn da_yun_separates_handover_age_from_covered_ages() {
        let c = Chart::new(
            DateTime::new(1990, 5, 15, 14, 30, 0),
            Gender::Male,
            Options::default(),
        );
        let l = compute(&c, 10);
        let j = chart_to_json(&c, &l);
        assert_balanced(&j);
        assert!(j.contains("\"handover_age\":8"), "{j}");
        assert!(j.contains("\"age_from\":9"), "{j}");
        assert!(j.contains("\"year_from\":1998"), "{j}");
    }

    /// 流年自生年鋪到末運末年，逐年不缺；虛歲與年份同步遞增。
    #[test]
    fn years_cover_birth_to_last_da_yun() {
        let c = Chart::new(
            DateTime::new(1990, 5, 15, 14, 30, 0),
            Gender::Male,
            Options::default(),
        );
        let l = compute(&c, 10);
        let j = chart_to_json(&c, &l);
        assert_balanced(&j);
        let last = l.da_yun.last().unwrap().year_range.1;
        let n = (last - c.solar_year + 1) as usize;
        assert_eq!(j.matches("\"gan_ten_god\"").count() >= n, true);
        for y in [c.solar_year, c.solar_year + 1, last] {
            assert!(j.contains(&format!("\"year\":{y}")), "缺流年 {y}");
        }
        // 生年虛歲一，末年虛歲 n
        assert!(j.contains("\"age\":1"), "{j}");
        assert!(j.contains(&format!("\"age\":{n}")), "缺末年虛歲 {n}");
    }

    /// 布林欄位須為 JSON 布林，不可用 0/1——取用端會把 0 當假、
    /// 卻也可能把它當成某種計數。
    #[test]
    fn booleans_are_real_booleans() {
        let j = sample();
        assert!(j.contains("\"late_zi_next_day\":true"), "{j}");
        assert!(j.contains("\"gan_yang\":true") || j.contains("\"gan_yang\":false"), "{j}");
        assert!(!j.contains("\"gan_yang\":1"), "{j}");
    }

    /// 真太陽時已移除，其欄位不應殘留。
    /// 注意 `solar_longitude` 是太陽視黃經，與出生地經度無涉，須留。
    #[test]
    fn no_birthplace_longitude_or_true_solar_time() {
        let j = sample();
        assert!(!j.contains("\"longitude\""), "出生地經度欄位應已移除：{j}");
        assert!(!j.contains("true_solar_time"), "真太陽時欄位應已移除：{j}");
        assert!(j.contains("\"solar_longitude\""), "太陽視黃經應保留：{j}");
    }
}
