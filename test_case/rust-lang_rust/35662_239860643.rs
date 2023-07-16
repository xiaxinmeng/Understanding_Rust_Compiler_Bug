
rustc -Ctarget-cpu=native -Zorbit=on -C opt-level=3 --test mini.rs
rustc -Ctarget-cpu=native -Zorbit=off -C opt-level=3 --test mini.rs
