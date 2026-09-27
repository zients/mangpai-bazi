# 賓主 handoff regression cases

Reuse fixture inputs A–F from [the root cases](../../mangpai-roots/references/regression-cases.md). Root expectations stay there; this file checks only scope and root attribution.

For blind evaluation, provide the evaluator with both skills and raw fixture inputs, not either expected-results file. First request root lookup only and check that it stops before 家內／家外 or root roles. Then feed those root tables to the 賓主 stage. Check that all eligible root positions and grades survive the handoff.

## Scope-only request

Ask: `原局月柱與大運，各自在原局內和原局對歲運的比較中屬主還是賓？`

Expected: 月柱 is 賓／家外 within the original chart and part of 主 when comparing the whole chart with luck. 大運 is 賓 in the latter scope and has no original-chart 家內／家外 position. No birth data or root lookup is needed for this classification.

## A. Main and borrowed roots coexist

- 年干甲: main 年支寅; borrowed 時支卯.
- 月干壬: main 月支亥; no borrowed roots.
- 日干癸: borrowed 月支亥; no main root, 全靠借根.
- 時干乙: main 時支卯; borrowed 年支寅.
- 月支亥 remains 家外 for 壬 and 癸, although its relative root role differs. Preserve both upstream shared-root pairs.

## B. Earth and fire use the same positions

- 年干戊 and 月干己 have only borrowed roots at 日支巳 and 時支午. Both are 全靠借根, and those branches stay 家內.
- 日干丙: main 日支巳; secondary 時支午.
- 時干丁: main 時支午; secondary 日支巳.
- Preserve grades and the two distinct shared-root pairs. Do not create an earth/fire shared-root pair.

## C. Tied main candidates and stronger borrowed roots

- 年干甲: main 年支辰; borrowed 時支未. The borrowed root has the higher grade.
- 月干壬: main 年支辰; secondary 月支丑.
- 日干戊 and 時干己: 日支戌 and 時支未 are tied main candidates, 取捨未定. No secondary roots. 年支辰 and 月支丑 remain borrowed.
- Do not select one candidate using self-position or 時支. Both earth stems still share all four roots.

## D. Repeated stems remain separate

- 年干乙 and 月干乙 each have main 月支卯, secondary 年支未, and borrowed 日支寅 plus 時支未.
- Their main and secondary roots are 家外; their borrowed roots are 家內. Preserve all four shared-root positions.
- 日干庚 and 時干癸 have all root-role fields 無 because the complete upstream inventory has no eligible roots.

## E. Original worked example

- 年干丙 has borrowed 時支巳 and 日支未 only, 全靠借根; both positions stay 家內.
- 日干乙 has main 日支未 and borrowed 年支寅 plus 月支寅. Both 寅 remain tied strongest roots without becoming main roots.
- 庚 and 辛 have all root-role fields 無. There is no shared-root pair.

## F. Separate chart and luck scopes

- 年干壬 and 月干癸: main 月支辰; no secondary or borrowed roots.
- 日干庚: main 時支酉; secondary 日支戌; borrowed 年支申, which is its strongest root.
- 時干辛: main 時支酉; secondary 日支戌; borrowed 年支申.
- 大運甲子 and 流年乙亥 are 賓 relative to the whole original chart. Do not assign them original chart positions or add 子亥 as original roots. Inferred 子 is also excluded.
- All four original pillars remain 主 relative to luck; 年月 remain 賓／家外 within the original chart. Preserve the two upstream shared-root pairs.
