plain
travis_time:end:048ab5c8:start=1560554370385953648,finish=1560554371226473299,duration=840519651
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:40] 
[01:06:40] running 9 tests
[01:06:40] iiiiiiiii
[01:06:40] 
[01:06:40]  finished in 0.157
[01:06:40] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:56] 
[01:06:56] running 122 tests
[01:07:21] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:07:26] .i.i......iii.i.....ii
[01:07:26] 
[01:07:26]  finished in 30.102
[01:07:26] travis_fold:end:test_debuginfo

---
[01:44:34] travis_fold:end:stage0-linkchecker

[01:44:34] travis_time:end:stage0-linkchecker:start=1560560653334738823,finish=1560560655669643267,duration=2334904444

[01:44:34] alloc/vec/struct.Vec.html:806: broken link fragment `#method.sort_unstable` pointing to `alloc/vec/struct.Vec.html`
[01:44:34] alloc/vec/struct.Vec.html:838: broken link fragment `#method.sort_unstable_by` pointing to `alloc/vec/struct.Vec.html`
[01:44:34] alloc/vec/struct.Vec.html:863: broken link fragment `#method.sort_unstable_by_key` pointing to `alloc/vec/struct.Vec.html`
[01:44:34] alloc/vec/struct.Vec.html:929: broken link fragment `#method.make_ascii_uppercase` pointing to `alloc/vec/struct.Vec.html`
[01:44:34] alloc/vec/struct.Vec.html:934: broken link fragment `#method.make_ascii_lowercase` pointing to `alloc/vec/struct.Vec.html`
[01:44:34] alloc/fmt/trait.Write.html:11: broken link - alloc/macro.write.html
[01:44:34] alloc/fmt/trait.Write.html:33: broken link - alloc/primitive.char.html
[01:44:34] alloc/fmt/trait.Write.html:34: broken link - alloc/primitive.char.html
[01:44:34] alloc/fmt/trait.Write.html:52: broken link - alloc/macro.write.html
[01:44:34] alloc/fmt/trait.Write.html:54: broken link - alloc/macro.write.html
[01:44:34] alloc/fmt/fn.write.html:15: broken link - alloc/macro.write.html
[01:44:34] alloc/fmt/trait.Binary.html:5: broken link - alloc/primitive.i8.html
[01:44:34] alloc/fmt/trait.Binary.html:5: broken link - alloc/primitive.i128.html
[01:44:34] alloc/fmt/trait.Binary.html:5: broken link - alloc/primitive.isize.html
[01:44:34] alloc/fmt/trait.Binary.html:10: broken link - alloc/primitive.i32.html
[01:44:34] alloc/fmt/struct.Formatter.html:367: broken link - alloc/macro.format_args.html
[01:44:34] alloc/fmt/struct.Arguments.html:5: broken link - alloc/macro.format_args.html
[01:44:34] alloc/fmt/struct.Arguments.html:8: broken link - alloc/macro.format_args.html
[01:44:34] alloc/str/struct.SplitTerminator.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.RSplitN.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.EncodeUtf16.html:1: broken link - alloc/primitive.u16.html
[01:44:34] alloc/str/struct.EncodeUtf16.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.EncodeUtf16.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.SplitAsciiWhitespace.html:3: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.SplitAsciiWhitespace.html:3: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.Lines.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.Lines.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.EscapeDefault.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/fn.from_utf8.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/fn.from_utf8.html:2: broken link - alloc/primitive.u8.html
[01:44:34] alloc/str/fn.from_utf8.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/str/fn.from_utf8.html:4: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/fn.from_utf8.html:14: broken link - alloc/primitive.slice.html
[01:44:34] alloc/str/struct.LinesAny.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.ParseBoolError.html:1: broken link - alloc/primitive.bool.html
[01:44:34] alloc/str/struct.RMatchIndices.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.Matches.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:21: broken link - alloc/primitive.char.html
[01:44:34] alloc/str/index.html:22: broken link - alloc/primitive.char.html
[01:44:34] alloc/str/index.html:23: broken link - alloc/primitive.u16.html
[01:44:34] alloc/str/index.html:24: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:25: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:26: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:28: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:29: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:30: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:31: broken link - alloc/primitive.bool.html
[01:44:34] alloc/str/index.html:32: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:33: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:34: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:35: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:36: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:37: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:40: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:41: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/index.html:44: broken link - alloc/primitive.u8.html
[01:44:34] alloc/str/struct.Split.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.RSplitTerminator.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.EscapeUnicode.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.RSplit.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.Bytes.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.Bytes.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.SplitWhitespace.html:3: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.SplitWhitespace.html:3: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.RMatches.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.EscapeDebug.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.SplitN.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.Utf8Error.html:1: broken link - alloc/primitive.u8.html
[01:44:34] alloc/str/struct.Utf8Error.html:64: broken link - alloc/char/constant.REPLACEMENT_CHARACTER.html
[01:44:34] alloc/str/fn.from_utf8_unchecked.html:7: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.CharIndices.html:1: broken link - alloc/primitive.char.html
[01:44:34] alloc/str/struct.CharIndices.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.CharIndices.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/trait.FromStr.html:6: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/trait.FromStr.html:6: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/trait.FromStr.html:6: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/trait.FromStr.html:45: broken link - alloc/result/enum.Result.html
[01:44:34] alloc/str/trait.FromStr.html:47: broken link - alloc/result/enum.Result.html
[01:44:34] alloc/str/trait.FromStr.html:49: broken link - alloc/primitive.i32.html
[01:44:34] alloc/str/struct.MatchIndices.html:1: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.Chars.html:1: broken link - alloc/primitive.char.html
[01:44:34] alloc/str/struct.Chars.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/str/struct.Chars.html:2: broken link - alloc/primitive.str.html
[01:44:34] alloc/slice/struct.RChunksMut.html:5: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RChunksMut.html:5: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/fn.from_raw_parts.html:11: broken link - alloc/ptr/struct.NonNull.html
[01:44:34] alloc/slice/fn.from_raw_parts.html:13: broken link - alloc/primitive.pointer.html
[01:44:34] alloc/slice/struct.RSplitN.html:4: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RSplitN.html:4: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.SplitNMut.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.SplitNMut.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RSplitNMut.html:4: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RSplitNMut.html:4: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.Windows.html:2: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.Windows.html:2: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RChunksExactMut.html:6: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RChunksExactMut.html:6: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.Iter.html:2: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.Iter.html:2: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.ChunksExactMut.html:6: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.ChunksExactMut.html:6: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.IterMut.html:2: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.IterMut.html:2: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.Split.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.Split.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.ChunksExact.html:6: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.ChunksExact.html:6: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.Chunks.html:5: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.Chunks.html:5: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RChunks.html:5: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RChunks.html:5: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RSplit.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RSplit.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.SplitMut.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.SplitMut.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RSplitMut.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RSplitMut.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.SplitN.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.SplitN.html:3: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.ChunksMut.html:5: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.ChunksMut.html:5: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RChunksExact.html:6: broken link - alloc/primitive.slice.html
[01:44:34] alloc/slice/struct.RChunksExact.html:6: broken link - alloc/primitive.slice.html
[01:44:34] alloc/borrow/trait.Borrow.html:11: broken link - alloc/primitive.str.html
[01:44:34] alloc/borrow/trait.Borrow.html:33: broken link - alloc/convert/trait.AsRef.html
[01:44:34] alloc/borrow/trait.Borrow.html:40: broken link - alloc/primitive.str.html
[01:44:39] core/primitive.slice.html:655: broken link fragment `#method.sort_by_key` pointing to `core/primitive.slice.html`
[01:44:39] core/primitive.slice.html:741: broken link fragment `#method.sort_by_cached_key` pointing to `core/primitive.slice.html`
[01:44:39] core/primitive.slice.html:1147: broken link fragment `#method.to_ascii_uppercase` pointing to `core/primitive.slice.html`
[01:44:39] core/primitive.slice.html:1152: broken link fragment `#method.to_ascii_lowercase` pointing to `core/primitive.slice.html`
[01:44:41] core/primitive.str.html:1095: broken link fragment `#method.to_ascii_uppercase` pointing to `core/primitive.str.html`
[01:44:41] core/primitive.str.html:1107: broken link fragment `#method.to_ascii_lowercase` pointing to `core/primitive.str.html`
[01:44:41] core/primitive.pointer.html:360: broken link - core/ptr/fn.copy.html
[01:44:41] core/primitive.pointer.html:361: broken link - core/ptr/fn.copy.html
[01:44:41] core/primitive.pointer.html:364: broken link - core/ptr/fn.copy_nonoverlapping.html
[01:44:41] core/primitive.pointer.html:365: broken link - core/ptr/fn.copy_nonoverlapping.html
[01:44:41] core/primitive.pointer.html:762: broken link - core/ptr/fn.copy.html
[01:44:41] core/primitive.pointer.html:763: broken link - core/ptr/fn.copy.html
[01:44:41] core/primitive.pointer.html:766: broken link - core/ptr/fn.copy_nonoverlapping.html
[01:44:41] core/primitive.pointer.html:767: broken link - core/ptr/fn.copy_nonoverlapping.html
[01:44:41] core/primitive.pointer.html:770: broken link - core/ptr/fn.copy.html
[01:44:41] core/primitive.pointer.html:771: broken link - core/ptr/fn.copy.html
[01:44:41] core/primitive.pointer.html:774: broken link - core/ptr/fn.copy_nonoverlapping.html
[01:44:41] core/primitive.pointer.html:775: broken link - core/ptr/fn.copy_nonoverlapping.html
[01:44:41] core/primitive.pointer.html:783: broken link - core/ptr/fn.write_bytes.html
[01:44:43] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:44:43] 
[01:44:43] 
[01:44:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:44:43] expected success, got: exit code: 101
---
travis_time:end:023c61e7:start=1560560667144021019,finish=1560560667149363423,duration=5342404
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014226b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:287c7b67
travis_time:start:287c7b67
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03359730
$ dmesg | grep -i kill
