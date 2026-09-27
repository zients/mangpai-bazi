# Workflow maintenance cases

These are behavioral checks, not prediction validation. Use synthetic pillars as symbolic inputs. For blind forward tests, give the agent only the relevant skill entry point and raw inputs; keep the expectations below out of its task prompt.

The roots and 賓主 stages retain their own [roots fixtures](../../mangpai-roots/references/regression-cases.md) and [attribution fixtures](../../mangpai-binzhu/references/regression-cases.md). These cases check the remaining stages and their handoffs.

## W1 — Complete reading and temporal additions

Input: `年戊寅、月辛巳、日乙丑、時丙戌`; hypothetical `大運甲申、流年乙未`, without calendar dates. Request a full 理法 reading. No support predicate is supplied.

Review invariants:

- Roots analyze four original stems only. 戊 roots 月巳、日丑、時戌, excluding 年寅; 辛 roots 日丑、時戌, excluding 月巳; 乙 roots 年寅; 丙 roots 月巳、時戌, excluding 年寅.
- Root roles: 戊主月巳、借日丑時戌; 辛全借日丑時戌; 乙全借年寅; 丙主時戌、借月巳. Later timing does not mutate these.
- Relative to 乙: 戊正財、辛七殺、丙傷官. 丙辛 is a 陰日主 傷官合殺 candidate, not confirmed 合化.
- Keep 年寅月巳穿, 年寅日丑暗合 example, 月巳日丑拱酉, 時丙戌自合 example and 日丑時戌 opening trigger distinct. Inferred 酉 adds no original root.
- Original 丑戌 opening does not require a complete 丑未戌三刑. Original 寅巳 and 丑戌 are incomplete triples. Temporal 申／未 do not complete either original 三刑.
- Local roots determine 時丙坐實; all four stems have qualified original roots and are 真. Support-dependent local / strength judgments retain their missing predicate or explicit conditional treatment.
- 庫 contents and path access remain separate from opening. Complete tool–target and interpretation stages with bounded candidates, rather than ending at roots or skipping them because strength is unresolved.
- Temporal sources and original scopes remain separate; calendar interval is unspecified. All applicable stages have a result or a precise pending reason.

## W2 — Support edges with a supplied predicate

For these cases only, define branch support by 本行 being the subject's element or generating it, and stem 同黨 by the same element relationship. Ignore other hidden/support channels for this stated predicate.

Input A: `壬子 辛亥 甲午 戊戌`, asking about 日干甲.

- 亥 remains excluded as a wood root. Complete original root inventory has no 甲 root.
- 日支午 does not support 甲 under the supplied predicate: 坐虛／假.
- M=true, H=false, E=false: the source does not determine 旺 or 弱 for this boundary.

Input B: `戊寅 庚申 丙寅 壬申`, asking about 日干丙.

- Both 寅 positions are excluded as fire roots, but 日寅本行木 supplies the stated local support.
- 原局無合格火根, local 實, 真假未定; M=false and H=false give 弱 under the supplied predicate despite 日支 support.

## W3 — Named paths and incomplete action conditions

Fragments:

1. 日主乙，年干丙、時干辛.
2. 日主甲，年干丁、時干庚.
3. 日主甲，年干己、月干辛、時干癸.

Expect fragment 1 to qualify as a 傷官合殺 pairing candidate, while fragment 2 does not satisfy this source's 陰日主 rule or a 丁庚五合. Fragment 3 retains 己剋癸 and 己→辛→癸 together; the intermediary's presence alone does not prove 通關. Missing branches do not justify a whole-chart rootlessness claim.

## W4 — Mechanical matches, opening and group names

- `戊辰 己丑 甲卯 壬午`, asking 自合、暗合、刑、破: distinguish 戊辰 mechanical 自合 candidate from 壬午 listed example; retain 卯午破 and avoid importing other schools' 刑／破 lists. Additional hidden-stem matches, if scanned, must be labeled mechanical.
- `甲寅 乙丑 庚卯 丁未`, asking 庫: 丑未 is an opening trigger; original 丑未戌三刑 lacks 戌. Entry counts and actual access require their own conditions.
- `日柱丙戌，另有寅、卯`, asking 魁罡、驛馬、桃花: 丙戌 matches this local special-column list; personal 神煞 and effects remain unsupported by group membership or name alone.
- Compare fragments `寅、未` and `寅、戌`: a branch 本行's 庫 and a 三合生地–庫 relationship are different facts. The source's 四隅入墓 phrase does not define which participant/correspondence to use. 寅戌拱午 does not by itself establish 入墓 or add an actual 午.

## W5 — Engine boundaries

Generate an engine fixture from `2003-03-18T14:00+08:00 --gender female --luck 4` using the existing CLI. Use the returned values rather than hardcoding an astronomical timestamp here.

Ask for 2019-01-15, one second before the returned 2019 `hand_over.time`, the exact handover, and the complete 2019 solar-year interval.

- January resolves through the interval containing that timestamp, not the JSON record named 2019 by number alone.
- Before and at handover choose the respective embedded fortunes; the 流年 remains the same across that within-year transition.
- Whole-year output splits at the supplied handover. Coverage year/age fields do not override it.
- The engine's original pillar / root results remain unchanged across temporal records; changed relations carry temporal source labels.

## Review criteria

Run metadata validation and check local Markdown links. Inspect forward-test outputs for complete requested stages, root / 十神 consistency, correct time scopes, explicit unresolved conditions and traceable paths. A successful metadata check is not evidence of behavioral correctness, and passing synthetic cases is not evidence of real-life predictive accuracy.
