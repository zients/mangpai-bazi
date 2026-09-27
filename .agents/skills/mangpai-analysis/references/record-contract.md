# Positioned records

Use these fields in prose or tables; no JSON serialization or software implementation is required.

| Field | Meaning |
| --- | --- |
| 對象 | Exact character and position, e.g. 年干甲, 日支未, 時支巳藏庚（中氣）. |
| 來源 | 原局, 大運, 小運, 流年, or 推衍; never silently change source. |
| 範圍 | 本柱, own half, whole original chart, or a specified temporal interval. |
| 依據 | Lookup, adopted convention, supplied condition, or conditional inference. |
| 條件 | What is met, absent, or unresolved, including the rule used to count support. |
| 結果 | Symbolic fact, action candidate, supported structural reading, or human interpretation hypothesis. |
| 未定 | Missing input or undefined rule and which conclusion it prevents. |

Stable position labels can be short IDs such as `year.gan`, `day.zhi`, `hour.hidden[庚]`, `大運[甲子].zhi`. They identify participants, not new engine fields. Repeated characters at different positions retain different IDs.

Keep four distinctions:

- `查表成立`: the supplied symbols match a fixed table.
- `作用候選`: the relation exists, but action conditions are incomplete.
- `條件成立（列明依據）`: the stated conditions for this particular structural claim are met. It is not a prediction of a real-life event.
- `未定`: input or doctrine is insufficient. Do not replace it with `不成立` or numeric confidence.

`無` means a checked absence within a complete, named scope. `不適用` means the requested category has no applicable object. A missing pillar cannot justify `全局無根`; an unconfirmed 合化 cannot justify `沒有合`.

Upstream records remain available. Later effects attach to existing participants: `日支未根仍存在；功能受作用待判` is different from deleting the root. Temporal changes get new scope labels; they never rewrite original-chart facts.
