# upstream

`programs/curve` was imported from the public reconstruction `lalalune/pumpfun-source` (mit), commit `2abb99ccb60766c3cd292e679dbac5314afb3f3b` (2026-10-30), path `programs/pump`, as it was: the one importing commit here is the byte-for-byte copy, and every later commit is one of our changes.

the public diff page is built from `git diff <import commit>..HEAD -- programs/curve`, plus the reconstruction's own scoring harness (`reconstruction/harness/score.py`) run on our identity build against the captured original.

what was left out of the import: `recovered/` and `link/` (the reconstruction's own scaffolding for matching bytes, not program source).
