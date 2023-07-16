
#!/usr/bin/env bash
out=$(cargo check 2>&1)
if echo "${out}" | grep -q "remove these paren"; then
    exit 1
fi
exit 0
