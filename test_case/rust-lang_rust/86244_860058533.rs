bash
#!/usr/bin/env bash
exec >test.rs
echo 'fn main() {'
for i in $(seq 16384); do
    echo '&"foo";'
done
echo '}'
