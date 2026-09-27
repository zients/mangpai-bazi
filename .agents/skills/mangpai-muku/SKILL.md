---
name: mangpai-muku
description: Analyze 盲派墓庫用途、開庫、入墓 and access to stored targets after basic relations and 賓主. Use for 辰戌丑未、財官庫、丑未戌任兩支開庫 or 同五行兩見入墓. Keep 庫 identity, root grade, opening, entry and actual access separate.
---

# 墓庫與取用

Source: the review document, sections 六、十六. A 庫, a root, entry into 墓, opening of 庫, and access to its contents are five distinct claims.

## Inputs

Accept positioned original and explicitly labeled temporal branches, original 日主, relevant hidden stems, relation records, and any tool–target path being examined. Use [foundations.md](../mangpai-analysis/references/foundations.md) for hidden stems / 十神, [mangpai-binzhu](../mangpai-binzhu/SKILL.md) for scope, and [mangpai-roots](../mangpai-roots/SKILL.md) for existing root grades. A lookup-only question does not need a complete chart.

## 1. Identify each actual 庫

| 支 | 五行庫 | Environment |
| --- | --- | --- |
| 辰 | 水庫 | 濕土 |
| 未 | 木庫 | 燥土 |
| 戌 | 火庫 | 燥土 |
| 丑 | 金庫 | 濕土 |

The additional `丑未戌入辰，以辰為土庫` is a separate adopted use. It neither changes 十二長生 nor creates a 土 root grade. Preserve the actual entering object and conditions; do not infer entry merely from the 土庫 name.

Determine a 庫's target role relative to the original day master and the specific stored element / hidden stem. 辰 is not universally 財庫: water is 印 for a wood day master and 財 for an earth day master. Do not collapse all hidden contents into the 庫's named element.

Reuse qualified roots without modification. A root's 本氣庫／餘氣庫 label does not establish 開庫, 入墓, or useful storage; 燥濕 does not upgrade a root.

## 2. Check 開庫 triggers

The adopted triggers are **辰戌相沖, or any two distinct members of 丑未戌 meeting**: 丑未、丑戌、未戌. Two occurrences of the same branch are not two distinct members.

For each matching positional pair, record `開庫觸發成立（本文約定）`, which branch / contents are being discussed, and the scope. This opening convention does not require the complete 原局丑未戌三刑 and does not establish a general two-branch 刑 rule.

The result identifies a trigger. Whether a specific content is released, damaged, usable, or obtained remains a separate path judgment. Temporal triggers apply only in their dated scope and cannot be reported as original-chart openings.

## 3. Check 入墓 candidates

The source proposes `同五行兩見再見庫；四隅見相應庫另論`. First inventory the actual same-element objects and the corresponding 庫. Keep exposed stems, branches, and hidden stems distinguishable; the unit counted, overlapping representations, and effective participation are unresolved (U07).

- Without an adopted counting rule, list candidates and the competing possible counts. Do not count one branch, its 本氣 and a matching exposed stem as three established independent entries by default.
- If a counting rule is explicitly supplied, state it and its scope, then test its conditions. Do not extend a one-case assumption into a universal rule.
- For 四隅, record the branch, its 本行／藏干, and the proposed receiving 庫. The phrase 相應庫 does not specify which participant or correspondence controls entry. Keep that basis explicit and unresolved unless supplied; do not silently substitute the 三合生地–庫 pairing or the branch 本行's 庫. A separate 拱局 match does not establish 入墓.
- Distinguish a 十二長生 墓 phase, an element's 庫, and a specific participant entering 墓. No one substitutes for the others.

Unknown entry conditions do not block the already determined 庫 identity or 開庫 trigger.

## 4. Connect contents to 主位

For each target being used, trace `庫／藏干 → 參與作用的工具或轉接者 → 主位`, retaining every participant and edge. Reuse [mangpai-work](../mangpai-work/SKILL.md) for path checks; if called from that skill, return the 庫 result for the caller to update the existing path rather than restarting the workflow.

Name obstructions, unresolved hidden-stem participation, possible damage and access conditions. Location in 日時, existence of a 財庫, or a trigger alone cannot prove possession, wealth, office or loss.

## Output

| 庫與位置／來源 | 庫五行及目標十神 | 相關根／藏氣 | 開庫觸發 | 入墓對象與計數口徑 | 連主位路徑 | 可定／未定 |
| --- | --- | --- | --- | --- | --- | --- |

Preserve all applicable positional triggers, even when repeated symbols occur. If no actual 庫 is present in the requested scope, state `不適用`; an inferred symbol is not an actual 庫.
