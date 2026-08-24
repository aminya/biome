---
"@biomejs/biome": patch
---

Unused suppression fixes now remove only comments reported by `suppressions/unused` and preserve recognized suppressions for rules that are not enabled in the current analysis run.
