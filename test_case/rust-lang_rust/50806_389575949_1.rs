
// 1. Initial "test-update-test" run
x.py bless ui
// 2. Optionally: Fix `//~ ERROR` annotations
(... fix errors ...)
// 3. Second "test-update-test" run to make sure everything passes now, maybe go back to `2.` if not
x.py bless ui
