sh
rg --no-filename '^[^/].*\.rs:$' target/debug/deps/*.d | sed 's/:$//' | sort -u | diff - <(fd '\.rs$' | sort -u)
