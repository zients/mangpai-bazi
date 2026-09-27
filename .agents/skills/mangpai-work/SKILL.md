---
name: mangpai-work
description: Build 盲派體用與做功 paths from positioned participants and conditions. Use for 印化官殺、食神制殺、傷官合殺、食傷生財、比劫取財、通關、能量效率 and whether a target connects to 主位. Distinguish candidate paths, effectiveness, cost and possession.
---

# 體用與做功

Source: the review document, sections 八–九、十二、十五. Explain how a tool can act on a target and connect to 主位. The appearance of 財 or 官殺 alone does not establish a completed path.

## Inputs and dependencies

Use the original 日主 and positioned original / temporal participants. For a whole-chart judgment, obtain roots, 賓主, relations and support from their owning skills. Reuse existing complete results. For a fragment, evaluate only the supplied edges and leave missing participants or conditions unknown.

- [mangpai-roots](../mangpai-roots/SKILL.md): root eligibility, grades and 共根.
- [mangpai-binzhu](../mangpai-binzhu/SKILL.md): scope and root attribution.
- [mangpai-relations](../mangpai-relations/SKILL.md): matching relations and action limits.
- [mangpai-support](../mangpai-support/SKILL.md): 虛實、真假 and adopted support predicates.
- [path-patterns.md](references/path-patterns.md): candidate path forms and required checks.

## 1. Assign roles without changing identities

| Role | Adopted categories | Meaning |
| --- | --- | --- |
| 體 | 印、比劫、食傷 | Candidate tools or supporting channels. |
| 用 | 財、官殺 | Targets to process or obtain. |
| 主位 | Original 日、時 | The positional anchor supplied by 賓主. |

The original 日主 remains the 十神 reference. A tool is not inherently beneficial; a target is not automatically owned. This 用 is not 扶抑喜用神. 祿 can identify a specific support point already established by the root stage, not an independent actor detached from its branch.

List exposed tools and targets first. Include relevant hidden participants with their hidden positions and participation status. Absence of exposed 財 does not prove absence of all 財; a hidden 財 does not automatically act like an exposed stem.

## 2. Build concrete candidate paths

Instantiate only patterns whose required participants actually appear in the named scope. Use positioned arrows, for example `年干庚（七殺）→月干壬（偏印）→日干甲（日主）`, when those are the supplied facts. Each arrow needs its element relationship and candidate action status.

For a complete reading, inspect routes through present tools and targets, including competing control / generation paths. Group repeated forms only if their participants stay visible. Missing links stay missing; do not insert hypothetical 印、財 or an inferred 拱字 to complete a route.

For 陰日主, 傷官與七殺 may match 五合; check the actual pair. 陽日主 does not meet this source's 傷官合殺 condition. Even a valid pair remains subject to the relation skill's 合 gates.

## 3. Check each edge and the whole path

For every material route, record:

1. Tool and target, with positions, 十神 and exposed / hidden status.
2. Root and local-support facts, including what remains unresolved. A rootless participant still exists; do not erase it or assume it can act without conditions.
3. The action mechanism for each edge, and any competing relation or required hidden-stem participation.
4. The actual link to 日／時主位. 家外 targets require a traced route; 家內 location alone does not establish use or possession.
5. Obstructions, unsupported links, and conditional ways the path might operate. A potential alternative does not erase the original conflict.
6. Applicable 庫 questions through [mangpai-muku](../mangpai-muku/SKILL.md). Pass the target and path, receive the 庫 result, and update this same record; do not restart the whole skill chain.

`通關` requires both intermediary edges and conditions for effective transfer. The mere presence of the intermediate element is insufficient. For 財→官殺→印, retain the original 財剋印 relation until a stated rule and evidence justify a changed effect.

If action thresholds, root shares, or access conditions are undefined, conclude `路徑候選，成效／歸屬未定` with the exact missing condition. Do not force either success or failure from an incomplete rule.

## 4. Separate capacity, method and cost

Compare three questions: support available to the subject/tool, ability of the path to act on its target, and the cost/obstruction of that route. `能量` and `效率` are interpretive terms, not physical or numeric measurements.

The adopted method order is 印化官殺／食傷制合官殺較高，食傷取財其次，比劫直接取財較低. Report it as `方式序位`, independently of whether this particular route works. A missing high-ranked route cannot outrank a supported lower-ranked route in actual effectiveness. Do not translate the ordering into wealth, status, virtue, salary or percentage scores.

For 食傷生財, distinguish output-to-resource connection from outward expenditure of the subject. For direct 比劫取財, examine capacity and competing claimants. 共根 and 借根 do not determine numeric shares, ownership or retained proceeds (U12).

## Output

| 路徑 | 工具／目標 | 逐段作用及參與位置 | 根與支持 | 阻礙／庫條件 | 連主位方式 | 路徑狀態／未定 |
| --- | --- | --- | --- | --- | --- | --- |

Add a concise comparison of `支持／成效／方式序位／代價` when multiple paths matter. Explicitly distinguish existing relations, conditionally viable structures and access that has not been demonstrated. Hand only these bounded records to [mangpai-interpretation](../mangpai-interpretation/SKILL.md).
