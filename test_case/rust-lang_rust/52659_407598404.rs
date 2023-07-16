
	LLVM ERROR: Cannot select: t10: ch = cleanupret t9
	In function: _ZN4core3str21_$LT$impl$u20$str$GT$4trim17h079b395f396c992aE
	error: Could not compile `core`.
	
	Caused by:
	  process didn't exit successfully: `D:\workspace\rust\build\bootstrap/debug/rustc --crate-name core libcore\lib.rs --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=ee59c66784deb9b7 -C extra-filename=-ee59c66784deb9b7 --out-dir D:\workspace\rust\build\x86_64-pc-windows-msvc\stage2-std\armv7-pc-windows-msvc\release\deps --target armv7-pc-windows-msvc -L dependency=D:\workspace\rust\build\x86_64-pc-windows-msvc\stage2-std\armv7-pc-windows-msvc\release\deps -L dependency=D:\workspace\rust\build\x86_64-pc-windows-msvc\stage2-std\release\deps` (exit code: 1)
	command did not execute successfully: "D:\\workspace\\rust\\build\\x86_64-pc-windows-msvc\\stage0/bin\\cargo.exe" "build" "--target" "armv7-pc-windows-msvc" "-j" "12" "-v" "--release" "--features" "panic-unwind backtrace" "--manifest-path" "D:\\workspace\\rust\\src/libstd/Cargo.toml" "--message-format" "json"
	expected success, got: exit code: 101
	thread 'main' panicked at 'cargo must succeed', bootstrap\compile.rs:1091:9
