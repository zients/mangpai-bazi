---
name: mangpai-timing
description: Compare 大運、小運 and 流年 with a preserved original 盲派 chart. Use for 歲運分析、交運前後、流年引動 or timing of candidate paths. Resolve bazi JSON solar-year and hand_over boundaries, then assess scoped changes without backfilling original roots or 三刑.
---

# 歲運與引動

Keep original-chart facts stable and evaluate what a specified temporal participant changes. The review document supplies the scope distinction and relation boundaries; the interval procedure below comes from the existing engine contract. Neither supplies a deterministic event-timing formula.

## Inputs

Accept an original chart plus `bazi` JSON temporal records, or explicitly labeled 大運／小運／流年 pillars. Accept a requested date, year or interval. Reuse the original roots, 賓主, relations, support and work paths; build only the missing prerequisites for the requested comparison.

For birth-data conversion or JSON fields, use [engine-input.md](../mangpai-analysis/references/engine-input.md). Pillars supplied without dates support a symbolic comparison, not a fabricated calendar interval. If timing is requested but absent, finish original-chart work and identify the missing time data.

## 1. Resolve the actual interval

For engine JSON:

1. A timestamp belongs to the `years[]` record with `start <= t < end`; compare actual instants with their offsets. A date without time that straddles a transition must retain both possibilities. Do not choose by Gregorian year number alone.
2. `fortune` describes the fortune at the solar year's start. If `hand_over` exists, use that fortune for `[start, hand_over.time)` and `hand_over.fortune` from the transition onward. At the exact supplied transition timestamp, use the new fortune.
3. If the request covers a whole interval crossing 立春 or a handover, split it at every relevant boundary. Keep the same 流年 before and after a within-year 大運 handover.
4. `years[].year` names a solar-year record, whose actual `start` / `end` must be displayed. A request explicitly about a Gregorian calendar year may intersect two such records; retain both. For a year-only reading using the solar-year convention, state that convention.
5. Prefer embedded `fortune` / `hand_over` to rejoining coverage ranges. `handover_age`, `age_from` and `year_from` cannot override actual transition timestamps. Use 小運 when it is the supplied current fortune; do not relabel it 大運. Preserve `null` / missing results rather than inventing a fortune.

If only labeled temporal pillars are supplied, retain their labels and mark the calendar interval `未提供`. Do not manufacture a correspondence to a Gregorian year, decade or age.

## 2. Preserve the two 賓主 scopes

Original 日時 remain 家內／主 and 年月 remain 家外／賓 within the original chart. Against temporal additions, the entire original chart is 主 and the supplied luck/year participants are 賓. A temporal branch is not a new 原局年月 position.

All temporal 十神 use the original 日主. Distinguish 大運, 小運 and 流年 sources even when their characters repeat. No source priority or numeric temporal weight is defined here.

## 3. Identify changes, then evaluate them

For each interval:

- Add only the temporal participants actually supplied for that interval. Preserve inferred symbols separately.
- Run [mangpai-relations](../mangpai-relations/SKILL.md) on newly relevant pairs/triples, including relevant luck–year interactions, with full source labels. Retain simultaneous original relations.
- Original 三刑 requires original three-branch completeness. A temporal third branch may create a mixed-scope candidate, never an original triple; additional effects of that mixed triple remain subject to defined rules.
- If a temporal branch matches a root position, reuse the eligibility / grade lookup from [mangpai-roots](../mangpai-roots/SKILL.md) in a separate `歲運根位對應／助力候選` record. Keep the original four-stem/four-branch root inventory unchanged. Temporal root roles, actual support strength and duration beyond the interval are not inferred from an original 主／次／借 label.
- Evaluate support changes under the same declared predicate through [mangpai-support](../mangpai-support/SKILL.md). A temporary helper does not change the baseline claim `原局有根／無根`. Do not silently convert 原局真假 into a new permanent verdict.
- Check newly triggered 庫 actions through [mangpai-muku](../mangpai-muku/SKILL.md), then compare the affected tools, targets and edges through [mangpai-work](../mangpai-work/SKILL.md). Return changes to the existing paths rather than restarting original lookup.

For each change, answer: **which original object or path is touched, by which temporal participant, by which relation, under which conditions?** If only a pair is known, report 引動候選; an exact date requires supplied time data and a defined timing rule. Do not infer an event from a clash, repeated character, target arrival or opening alone.

## Output

| 時段與邊界 | 流年 | 所行大運／小運 | 原局對象／原有路徑 | 歲運新增關係 | 支持／庫／做功變化 | 可定／未定 |
| --- | --- | --- | --- | --- | --- | --- |

Show before/after handover rows when applicable. Keep the baseline visible by reference and summarize only actual changes. Pass dated, conditional structures to [mangpai-interpretation](../mangpai-interpretation/SKILL.md); do not invent exact event dates or certainty.
