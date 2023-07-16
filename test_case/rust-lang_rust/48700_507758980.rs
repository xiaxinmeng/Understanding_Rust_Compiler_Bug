
rustc --edition=2018 --crate-name grep_lite src/main.rs --color always --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=0456ed87980f8088 -C extra-filename=-0456ed87980f8088 --out-dir /home/kobenauf/code/grep-lite/target/debug/deps -C incremental=/home/kobenauf/code/grep-lite/target/debug/incremental -L dependency=/home/kobenauf/code/grep-lite/target/debug/deps --extern regex=/home/kobenauf/code/grep-lite/target/debug/deps/libregex-988e30d4a642ebea.rlib

warning: Error finalizing incremental compilation session directory `/home/kobenauf/code/grep-lite/target/debug/incremental/grep_lite-14tfocywywcnj/s-fdpofyvsd6-1dgd059-working`: Permission denied (os error 13)
