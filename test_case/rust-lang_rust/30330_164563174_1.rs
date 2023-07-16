
rustc foo.rs && nm -a ./foo | awk '{print $3}' | sort | md5sum
