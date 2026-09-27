# Existing chart engine

Use the project's `bazi` CLI for birth-data conversion. Do not modify application code to perform a reading. The repository README and current JSON fields define the engine interface.

```sh
cargo run --quiet -- '1990-05-15T14:30+08:00' --gender male
```

This command is an example, not a default birth date. Supply the user's data. `--gender` accepts male/female and the documented aliases; it is required by this CLI. `--luck N` controls 大運 count (default 10, maximum 12). `--early-zi` keeps 23:00 on the same civil date; the default advances the day at 23:00. Quote shell arguments safely.

## Input conventions

- Use local civil birth date and time with an explicit historical UTC offset. Do not silently assume `+08:00`, convert every locality to Beijing time, or infer a missing birth hour.
- The engine does not resolve IANA zones or true solar time. A place name alone is not an offset; unresolved historical/DST conversion stays unresolved until verified.
- Year changes at 立春; month changes at the twelve 節. They are not lunar-month or January boundaries.
- Offset converts the local instant to UTC for solar-term comparisons. Day and hour use the supplied local clock and selected late-子 rule.
- Supported years are 1600–2200. A supplied four-pillar fragment can still be discussed symbolically without fabricating a timestamp.
- If a time lies close to a solar-term boundary, report the input precision and engine convention. Do not silently choose a different pillar.

## JSON facts

`pillars.{year,month,day,hour}` contains `ganzhi`, `gan`, `gan_wuxing`, `gan_yang`, `gan_ten_god`, `zhi`, `zhi_wuxing`, `zhi_animal`, `nayin`, and `hidden[]` with `gan`, `role`, `wuxing`, `ten_god`.

Retain `input.datetime`, `input.timezone`, `input.late_zi_next_day`, `input.gender` and `day_master`. Do not substitute `hidden` for the root skill or treat every hidden stem as exposed. 納音 is available data; the supplied doctrine does not define an independent 納音 decision stage.

`luck.da_yun[]` gives `ganzhi`, `handover_time`, `handover_age`, and covered year/age ranges. `luck.xiao_yun[]` covers the period before 大運. Use the engine's result rather than manually deriving a different starting age or direction.

`years[]` gives solar-year `start` / `end`, 年干支, and the embedded `fortune` at the start of that interval. Optional `hand_over.time` and `hand_over.fortune` identify the within-year transition. The embedded fortune uses `ganzhi`; it need not contain separate `gan` / `zhi` fields.

`handover_age` is age at transition, while `age_from` / `age_to` describe covered solar years. Neither age nor `year_from` is a substitute for the transition timestamp. Use [mangpai-timing](../../mangpai-timing/SKILL.md) for interval selection and interpretation.
