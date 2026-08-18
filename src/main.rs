//! 命令列排盤。輸出 JSON。
//!
//! 人要好看的版面請接 JSON 自行呈現；本工具只負責算，不負責排版。

use mangpai_bazi::astro::DateTime;
use mangpai_bazi::chart::{Chart, Gender, Options};
use mangpai_bazi::{compute_luck, json};

const USAGE: &str = "\
用法：bazi <RFC3339 時刻> --gender <male|female> [選項]

  bazi 1990-05-15T14:30+08:00 --gender male
  bazi 1990-05-15T14:30:00+08:00 -g f --luck 12

時刻須為出生地之當地民用時，且必須自帶時區偏移。八字以北京時間為準，
台灣同此時區，寫 +08:00；生於他處者寫該地之偏移，如 -05:00。

偏移與時辰皆不可省，本工具不代為假定——時區猜錯即排錯年月柱，
時辰猜錯即排錯時柱，二者都不會報錯，只會靜靜地給你一張錯盤。

輸出為 JSON，涵蓋四柱、藏干、十神、納音、節氣、大運、小運，
及自生年至末運末年的每一個流年。

選項
  -g, --gender <male|female>  性別，男 male/m、女 female/f。必填。
      --luck <n>              大運步數，預設 10，上限 12。
      --early-zi              23 時仍算當日，不進次日子時。
  -h, --help                  顯示本說明。
";

fn main() {
    match run() {
        Ok(s) => print!("{s}"),
        Err(e) => {
            eprintln!("錯誤：{e}\n\n{USAGE}");
            std::process::exit(1);
        }
    }
}

fn run() -> Result<String, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        return Ok(USAGE.to_string());
    }

    let mut positional: Vec<&str> = Vec::new();
    let mut gender: Option<Gender> = None;
    let mut opts = Options::default();
    let mut luck_count: u32 = 10;

    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        let mut next = |name: &str| -> Result<String, String> {
            i += 1;
            args.get(i).cloned().ok_or_else(|| format!("{name} 缺少參數值"))
        };
        match a {
            "-g" | "--gender" => {
                let v = next("--gender")?;
                gender = Some(match v.to_lowercase().as_str() {
                    "male" | "m" | "男" | "乾" => Gender::Male,
                    "female" | "f" | "女" | "坤" => Gender::Female,
                    other => return Err(format!("性別無法辨識：{other}")),
                });
            }
            "--luck" => {
                luck_count = next("--luck")?
                    .parse()
                    .map_err(|_| "大運步數須為正整數".to_string())?;
            }
            "--early-zi" => opts.late_zi_next_day = false,
            other if other.starts_with('-') => return Err(format!("未知選項：{other}")),
            other => positional.push(other),
        }
        i += 1;
    }

    let gender = gender.ok_or("必須指定 --gender")?;
    if !(1..=12).contains(&luck_count) {
        return Err("大運步數須介於 1 至 12".into());
    }
    let (dt, tz) = parse_rfc3339(&positional.join(" "))?;
    opts.tz = tz;

    let chart = Chart::new(dt, gender, opts);
    let luck = compute_luck(&chart, luck_count);
    Ok(json::chart_to_json(&chart, &luck))
}

/// 解析 RFC 3339 時刻，回傳民用時與時區偏移（小時）。
///
/// 接受 `2024-09-17T18:00:00+08:00`、`2024-09-17T18:00+08:00`、
/// `2024-09-17T18:00:00Z`。日期與時刻之間亦可用空白替代 `T`。
///
/// 時分與時區偏移皆為必填。漏給時分即排錯時柱，漏給偏移即排錯年月柱，
/// 二者都不會顯露為錯誤，故寧可報錯不可默補。
fn parse_rfc3339(raw: &str) -> Result<(DateTime, f64), String> {
    let s = raw.trim();
    // 位元組切割前先確認邊界——全形數字等非 ASCII 會落在字元中間
    if s.len() < 10 || !s.is_char_boundary(10) || !s.is_ascii() {
        return Err(format!(
            "時刻須為半形 RFC 3339 格式，如 1990-05-15T14:30+08:00，收到「{raw}」"
        ));
    }

    let (date, rest) = s.split_at(10);
    let d: Vec<&str> = date.split('-').collect();
    if d.len() != 3 || d[0].len() != 4 {
        return Err(format!("日期須為 YYYY-MM-DD，收到「{date}」"));
    }
    let num = |x: &str, what: &str| -> Result<i64, String> {
        x.parse::<i64>().map_err(|_| format!("{what}無法解析：{x}"))
    };
    let (y, mo, dd) = (num(d[0], "年")?, num(d[1], "月")?, num(d[2], "日")?);

    let rest = rest.trim();
    if rest.is_empty() {
        return Err(format!(
            "須給出生時分與時區偏移，格式為 YYYY-MM-DDTHH:MM+08:00，收到「{raw}」"
        ));
    }

    let body = rest.strip_prefix('T').or_else(|| rest.strip_prefix('t')).unwrap_or(rest);
    let (clock, tz) = split_offset(body)?;
    let tz = tz.ok_or_else(|| {
        format!(
            "時刻須自帶時區偏移，如 +08:00 或 -05:00，收到「{raw}」。\n\
             本工具不預設時區——猜錯會靜默排出錯盤。"
        )
    })?;

    let c: Vec<&str> = clock.split(':').collect();
    if c.len() < 2 {
        return Err(format!("時刻須為 HH:MM[:SS]，收到「{clock}」"));
    }
    let (h, mi) = (num(c[0], "時")?, num(c[1], "分")?);
    let sec = match c.get(2) {
        // 容許小數秒，逕行捨去
        Some(x) => num(x.split('.').next().unwrap_or("0"), "秒")?,
        None => 0,
    };

    Ok((validated(y, mo, dd, h, mi, sec)?, tz))
}

/// 自時刻尾端切出時區偏移，回傳（時刻, 偏移小時）。
fn split_offset(body: &str) -> Result<(&str, Option<f64>), String> {
    if let Some(clock) = body.strip_suffix(['Z', 'z']) {
        return Ok((clock, Some(0.0)));
    }
    for (i, ch) in body.char_indices().rev() {
        if ch == '+' || ch == '-' {
            let (clock, off) = body.split_at(i);
            let sign = if ch == '-' { -1.0 } else { 1.0 };
            let parts: Vec<&str> = off[1..].split(':').collect();
            let hh: f64 = parts[0].parse().map_err(|_| format!("時區偏移無法解析：{off}"))?;
            let mm: f64 = match parts.get(1) {
                Some(x) => x.parse().map_err(|_| format!("時區偏移無法解析：{off}"))?,
                None => 0.0,
            };
            if hh > 14.0 || mm >= 60.0 {
                return Err(format!("時區偏移超出範圍：{off}"));
            }
            return Ok((clock, Some(sign * (hh + mm / 60.0))));
        }
    }
    Ok((body, None))
}

/// 該年該月之日數。
fn days_in_month(y: i64, m: i64) -> i64 {
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
        _ => 0,
    }
}

fn validated(y: i64, mo: i64, d: i64, h: i64, mi: i64, sec: i64) -> Result<DateTime, String> {
    if !(1600..=2200).contains(&y) {
        return Err(format!("年份 {y} 超出支援範圍 1600–2200"));
    }
    if !(1..=12).contains(&mo) {
        return Err(format!("月份 {mo} 不合法"));
    }
    // 須按當月實際日數核驗。逕放行則儒略日換算會靜默滾入次月，
    // 排出一張看似無誤而實則錯日之盤。
    let last = days_in_month(y, mo);
    if !(1..=last).contains(&d) {
        return Err(format!("{y} 年 {mo} 月無 {d} 日，該月共 {last} 日"));
    }
    if !(0..=23).contains(&h) || !(0..=59).contains(&mi) || !(0..=59).contains(&sec) {
        return Err("時刻不合法".into());
    }
    Ok(DateTime::new(
        y as i32, mo as u32, d as u32, h as u32, mi as u32, sec as u32,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(s: &str) -> (DateTime, f64) {
        parse_rfc3339(s).expect(s)
    }

    #[test]
    fn parses_rfc3339_forms() {
        let (dt, tz) = ok("2024-09-17T18:00:00+08:00");
        assert_eq!((dt.year, dt.month, dt.day, dt.hour, dt.minute), (2024, 9, 17, 18, 0));
        assert_eq!(tz, 8.0);

        assert_eq!(ok("2024-09-17T18:00+08:00").1, 8.0);
        assert_eq!(ok("2024-09-17T10:00:00Z").1, 0.0);
        assert_eq!(ok("2024-09-17T06:00:00-04:00").1, -4.0);
        assert_eq!(ok("2024-09-17T15:30:00+05:30").1, 5.5);
        // 日期與時刻間以空白相隔亦可
        assert_eq!(ok("2024-09-17 18:00+08:00").0.hour, 18);
        // 小數秒逕行捨去
        assert_eq!(ok("2024-09-17T18:00:30.750+08:00").0.second, 30);
    }

    /// 時區偏移不可省。預設為北京時間會靜默排出錯盤，
    /// 而錯盤與對盤在輸出上毫無分別。
    #[test]
    fn rejects_missing_timezone_offset() {
        for bad in ["2024-09-17T18:00", "2024-09-17 18:00", "2024-09-17T18:00:00"] {
            let e = parse_rfc3339(bad).expect_err(bad);
            assert!(e.contains("時區偏移"), "{bad} 之錯誤訊息應提及時區偏移：{e}");
        }
    }

    #[test]
    fn rejects_missing_time() {
        assert!(parse_rfc3339("2024-09-17").is_err());
    }

    /// 全形數字等非 ASCII 曾使位元組切割落在字元中間而 panic。
    #[test]
    fn rejects_non_ascii_without_panicking() {
        for bad in [
            "２０２４-09-17T18:00+08:00",
            "2024-09-17T18：00+08:00",
            "二〇二四年九月十七日",
            "甲",
        ] {
            assert!(parse_rfc3339(bad).is_err(), "應拒絕 {bad}");
        }
    }

    /// 不存在之日期須報錯。放行則儒略日換算靜默滾入次月。
    #[test]
    fn rejects_nonexistent_dates() {
        for bad in [
            "2023-02-29T12:00+08:00",
            "2024-02-30T12:00+08:00",
            "2023-04-31T12:00+08:00",
            "2023-06-31T12:00+08:00",
            "2023-09-31T12:00+08:00",
            "2023-11-31T12:00+08:00",
        ] {
            assert!(parse_rfc3339(bad).is_err(), "應拒絕 {bad}");
        }
        assert!(parse_rfc3339("2024-02-29T12:00+08:00").is_ok());
        assert!(parse_rfc3339("2000-02-29T12:00+08:00").is_ok()); // 四百年閏
        assert!(parse_rfc3339("1900-02-29T12:00+08:00").is_err()); // 百年不閏
        for (m, last) in [(1, 31), (4, 30), (7, 31), (11, 30), (12, 31)] {
            let s = format!("2023-{m:02}-{last}T12:00+08:00");
            assert!(parse_rfc3339(&s).is_ok(), "應接受 {s}");
        }
    }

    #[test]
    fn rejects_malformed_input() {
        for bad in [
            "2024-13-17T18:00+08:00",
            "2024-09-32T18:00+08:00",
            "2024-09-17T25:00+08:00",
            "2024-09-17T18:70+08:00",
            "2024-09-17T18:00+25:00",
            "1500-09-17T18:00+08:00",
            "24-09-17T18:00+08:00",
            "abc",
        ] {
            assert!(parse_rfc3339(bad).is_err(), "應拒絕 {bad}");
        }
    }
}
