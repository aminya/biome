---
"@biomejs/biome": patch
---

Unused suppression fixes now remove only comments reported by `suppressions/unused`, preserve recognized suppressions for rules that are not enabled by default, and remove suppressions for rules explicitly disabled in the configuration.
