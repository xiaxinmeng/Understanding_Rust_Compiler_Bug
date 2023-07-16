bash
#!/usr/bin/env bash
# Run Example 1
cargo clean
cargo fix 2>&1 | grep "Fixed"
if [[ $? -gt 0 ]]; then
    exit 0
else
    exit 1
fi
