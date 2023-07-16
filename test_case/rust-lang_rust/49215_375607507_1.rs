sh
cat /proc/$(pgrep cargo)/environ | tr '\0' '\n' | grep RUSTFLAGS
