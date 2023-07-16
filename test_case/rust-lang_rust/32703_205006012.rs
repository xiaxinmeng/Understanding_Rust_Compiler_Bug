
DEBUG:cargo::ops::cargo_compile: compile; manifest-path=s:\Dev\Rust\Projects\test\special\Cargo.toml
DEBUG:cargo::ops::cargo_compile: loaded package; package=special v0.1.0 (file:///S:/Dev/Rust/Projects/test/special)
DEBUG:cargo::core::registry: load/missing  file:///S:/Dev/Rust/Projects/test/special
DEBUG:cargo::core::registry: load/locked   file:///S:/Dev/Rust/Projects/test/special
DEBUG:cargo::ops::cargo_rustc::fingerprint: fingerprint at: S:\Dev\Rust\Projects\test\special\target\debug\.fingerprint\special-2b6780032a516da7\bin-special
INFO:cargo::ops::cargo_rustc::fingerprint: fingerprint error for special v0.1.0 (file:///S:/Dev/Rust/Projects/test/special): failed to read `S:\Dev\Rust\Projects\test\special\target\debug\.fingerprint\special-2b6780032a516da7\bin-special`
INFO:cargo::ops::cargo_rustc::fingerprint:   cause: The system cannot find the file specified. (os error 2)
INFO:cargo::ops::cargo_rustc::job_queue: start: special v0.1.0 (file:///S:/Dev/Rust/Projects/test/special) => Target(bin: special)/Profile(build) => Host
   Compiling special v0.1.0 (file:///S:/Dev/Rust/Projects/test/special)
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\libstd-18402db3.rlib
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\std-18402db3.dll
INFO:rustc_metadata::loader: rlib reading metadata from: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\libstd-18402db3.rlib
INFO:rustc_metadata::loader: reading "libstd-18402db3.rlib" => Duration { secs: 0, nanos: 137346 }
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\libcore-18402db3.rlib
INFO:rustc_metadata::loader: rlib reading metadata from: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\libcore-18402db3.rlib
INFO:rustc_metadata::loader: reading "libcore-18402db3.rlib" => Duration { secs: 0, nanos: 110179 }
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\libcollections-18402db3.rlib
INFO:rustc_metadata::loader: rlib reading metadata from: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\libcollections-18402db3.rlib
INFO:rustc_metadata::loader: reading "libcollections-18402db3.rlib" => Duration { secs: 0, nanos: 120744 }
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\librustc_unicode-18402db3.rlib
INFO:rustc_metadata::loader: rlib reading metadata from: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\librustc_unicode-18402db3.rlib
INFO:rustc_metadata::loader: reading "librustc_unicode-18402db3.rlib" => Duration { secs: 0, nanos: 102330 }
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liballoc-18402db3.rlib
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liballoc_jemalloc-18402db3.rlib
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liballoc_system-18402db3.rlib
INFO:rustc_metadata::loader: rlib reading metadata from: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liballoc_system-18402db3.rlib
INFO:rustc_metadata::loader: reading "liballoc_system-18402db3.rlib" => Duration { secs: 0, nanos: 162702 }
INFO:rustc_metadata::loader: Rejecting via crate name
INFO:rustc_metadata::loader: metadata mismatch
INFO:rustc_metadata::loader: rlib reading metadata from: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liballoc_jemalloc-18402db3.rlib
INFO:rustc_metadata::loader: reading "liballoc_jemalloc-18402db3.rlib" => Duration { secs: 0, nanos: 153646 }
INFO:rustc_metadata::loader: Rejecting via crate name
INFO:rustc_metadata::loader: metadata mismatch
INFO:rustc_metadata::loader: rlib reading metadata from: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liballoc-18402db3.rlib
INFO:rustc_metadata::loader: reading "liballoc-18402db3.rlib" => Duration { secs: 0, nanos: 157873 }
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\librand-18402db3.rlib
INFO:rustc_metadata::loader: rlib reading metadata from: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\librand-18402db3.rlib
INFO:rustc_metadata::loader: reading "librand-18402db3.rlib" => Duration { secs: 0, nanos: 106254 }
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liblibc-18402db3.rlib
INFO:rustc_metadata::loader: rlib reading metadata from: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liblibc-18402db3.rlib
INFO:rustc_metadata::loader: reading "liblibc-18402db3.rlib" => Duration { secs: 0, nanos: 95991 }
INFO:rustc_metadata::loader: lib candidate: C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liballoc_system-18402db3.rlib
INFO:rustc_metadata::loader: rlib reading metadata from: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liballoc_system-18402db3.rlib
INFO:rustc_metadata::loader: reading "liballoc_system-18402db3.rlib" => Duration { secs: 0, nanos: 100519 }
INFO:rustc_metadata::creader: injecting a dep from 5 to 8
INFO:rustc_metadata::creader: injecting a dep from 5 to 8
INFO:rustc_metadata::creader: resolved crates:
INFO:rustc_metadata::creader:   name: alloc
INFO:rustc_metadata::creader:   cnum: 5
INFO:rustc_metadata::creader:   hash: 96d9fe4bfb8d60af
INFO:rustc_metadata::creader:   reqd: true
INFO:rustc_metadata::creader:    rlib: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liballoc-18402db3.rlib
INFO:rustc_metadata::creader:   name: libc
INFO:rustc_metadata::creader:   cnum: 7
INFO:rustc_metadata::creader:   hash: 7c6ee9d284ffe489
INFO:rustc_metadata::creader:   reqd: true
INFO:rustc_metadata::creader:    rlib: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liblibc-18402db3.rlib
INFO:rustc_metadata::creader:   name: std
INFO:rustc_metadata::creader:   cnum: 1
INFO:rustc_metadata::creader:   hash: 5cc09966f365d4b7
INFO:rustc_metadata::creader:   reqd: true
INFO:rustc_metadata::creader:   dylib: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\std-18402db3.dll
INFO:rustc_metadata::creader:    rlib: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\libstd-18402db3.rlib
INFO:rustc_metadata::creader:   name: collections
INFO:rustc_metadata::creader:   cnum: 3
INFO:rustc_metadata::creader:   hash: f9637d43ef019476
INFO:rustc_metadata::creader:   reqd: true
INFO:rustc_metadata::creader:    rlib: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\libcollections-18402db3.rlib
INFO:rustc_metadata::creader:   name: rustc_unicode
INFO:rustc_metadata::creader:   cnum: 4
INFO:rustc_metadata::creader:   hash: 11871ba6f66bd062
INFO:rustc_metadata::creader:   reqd: true
INFO:rustc_metadata::creader:    rlib: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\librustc_unicode-18402db3.rlib
INFO:rustc_metadata::creader:   name: rand
INFO:rustc_metadata::creader:   cnum: 6
INFO:rustc_metadata::creader:   hash: 5f5961029f1bd2c9
INFO:rustc_metadata::creader:   reqd: true
INFO:rustc_metadata::creader:    rlib: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\librand-18402db3.rlib
INFO:rustc_metadata::creader:   name: core
INFO:rustc_metadata::creader:   cnum: 2
INFO:rustc_metadata::creader:   hash: 452e08e7d0ce40b2
INFO:rustc_metadata::creader:   reqd: true
INFO:rustc_metadata::creader:    rlib: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\libcore-18402db3.rlib
INFO:rustc_metadata::creader:   name: alloc_system
INFO:rustc_metadata::creader:   cnum: 8
INFO:rustc_metadata::creader:   hash: f60c78d881366852
INFO:rustc_metadata::creader:   reqd: false
INFO:rustc_metadata::creader:    rlib: \\?\C:\Users\~~~~~\.multirust\toolchains\nightly\lib\rustlib\i686-pc-windows-gnu\lib\liballoc_system-18402db3.rlib
INFO:rustc::middle::region: CodeExtent(0) = DestructionScope(4294967295) [parent=1]
INFO:rustc::middle::region: CodeExtent(1) = Misc(4294967295) [parent=1]
INFO:rustc::middle::region: CodeExtent(2) = DestructionScope(2) [parent=0]
INFO:rustc::middle::region: CodeExtent(3) = DestructionScope(3) [parent=0]
INFO:rustc::middle::region: CodeExtent(4) = DestructionScope(4) [parent=0]
INFO:rustc::middle::region: CodeExtent(5) = DestructionScope(9) [parent=0]
INFO:rustc::middle::region: CodeExtent(6) = Misc(18) [parent=0]
INFO:rustc::middle::region: CodeExtent(7) = DestructionScope(15) [parent=0]
INFO:rustc::middle::region: CodeExtent(8) = DestructionScope(14) [parent=0]
INFO:rustc::middle::region: CodeExtent(9) = CallSiteScope { fn_id: 24, body_id: 30 } [parent=0]
INFO:rustc::middle::region: CodeExtent(10) = ParameterScope { fn_id: 24, body_id: 30 } [parent=9]
INFO:rustc::middle::region: CodeExtent(11) = Misc(27) [parent=0]
INFO:rustc::middle::region: CodeExtent(12) = DestructionScope(30) [parent=10]
INFO:rustc::middle::region: CodeExtent(13) = Misc(30) [parent=12]
INFO:rustc::middle::region: CodeExtent(14) = Misc(31) [parent=13]
INFO:rustc::middle::region: CodeExtent(15) = Misc(32) [parent=14]
INFO:rustc::middle::region: CodeExtent(16) = Misc(33) [parent=15]
INFO:rustc::middle::region: CodeExtent(17) = Misc(34) [parent=16]
INFO:rustc::middle::region: CodeExtent(18) = Misc(35) [parent=14]
INFO:rustc::middle::region: CodeExtent(19) = Misc(36) [parent=18]
INFO:rustc::middle::region: CodeExtent(20) = Misc(37) [parent=19]
INFO:rustc::middle::region: CodeExtent(21) = DestructionScope(24) [parent=0]
INFO:rustc::middle::region: CodeExtent(22) = DestructionScope(23) [parent=0]
INFO:rustc::middle::region: CodeExtent(23) = CallSiteScope { fn_id: 45, body_id: 51 } [parent=0]
INFO:rustc::middle::region: CodeExtent(24) = ParameterScope { fn_id: 45, body_id: 51 } [parent=23]
INFO:rustc::middle::region: CodeExtent(25) = Misc(48) [parent=0]
INFO:rustc::middle::region: CodeExtent(26) = DestructionScope(51) [parent=24]
INFO:rustc::middle::region: CodeExtent(27) = Misc(51) [parent=26]
INFO:rustc::middle::region: CodeExtent(28) = Misc(52) [parent=27]
INFO:rustc::middle::region: CodeExtent(29) = Misc(53) [parent=28]
INFO:rustc::middle::region: CodeExtent(30) = Misc(54) [parent=29]
INFO:rustc::middle::region: CodeExtent(31) = Misc(55) [parent=30]
INFO:rustc::middle::region: CodeExtent(32) = Misc(56) [parent=29]
INFO:rustc::middle::region: CodeExtent(33) = Misc(57) [parent=32]
INFO:rustc::middle::region: CodeExtent(34) = DestructionScope(45) [parent=0]
INFO:rustc::middle::region: CodeExtent(35) = DestructionScope(44) [parent=0]
INFO:rustc::middle::region: CodeExtent(36) = CallSiteScope { fn_id: 72, body_id: 74 } [parent=0]
INFO:rustc::middle::region: CodeExtent(37) = ParameterScope { fn_id: 72, body_id: 74 } [parent=36]
INFO:rustc::middle::region: CodeExtent(38) = DestructionScope(74) [parent=37]
INFO:rustc::middle::region: CodeExtent(39) = Misc(74) [parent=38]
INFO:rustc::middle::region: CodeExtent(40) = Remainder(BlockRemainder { block: 74, first_statement_index: 0 }) [parent=39]
INFO:rustc::middle::region: CodeExtent(41) = DestructionScope(75) [parent=40]
INFO:rustc::middle::region: CodeExtent(42) = Misc(75) [parent=41]
INFO:rustc::middle::region: CodeExtent(43) = Misc(77) [parent=42]
INFO:rustc::middle::region: CodeExtent(44) = Misc(78) [parent=42]
INFO:rustc::middle::region: CodeExtent(45) = Misc(79) [parent=44]
INFO:rustc::middle::region: CodeExtent(46) = Remainder(BlockRemainder { block: 74, first_statement_index: 1 }) [parent=40]
INFO:rustc::middle::region: CodeExtent(47) = DestructionScope(80) [parent=46]
INFO:rustc::middle::region: CodeExtent(48) = Misc(80) [parent=47]
INFO:rustc::middle::region: CodeExtent(49) = Misc(82) [parent=48]
INFO:rustc::middle::region: CodeExtent(50) = Misc(83) [parent=48]
INFO:rustc::middle::region: CodeExtent(51) = Misc(84) [parent=50]
INFO:rustc::middle::region: CodeExtent(52) = Remainder(BlockRemainder { block: 74, first_statement_index: 2 }) [parent=46]
INFO:rustc::middle::region: CodeExtent(53) = DestructionScope(85) [parent=52]
INFO:rustc::middle::region: CodeExtent(54) = Misc(85) [parent=53]
INFO:rustc::middle::region: CodeExtent(55) = Misc(87) [parent=54]
INFO:rustc::middle::region: CodeExtent(56) = Misc(88) [parent=54]
INFO:rustc::middle::region: CodeExtent(57) = Misc(89) [parent=56]
INFO:rustc::middle::region: CodeExtent(58) = Misc(90) [parent=57]
INFO:rustc::middle::region: CodeExtent(59) = Misc(91) [parent=57]
INFO:rustc::middle::region: CodeExtent(60) = Remainder(BlockRemainder { block: 74, first_statement_index: 3 }) [parent=52]
INFO:rustc::middle::region: CodeExtent(61) = DestructionScope(92) [parent=60]
INFO:rustc::middle::region: CodeExtent(62) = Misc(92) [parent=61]
INFO:rustc::middle::region: CodeExtent(63) = Misc(94) [parent=62]
INFO:rustc::middle::region: CodeExtent(64) = Misc(95) [parent=62]
INFO:rustc::middle::region: CodeExtent(65) = Misc(96) [parent=64]
INFO:rustc::middle::region: CodeExtent(66) = Misc(97) [parent=65]
INFO:rustc::middle::region: CodeExtent(67) = Misc(98) [parent=65]
INFO:rustc::middle::region: CodeExtent(68) = DestructionScope(99) [parent=60]
INFO:rustc::middle::region: CodeExtent(69) = Misc(99) [parent=68]
INFO:rustc::middle::region: CodeExtent(70) = Misc(100) [parent=69]
INFO:rustc::middle::region: CodeExtent(71) = Misc(102) [parent=70]
INFO:rustc::middle::region: CodeExtent(72) = Misc(104) [parent=71]
INFO:rustc::middle::region: CodeExtent(73) = Misc(105) [parent=72]
INFO:rustc::middle::region: CodeExtent(74) = Remainder(BlockRemainder { block: 105, first_statement_index: 0 }) [parent=73]
INFO:rustc::middle::region: CodeExtent(75) = DestructionScope(106) [parent=74]
INFO:rustc::middle::region: CodeExtent(76) = Misc(106) [parent=75]
INFO:rustc::middle::region: CodeExtent(77) = Misc(119) [parent=74]
INFO:rustc::middle::region: CodeExtent(78) = Misc(120) [parent=71]
INFO:rustc::middle::region: CodeExtent(79) = Misc(121) [parent=78]
INFO:rustc::middle::region: CodeExtent(80) = Misc(122) [parent=79]
INFO:rustc::middle::region: CodeExtent(81) = Misc(123) [parent=80]
INFO:rustc::middle::region: CodeExtent(82) = Misc(124) [parent=81]
INFO:rustc::middle::region: CodeExtent(83) = Misc(125) [parent=80]
INFO:rustc::middle::region: CodeExtent(84) = Misc(126) [parent=83]
INFO:rustc::middle::region: CodeExtent(85) = Misc(127) [parent=79]
INFO:rustc::middle::region: CodeExtent(86) = Misc(128) [parent=79]
INFO:rustc::middle::region: CodeExtent(87) = Misc(129) [parent=79]
INFO:rustc::middle::region: CodeExtent(88) = DestructionScope(130) [parent=79]
INFO:rustc::middle::region: CodeExtent(89) = Misc(130) [parent=88]
INFO:rustc::middle::region: CodeExtent(90) = Misc(131) [parent=89]
INFO:rustc::middle::region: CodeExtent(91) = Misc(133) [parent=90]
INFO:rustc::middle::region: CodeExtent(92) = Misc(134) [parent=90]
INFO:rustc::middle::region: CodeExtent(93) = Misc(132) [parent=90]
INFO:rustc::middle::region: CodeExtent(94) = Misc(135) [parent=89]
INFO:rustc::middle::region: CodeExtent(95) = Misc(137) [parent=94]
INFO:rustc::middle::region: CodeExtent(96) = Misc(138) [parent=94]
INFO:rustc::middle::region: CodeExtent(97) = Misc(136) [parent=94]
INFO:rustc::middle::region: CodeExtent(98) = Misc(103) [parent=71]
INFO:rustc::middle::region: CodeExtent(99) = Misc(101) [parent=70]
INFO:rustc::middle::region: CodeExtent(100) = DestructionScope(72) [parent=0]
INFO:rustc::middle::region: CodeExtent(101) = Misc(114) [parent=0]
INFO:rustc::middle::region: CodeExtent(102) = Misc(115) [parent=101]
INFO:rustc::middle::region: CodeExtent(103) = Misc(116) [parent=102]
INFO:rustc::middle::region: CodeExtent(104) = Misc(117) [parent=102]
INFO:rustc::middle::region: CodeExtent(105) = Misc(118) [parent=102]
INFO:rustc::middle::region: CodeExtent(106) = DestructionScope(107) [parent=0]
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Alice as std::marker::Sized>))` at depth 0 yielded Ok(Some)
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Bob as std::marker::Sized>))` at depth 0 yielded Ok(Some)
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<i32 as std::marker::Sized>))` at depth 0 yielded Ok(Some)
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<(Bob, Alice) as Thing<i32>>))` at depth 0 yielded Ok(Some)
INFO:rustc::traits::fulfill: check_duplicate_trait entry: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Bob as std::marker::Sized>))` at depth 1 yielded Ok(Some)
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Alice as std::marker::Sized>))` at depth 1 yielded Ok(Some)
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Alice as std::marker::Sized>))` at depth 0 yielded Ok(Some)
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<Bob as std::marker::Sized>))` at depth 0 yielded Ok(Some)
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<(Bob, Alice) as Thing<i32>>))` at depth 0 yielded Ok(Some)
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait entry: `Binder(TraitPredicate(<Bob as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait entry: `Binder(TraitPredicate(<Alice as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<Bob as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<Alice as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<Alice as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<Bob as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<Alice as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<Bob as std::marker::Sized>))`
INFO:rustc::traits::fulfill: check_duplicate_trait hit: `Binder(TraitPredicate(<i32 as std::marker::Sized>))`
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<(Bob, Alice) as Thing<i32>>))` at depth 0 yielded Ok(Some)
