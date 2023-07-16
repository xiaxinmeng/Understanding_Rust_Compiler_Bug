sh
> rg -A6 'unused borrow that must be used' | rg --case-sensitive '\.\.\w*\];' | wc -l
60
