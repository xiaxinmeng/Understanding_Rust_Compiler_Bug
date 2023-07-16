
// 1. Initial run
x.py test ui
// 2. Update stderr/stdout
update-all-reference.sh
// 3. Was it enough or some `//~ ERROR` annotations need update as well?
x.py test ui
// 4. Optionally: Fix `//~ ERROR` annotations
(... fix errors ...)
// 5. Was it enough or stderr/stdout need updating again?
x.py test ui
// 6. Optionally: Update stderr/stdout
update-all-reference.sh
// 7. Make sure everything passes now, maybe go back to `4.` if not
x.py test ui
