---
name: mangpai-analysis
description: Run the complete 盲派理法 reading workflow from four pillars or bazi JSON, or route a focused question to the appropriate stage. Use for 完整看盤、理法分析、全流程、命盤綜合判讀. Coordinate 尋根基、賓主、干支作用、虛實真假旺衰、墓庫、體用做功、歲運與取象.
---

# 盲派理法總流程

Build a reading from positioned symbols, explicit rules, and traceable paths. Complete every applicable stage, preserving unresolved doctrine instead of supplying another school's rules.

The bundled rules compile the project's sibling source `../docs/bazi-foundations-review.md`, sections 一–十九. The user-approved split puts root eligibility in `mangpai-roots` and 主根／次根／借根 in `mangpai-binzhu`. These linked skills and references are sufficient at runtime; the sibling source is for maintenance and comparison.

## 1. Fix input and scope

Accept labeled 年、月、日、時 pillars, `bazi` JSON, or birth data for the existing chart engine. Distinguish a real chart, a synthetic chart, and an incomplete symbolic fragment. Do not demand birth data to analyze already supplied pillars or fragments.

- For birth-data conversion, follow [engine-input.md](references/engine-input.md). Ask only for missing data needed to compute the requested chart or timing; continue independent work.
- For four pillars, preserve the supplied order and source. Missing pillars remain unknown; do not infer them from a desired relation. A synthetic example need not be claimed as a possible birth timestamp.
- For JSON, retain `input`, `pillars`, `day_master`, and any requested temporal records. Check that the day master agrees with the day stem. Expose conflicting inputs rather than silently selecting one.
- State whether the request concerns the original chart, a specified time, or a rule. Separate missing chart data from an undefined rule.

For a focused request, run only the relevant stage and its necessary prerequisites. A pure relation lookup does not require a full root inventory; a whole reading does. Do not expand a roots-only request into life interpretation.

## 2. Establish facts

Use [foundations.md](references/foundations.md) for element, polarity, 藏干 and 十神 lookup. Reuse matching engine fields; do not treat `hidden` as a root verdict. Preserve exposed stems and hidden stems as different participants. Optional 十二長生 and symbolic labels do not override root eligibility.

Use [record-contract.md](references/record-contract.md) to keep positions, scopes, conditions, and certainty explicit throughout the reading. It is a reporting convention, not an additional 命理 rule.

## 3. Run the applicable stages

| Stage | Skill | Required result |
| --- | --- | --- |
| 尋根基 | [mangpai-roots](../mangpai-roots/SKILL.md) | All eligible original root positions, grades, exclusions, tied strongest roots and 共根; retain both tables. |
| 賓主與根歸屬 | [mangpai-binzhu](../mangpai-binzhu/SKILL.md) | Fixed 家內外, separate temporal scope, each stem's 主／次／借根. |
| 干支作用 | [mangpai-relations](../mangpai-relations/SKILL.md) | Positioned relation candidates, simultaneous relations and unresolved action conditions. |
| 虛實真假與旺衰 | [mangpai-support](../mangpai-support/SKILL.md) | Separate local support, whole-chart roots, support predicate and strength result. |
| 體用與做功 | [mangpai-work](../mangpai-work/SKILL.md) | Tools, targets, actual path participants, obstacles and connection to 主位. |
| 墓庫 | [mangpai-muku](../mangpai-muku/SKILL.md) | When 庫 occurs, distinguish its identity, root role, opening, entry and access; return results to the work paths. |
| 歲運 | [mangpai-timing](../mangpai-timing/SKILL.md) | When time is requested or supplied, compare dated overlays with the preserved original chart. |
| 取象與核實 | [mangpai-interpretation](../mangpai-interpretation/SKILL.md) | Structure-backed interpretations, alternatives and questions that can check them. |

Build candidate work paths before deciding what a 庫 is being used for. Then resolve applicable 庫 questions and update the path status. If no 庫 appears, mark this stage `不適用`; a root grade alone never establishes 開庫 or 入墓.

Temporal overlays may require revisiting relations, support, 庫 and paths **within the dated scope**. Do not replace the original records or rerun original root lookup with extra branches. With no timing request, finish the original reading and mark timing `未展開`; with timing requested but unavailable, list the specific missing input.

## 4. Deliver the reading

Respond in the user's language. For a full reading, organize results in this order:

1. Input, adopted conventions and known limits.
2. Basic pillar facts and the complete roots / attribution tables.
3. Key relations and the support table; distinguish candidate action from confirmed lookup.
4. Tool–target paths, applicable 庫 results and connection to 主位.
5. Requested temporal changes, with exact scope or interval.
6. Bounded interpretations and a short unresolved list.

For each material conclusion, show **理** (the adopted rule) and **法** (the concrete participants and checks), followed by the result and its limit. Keep facts supporting later conclusions available; merge repeated explanations rather than dropping positions or failed conditions.

Finish all independent judgments even when one rule is unresolved. Use [unresolved-rules.md](references/unresolved-rules.md) to name the missing definition and affected conclusion. Ask for a doctrine choice only when the user wants to settle it; do not interrupt each stage during a requested complete draft.

## Completion check

- Original participants, hidden participants, temporal participants and inferred symbols remain distinguishable.
- Root grades and 十神 identities survive all later interpretation unchanged.
- Every applied label has its rule, scope, participants and conditions; missing evidence stays unknown.
- Competing relations remain visible; a middle element, an opening, or a target's presence alone does not prove a completed path.
- Main / borrowed roots do not allocate shares or prove possession.
- Every requested stage has a result, `不適用`, or a specific unresolved reason.
- Interpretations do not exceed their supporting structures or turn symbolic examples into verified prediction.

## Maintenance

Use [workflow-cases.md](references/workflow-cases.md) for behavioral review after workflow changes. Do not load expected test results during an ordinary reading or a blind forward test.
