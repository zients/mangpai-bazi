---
name: mangpai-relations
description: Identify positioned 盲派干支作用 candidates, including 生剋、五合、干沖、六合、三合三會、半合拱局、沖穿刑破、自合暗合. Use when checking which relations exist and which action conditions remain unresolved; do not infer 合化 or damage from a matching pair alone.
---

# 干支作用

Separate a table match from an effective action. Source: the review document, sections 二–七、十六. Load [relation-tables.md](references/relation-tables.md) for exact accepted combinations; use [foundations.md](../mangpai-analysis/references/foundations.md) for element and hidden-stem lookup.

## Inputs

Accept positioned pillars or a labeled fragment. Reuse roots and 賓主 results when available. A matching question needs only the relevant symbols; evaluating action requires positions and conditions. Keep original, temporal, and inferred participants distinct. Repeated characters at different positions create separate matches.

## Procedure

1. **Inventory participants.** Record each exposed stem, branch, and relevant hidden stem with its position. A hidden participant is not an exposed one. Keep temporal additions in their supplied scope.
2. **Find direct relations.** Check positioned stem pairs for generation, control, 五合 and 干沖; branch pairs/triples for the accepted tables. A branch 本行 generation/control candidate is separate from a particular 藏干 action. Do not silently treat all hidden cross-products as effective edges.
3. **Check completeness.** Record complete triples, 半合, and 拱 endpoints accurately. If a middle branch is already present, retain its actual position and the complete 三合; do not describe it as an absent inferred symbol. If absent, mark the middle symbol `推衍，非原局實字`.
4. **Apply local conventions.** 原局三刑 requires all three original branches. For 寅巳申, explicitly list 寅巳穿、巳申合、寅申沖. Temporal participation is separately labeled; it does not complete an original triple. 庫 opening uses [mangpai-muku](../mangpai-muku/SKILL.md), not an invented two-branch 三刑 rule.
5. **Inspect 自合／暗合.** Distinguish listed examples from additional mechanical matches. For each, name the actual hidden stem and 五合 pair. Both kinds still need conditions for action. Do not convert “not listed” into either forbidden or confirmed.
6. **Assess action only as far as supported.** Reuse root eligibility, positions, and support records. Record any supplied action criterion and its evidence. If 季節、根氣、助力、位置、受制 or priority thresholds are undefined, retain a candidate and name the missing criterion.

For a complete relation scan, retain every matched positional pair/triple in the working record. Group identical rule names in display only if every position remains visible. For a targeted question, limit the scan to the involved objects and relevant competing relations.

## Conditions and boundaries

- 有合 identifies a pair. 合絆、合化、反化 are separate claims; the proposed transformation element does not replace participants' original element or 十神 automatically.
- 爭合 means one participant has multiple matching counterparts; preserve every pair before considering effects.
- 合 does not universally cancel 沖; being controlled does not automatically make a stem unable to 合. State any actual interference mechanism and unresolved priority.
- For 沖, separate 引動, change of an existing path, and functional damage. The source names 相當→沖起、弱沖強→沖動、強沖弱或同屬性→沖壞, but comparison thresholds and 同屬性 scope are unresolved. Even an adopted 沖壞 label does not by itself prove a damaged root or life loss.
- 穿 does not delete all roots located in that branch. Attach any possible functional effect to the particular root or path; preserve its lookup eligibility.
- A generation/control relation and an effective 通關 path are different. Hand path questions to [mangpai-work](../mangpai-work/SKILL.md).

## Output

| 範圍 | 參與者與柱位 | 關係／所向 | 查表或候選依據 | 作用條件 | 可定結果／未定 |
| --- | --- | --- | --- | --- | --- |

Keep `五合配對成立／合化未定` and similar distinctions in the same row. Where relevant, add a short list of competing relations, inferred symbols excluded from the original inventory, and the specific pending rule IDs in [unresolved-rules.md](../mangpai-analysis/references/unresolved-rules.md).

## Self-check

- Every edge has actual positions and sources; hidden stems stay hidden.
- No absent 拱 symbol is added as a branch or root.
- 原局三刑 and mixed-time candidates are separated; 破 uses only the local table.
- 自合／暗合 examples and mechanical extensions are distinguishable.
- Pair existence is not reported as completed transformation, 通關, damage, or possession.
