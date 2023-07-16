sh
rg -A6 'unused borrow that must be used' | rg '[A-Z]+::ptr\(\)' | wc -l
1941
