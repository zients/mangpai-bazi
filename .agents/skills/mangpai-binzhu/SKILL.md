---
name: mangpai-binzhu
description: Classify 盲派賓主 and root attribution after 尋根基. Use for 家內／家外, original-chart versus luck-pillar scope, or 主根／次根／借根. Reuse mangpai-roots for root eligibility, grades, positions, and shared roots.
---

# 賓主與根的歸屬

Separate fixed chart positions from each stem's root attribution. This stage assigns scope and root roles; 體用, root shares, usable resources, strength, and life interpretations belong to later stages.

For a complete reading, return both tables to [mangpai-analysis](../mangpai-analysis/SKILL.md). Downstream stages reuse this attribution rather than choosing a different main root from strength or work-path results.

## Inputs and handoff

Accept four pillars labeled 年、月、日、時, or `bazi` JSON at `pillars.{year,month,day,hour}`. Luck pillars are optional context with their source retained.

For root attribution, use both tables from [mangpai-roots](../mangpai-roots/SKILL.md): every original stem position, every eligible root position and grade, exclusions, and shared-root pairs. A strongest-root summary alone is insufficient. If complete root results are unavailable, run that skill first. If only a scope question is asked, complete step 1 without requesting a chart or running root lookup unnecessarily.

Root eligibility and grade ordering have one source: `mangpai-roots`. Preserve its results. Consult its grade rules when needed; do not introduce another root table here. Keep unknown pillars or grades unresolved rather than treating missing data as no root.

## 1. Fix the comparison scope

| 比較範圍 | 主 | 賓 |
| --- | --- | --- |
| 原局內，以日主為視角 | 日、時＝家內 | 年、月＝家外 |
| 原局與歲運相比 | 整個原局 | 大運、流年 |

家內／家外 always describes original-chart position from the day master's viewpoint. A 日支未 is 家內 for every stem, including a year or month stem. A 月支 remains 賓／家外 within the original chart, while still belonging to 主 when comparing the whole chart with luck pillars.

Keep the two scopes in separate columns. Luck pillars are 賓 in the second scope; do not relabel them as original 年月 positions or add their branches to the original root table. 主／賓 describes position, not good/bad outcomes or possession of resources.

## 2. Find each stem's own half

| 天干位置 | 自己那半 | 對面那半 |
| --- | --- | --- |
| 年、月 | 年月＝家外 | 日時＝家內 |
| 日、時 | 日時＝家內 | 年月＝家外 |

Apply this independently to all four stem positions, including repeated stems. Root attribution is relative to the particular stem; fixed 家內／家外 labels do not change with that stem.

## 3. Assign root roles

Partition every eligible root by the stem's own half, then apply these rules:

- A uniquely highest-grade root in its own half is 主根. Lower-grade roots in that half are 次根.
- If several roots tie for highest grade in its own half, retain all positions as `主根候選並列，取捨未定`. Lower-grade roots remain 次根. Do not invent a day-position, 月令, or 時支 priority.
- Every root in the opposite half is 借根, even when 主根 or tied candidates exist.
- No root in its own half, but roots across the line: record `無主根，全靠借根`.
- A complete chart with no eligible root: record 主根、次根、借根 all as `無`. An incomplete root inventory remains `未定`; label any identified roots as provisional.

Grades stay unchanged. The strongest root across the whole chart can be borrowed, while a lower-grade root in the stem's own half is its main root. A shared root can have different roles for the stems sharing it; preserve the original shared-root relation and classify each stem separately. This step does not allocate shares or decide how much support can actually be used.

## 4. Output

For a complete chart, produce two tables. A scope-only request needs only the applicable scope explanation.

**表一：賓主位置**

| 來源 | 柱／字 | 原局內：主賓／家內外 | 與歲運相比：主賓 |
| --- | --- | --- | --- |

Include four original pillar rows. If luck pillars were supplied, append them with `不適用（非原局位置）` in the original-position column and `賓` in the chart-versus-luck column. Preserve the source label 大運 or 流年.

**表二：逐干根歸屬**

| 天干（柱） | 自己那半 | 主根／並列候選 | 次根 | 借根 | 備註 |
| --- | --- | --- | --- | --- | --- |

List each root's branch position and inherited grade. Use `無` for confirmed absences and `未定` for unresolved choices or missing data. The notes can identify 全靠借根, strongest-root versus main-root differences, or a shared root's different roles. Repeated branches remain separate entries. Do not reduce multiple borrowed roots to just the strongest one.

Completion requires every eligible original root to appear in a role column, or among the explicit tied candidates, for its stem. The assignment must preserve the upstream root inventory and grades.

## Example

Four pillars: `丙寅 庚寅 乙未 辛巳`. The root lookup supplies 丙 with 時支巳（祿）and 日支未（餘氣庫）, 乙 with 年支寅／月支寅（比劫）and 日支未（本氣庫）, and no eligible root for 庚 or 辛.

| 來源 | 柱／字 | 原局內：主賓／家內外 | 與歲運相比：主賓 |
| --- | --- | --- | --- |
| 原局 | 年柱丙寅 | 賓／家外 | 主 |
| 原局 | 月柱庚寅 | 賓／家外 | 主 |
| 原局 | 日柱乙未 | 主／家內 | 主 |
| 原局 | 時柱辛巳 | 主／家內 | 主 |

| 天干（柱） | 自己那半 | 主根／並列候選 | 次根 | 借根 | 備註 |
| --- | --- | --- | --- | --- | --- |
| 丙（年） | 年月 | 無 | 無 | 時支巳（祿）、日支未（餘氣庫） | 全靠借根；兩根均在家內 |
| 庚（月） | 年月 | 無 | 無 | 無 | 原局無合格根 |
| 乙（日） | 日時 | 日支未（本氣庫） | 無 | 年支寅、月支寅（比劫） | 兩處寅是同級最強根，主根仍為未 |
| 辛（時） | 日時 | 無 | 無 | 無 | 原局無合格根 |

The 日支未 stays 家內 for both 丙 and 乙. It is borrowed by 年干丙 and is the main root of 日干乙. Their elements differ, so this does not create 共根.

Year/month counterexample: for 月干乙 in `癸未 乙卯 庚寅 癸未`, 月支卯（祿）is 主根, 年支未（本氣庫）is 次根, and 日支寅（比劫）plus 時支未（本氣庫）are 借根. Its main root is 家外, while its borrowed roots are 家內.

## Self-check

- [ ] The original-position scope and chart-versus-luck scope remain separate.
- [ ] 家內＝日時 and 家外＝年月 stay fixed for every stem.
- [ ] Every stem uses its own half; year/month stems are not treated as day/hour stems.
- [ ] Every upstream eligible root retains its position and grade and receives a role or explicit tied-candidate status.
- [ ] Main roots and borrowed roots can coexist; all borrowed positions are retained.
- [ ] Tied main candidates stay unresolved; 旺點 labels do not break ties.
- [ ] Shared-root pairs remain intact while roles are assigned separately.
- [ ] Root lookup, resource shares, 體用, ability, and life outcomes are not redefined here.

## Maintenance validation

After changing this skill or its handoff, use [regression-cases.md](references/regression-cases.md). Load those cases only for maintenance, not routine readings.
