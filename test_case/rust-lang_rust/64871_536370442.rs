plain
2019-09-29T23:06:00.4190857Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-29T23:06:00.4413802Z ##[command]git config gc.auto 0
2019-09-29T23:06:00.4511187Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-29T23:06:00.4572006Z ##[command]git config --get-all http.proxy
2019-09-29T23:06:00.4738079Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64871/merge:refs/remotes/pull/64871/merge
---
2019-09-30T00:56:59.3327312Z    Compiling mdbook-linkcheck v0.3.0
2019-09-30T00:57:12.8218552Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-09-30T00:57:17.3893027Z     Finished release [optimized] target(s) in 8m 35s
2019-09-30T00:59:23.9164085Z Error: there are broken links
2019-09-30T00:59:23.9166215Z  Caused By: There was an error while fetching "https://ci.appveyor.com/project/rust-lang/rust", https://ci.appveyor.com/project/rust-lang/rust: timed out
2019-09-30T00:59:23.9167108Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-09-30T00:59:23.9168300Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-09-30T00:59:23.9168830Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-09-30T00:59:23.9169790Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-09-30T00:59:23.9170844Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_engine/forest/struct.Forest.html" returned 404 Not Found
---
2019-09-30T01:17:52.1758471Z normalized stderr:
2019-09-30T01:17:52.1758499Z 
2019-09-30T01:17:52.1758524Z 
2019-09-30T01:17:52.1758581Z expected stderr:
2019-09-30T01:17:52.1758625Z error: the variable `_index` is used as a loop counter.
2019-09-30T01:17:52.1758899Z    |
2019-09-30T01:17:52.1758899Z    |
2019-09-30T01:17:52.1758956Z LL |     for _v in &vec {
2019-09-30T01:17:52.1759041Z    |               ^^^^ help: consider using: `for (_index, _v) in (&vec).enumerate()`
2019-09-30T01:17:52.1759332Z    = note: `-D clippy::explicit-counter-loop` implied by `-D warnings`
2019-09-30T01:17:52.1759366Z 
2019-09-30T01:17:52.1759366Z 
2019-09-30T01:17:52.1759420Z error: the variable `_index` is used as a loop counter.
2019-09-30T01:17:52.1759681Z    |
2019-09-30T01:17:52.1759681Z    |
2019-09-30T01:17:52.1759721Z LL |     for _v in &vec {
2019-09-30T01:17:52.1759767Z    |               ^^^^ help: consider using: `for (_index, _v) in (&vec).enumerate()`
2019-09-30T01:17:52.1759812Z 
2019-09-30T01:17:52.1759852Z error: the variable `count` is used as a loop counter.
2019-09-30T01:17:52.1760111Z    |
2019-09-30T01:17:52.1760111Z    |
2019-09-30T01:17:52.1760152Z LL |         for ch in text.chars() {
2019-09-30T01:17:52.1760201Z    |                   ^^^^^^^^^^^^ help: consider using: `for (count, ch) in text.chars().enumerate()`
2019-09-30T01:17:52.1760245Z 
2019-09-30T01:17:52.1760307Z error: the variable `count` is used as a loop counter.
2019-09-30T01:17:52.1760555Z    |
2019-09-30T01:17:52.1760555Z    |
2019-09-30T01:17:52.1760619Z LL |         for ch in text.chars() {
2019-09-30T01:17:52.1760669Z    |                   ^^^^^^^^^^^^ help: consider using: `for (count, ch) in text.chars().enumerate()`
2019-09-30T01:17:52.1760700Z 
2019-09-30T01:17:52.1760741Z error: the variable `count` is used as a loop counter.
2019-09-30T01:17:52.1761000Z    |
2019-09-30T01:17:52.1761038Z LL |         for _i in 3..10 {
2019-09-30T01:17:52.1761038Z LL |         for _i in 3..10 {
2019-09-30T01:17:52.1761101Z    |                   ^^^^^ help: consider using: `for (count, _i) in (3..10).enumerate()`
2019-09-30T01:17:52.1761172Z error: aborting due to 5 previous errors
2019-09-30T01:17:52.1761198Z 
2019-09-30T01:17:52.1761248Z 
2019-09-30T01:17:52.1761271Z 
2019-09-30T01:17:52.1761271Z 
2019-09-30T01:17:52.1761308Z diff of stderr:
2019-09-30T01:17:52.1761333Z 
2019-09-30T01:17:52.1761560Z -error: the variable `_index` is used as a loop counter.
2019-09-30T01:17:52.1761925Z -   |
2019-09-30T01:17:52.1761925Z -   |
2019-09-30T01:17:52.1762931Z -LL |     for _v in &vec {
2019-09-30T01:17:52.1763429Z -   |               ^^^^ help: consider using: `for (_index, _v) in (&vec).enumerate()`
2019-09-30T01:17:52.1763847Z -   = note: `-D clippy::explicit-counter-loop` implied by `-D warnings`
2019-09-30T01:17:52.1764040Z -
2019-09-30T01:17:52.1764040Z -
2019-09-30T01:17:52.1764259Z -error: the variable `_index` is used as a loop counter.
2019-09-30T01:17:52.1765028Z -   |
2019-09-30T01:17:52.1765028Z -   |
2019-09-30T01:17:52.1765225Z -LL |     for _v in &vec {
2019-09-30T01:17:52.1765474Z -   |               ^^^^ help: consider using: `for (_index, _v) in (&vec).enumerate()`
2019-09-30T01:17:52.1765998Z -
2019-09-30T01:17:52.1766226Z -error: the variable `count` is used as a loop counter.
2019-09-30T01:17:52.1766585Z -   |
2019-09-30T01:17:52.1766585Z -   |
2019-09-30T01:17:52.1766796Z -LL |         for ch in text.chars() {
2019-09-30T01:17:52.1767060Z -   |                   ^^^^^^^^^^^^ help: consider using: `for (count, ch) in text.chars().enumerate()`
2019-09-30T01:17:52.1767225Z -
2019-09-30T01:17:52.1767446Z -error: the variable `count` is used as a loop counter.
2019-09-30T01:17:52.1767866Z -   |
2019-09-30T01:17:52.1767866Z -   |
2019-09-30T01:17:52.1768091Z -LL |         for ch in text.chars() {
2019-09-30T01:17:52.1768360Z -   |                   ^^^^^^^^^^^^ help: consider using: `for (count, ch) in text.chars().enumerate()`
2019-09-30T01:17:52.1768540Z -
2019-09-30T01:17:52.1768776Z -error: the variable `count` is used as a loop counter.
2019-09-30T01:17:52.1769187Z -   |
2019-09-30T01:17:52.1769391Z -LL |         for _i in 3..10 {
2019-09-30T01:17:52.1769391Z -LL |         for _i in 3..10 {
2019-09-30T01:17:52.1769668Z -   |                   ^^^^^ help: consider using: `for (count, _i) in (3..10).enumerate()`
2019-09-30T01:17:52.1770062Z -error: aborting due to 5 previous errors
2019-09-30T01:17:52.1770256Z -
2019-09-30T01:17:52.1770419Z -
2019-09-30T01:17:52.1770447Z 
2019-09-30T01:17:52.1770447Z 
2019-09-30T01:17:52.1770611Z The actual stderr differed from the expected stderr.
2019-09-30T01:17:52.1771109Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/explicit_counter_loop.stderr
2019-09-30T01:17:52.1771177Z To update references, run this command from build directory:
2019-09-30T01:17:52.1771571Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'explicit_counter_loop.rs'
2019-09-30T01:17:52.1771625Z 
2019-09-30T01:17:52.1771670Z error: 1 errors occurred comparing output.
2019-09-30T01:17:52.1771714Z status: exit code: 0
2019-09-30T01:17:52.1773685Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/explicit_counter_loop.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/explicit_counter_loop.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/explicit_counter_loop.stage-id.aux" "-A" "unused"
2019-09-30T01:17:52.1774205Z ------------------------------------------
2019-09-30T01:17:52.1774240Z 
2019-09-30T01:17:52.1774456Z ------------------------------------------
2019-09-30T01:17:52.1774501Z stderr:
---
2019-09-30T01:17:52.1775382Z normalized stderr:
2019-09-30T01:17:52.1775410Z 
2019-09-30T01:17:52.1775435Z 
2019-09-30T01:17:52.1775495Z expected stderr:
2019-09-30T01:17:52.1776292Z error: you seem to want to iterate on a map's values
2019-09-30T01:17:52.1776591Z    |
2019-09-30T01:17:52.1776591Z    |
2019-09-30T01:17:52.1776645Z LL |     for (_, v) in &m {
2019-09-30T01:17:52.1776739Z    |
2019-09-30T01:17:52.1776739Z    |
2019-09-30T01:17:52.1776955Z    = note: `-D clippy::for-kv-map` implied by `-D warnings`
2019-09-30T01:17:52.1777064Z    |
2019-09-30T01:17:52.1777064Z    |
2019-09-30T01:17:52.1777103Z LL |     for v in m.values() {
2019-09-30T01:17:52.1777171Z 
2019-09-30T01:17:52.1777171Z 
2019-09-30T01:17:52.1777396Z error: you seem to want to iterate on a map's values
2019-09-30T01:17:52.1777628Z    |
2019-09-30T01:17:52.1777628Z    |
2019-09-30T01:17:52.1777666Z LL |     for (_, v) in &*m {
2019-09-30T01:17:52.1777773Z help: use the corresponding method
2019-09-30T01:17:52.1777812Z    |
2019-09-30T01:17:52.1777812Z    |
2019-09-30T01:17:52.1778051Z LL |     for v in (*m).values() {
2019-09-30T01:17:52.1778163Z 
2019-09-30T01:17:52.1778163Z 
2019-09-30T01:17:52.1778463Z error: you seem to want to iterate on a map's values
2019-09-30T01:17:52.1778732Z    |
2019-09-30T01:17:52.1778732Z    |
2019-09-30T01:17:52.1778770Z LL |     for (_, v) in &mut m {
2019-09-30T01:17:52.1778869Z help: use the corresponding method
2019-09-30T01:17:52.1778906Z    |
2019-09-30T01:17:52.1778906Z    |
2019-09-30T01:17:52.1778961Z LL |     for v in m.values_mut() {
2019-09-30T01:17:52.1779029Z 
2019-09-30T01:17:52.1779029Z 
2019-09-30T01:17:52.1779234Z error: you seem to want to iterate on a map's values
2019-09-30T01:17:52.1779489Z    |
2019-09-30T01:17:52.1779489Z    |
2019-09-30T01:17:52.1779527Z LL |     for (_, v) in &mut *m {
2019-09-30T01:17:52.1779624Z help: use the corresponding method
2019-09-30T01:17:52.1779663Z    |
2019-09-30T01:17:52.1779663Z    |
2019-09-30T01:17:52.1779702Z LL |     for v in (*m).values_mut() {
2019-09-30T01:17:52.1779793Z 
2019-09-30T01:17:52.1779793Z 
2019-09-30T01:17:52.1779999Z error: you seem to want to iterate on a map's keys
2019-09-30T01:17:52.1780245Z    |
2019-09-30T01:17:52.1780245Z    |
2019-09-30T01:17:52.1780283Z LL |     for (k, _value) in rm {
2019-09-30T01:17:52.1780381Z help: use the corresponding method
2019-09-30T01:17:52.1780418Z    |
2019-09-30T01:17:52.1780418Z    |
2019-09-30T01:17:52.1780494Z LL |     for k in rm.keys() {
2019-09-30T01:17:52.1780577Z 
2019-09-30T01:17:52.1780617Z error: aborting due to 5 previous errors
2019-09-30T01:17:52.1780651Z 
2019-09-30T01:17:52.1780675Z 
2019-09-30T01:17:52.1780675Z 
2019-09-30T01:17:52.1780714Z 
2019-09-30T01:17:52.1780751Z diff of stderr:
2019-09-30T01:17:52.1780777Z 
2019-09-30T01:17:52.1780995Z -error: you seem to want to iterate on a map's values
2019-09-30T01:17:52.1781493Z -   |
2019-09-30T01:17:52.1781493Z -   |
2019-09-30T01:17:52.1781720Z -LL |     for (_, v) in &m {
2019-09-30T01:17:52.1782086Z -   |
2019-09-30T01:17:52.1782086Z -   |
2019-09-30T01:17:52.1782296Z -   = note: `-D clippy::for-kv-map` implied by `-D warnings`
2019-09-30T01:17:52.1783088Z -   |
2019-09-30T01:17:52.1783088Z -   |
2019-09-30T01:17:52.1783286Z -LL |     for v in m.values() {
2019-09-30T01:17:52.1783672Z -
2019-09-30T01:17:52.1783672Z -
2019-09-30T01:17:52.1783888Z -error: you seem to want to iterate on a map's values
2019-09-30T01:17:52.1784286Z -   |
2019-09-30T01:17:52.1784286Z -   |
2019-09-30T01:17:52.1784483Z -LL |     for (_, v) in &*m {
2019-09-30T01:17:52.1785050Z -help: use the corresponding method
2019-09-30T01:17:52.1785267Z -   |
2019-09-30T01:17:52.1785267Z -   |
2019-09-30T01:17:52.1785487Z -LL |     for v in (*m).values() {
2019-09-30T01:17:52.1786037Z -
2019-09-30T01:17:52.1786037Z -
2019-09-30T01:17:52.1786243Z -error: you seem to want to iterate on a map's values
2019-09-30T01:17:52.1786616Z -   |
2019-09-30T01:17:52.1786616Z -   |
2019-09-30T01:17:52.1786802Z -LL |     for (_, v) in &mut m {
2019-09-30T01:17:52.1787175Z -help: use the corresponding method
2019-09-30T01:17:52.1787357Z -   |
2019-09-30T01:17:52.1787357Z -   |
2019-09-30T01:17:52.1787544Z -LL |     for v in m.values_mut() {
2019-09-30T01:17:52.1787908Z -
2019-09-30T01:17:52.1787908Z -
2019-09-30T01:17:52.1788109Z -error: you seem to want to iterate on a map's values
2019-09-30T01:17:52.1788485Z -   |
2019-09-30T01:17:52.1788485Z -   |
2019-09-30T01:17:52.1788671Z -LL |     for (_, v) in &mut *m {
2019-09-30T01:17:52.1789171Z -help: use the corresponding method
2019-09-30T01:17:52.1792873Z -   |
2019-09-30T01:17:52.1792873Z -   |
2019-09-30T01:17:52.1793158Z -LL |     for v in (*m).values_mut() {
2019-09-30T01:17:52.1793559Z -
2019-09-30T01:17:52.1793559Z -
2019-09-30T01:17:52.1793778Z -error: you seem to want to iterate on a map's keys
2019-09-30T01:17:52.1794174Z -   |
2019-09-30T01:17:52.1794174Z -   |
2019-09-30T01:17:52.1794375Z -LL |     for (k, _value) in rm {
2019-09-30T01:17:52.1794774Z -help: use the corresponding method
2019-09-30T01:17:52.1794984Z -   |
2019-09-30T01:17:52.1794984Z -   |
2019-09-30T01:17:52.1795199Z -LL |     for k in rm.keys() {
2019-09-30T01:17:52.1795620Z -
2019-09-30T01:17:52.1795850Z -error: aborting due to 5 previous errors
2019-09-30T01:17:52.1796158Z -
2019-09-30T01:17:52.1796314Z -
---
2019-09-30T01:17:52.1797176Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'for_kv_map.rs'
2019-09-30T01:17:52.1797218Z 
2019-09-30T01:17:52.1797277Z error: 1 errors occurred comparing output.
2019-09-30T01:17:52.1797319Z status: exit code: 0
2019-09-30T01:17:52.1798803Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/for_kv_map.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/for_kv_map.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/for_kv_map.stage-id.aux" "-A" "unused"
2019-09-30T01:17:52.1799297Z ------------------------------------------
2019-09-30T01:17:52.1799329Z 
2019-09-30T01:17:52.1799556Z ------------------------------------------
2019-09-30T01:17:52.1799598Z stderr:
---
2019-09-30T01:17:52.1826473Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'for_loop_over_option_result.rs'
2019-09-30T01:17:52.1826552Z 
2019-09-30T01:17:52.1826595Z error: 1 errors occurred comparing output.
2019-09-30T01:17:52.1826635Z status: exit code: 1
2019-09-30T01:17:52.1828025Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/for_loop_over_option_result.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/for_loop_over_option_result.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/for_loop_over_option_result.stage-id.aux" "-A" "unused"
2019-09-30T01:17:52.1828427Z ------------------------------------------
2019-09-30T01:17:52.1828474Z 
2019-09-30T01:17:52.1828672Z ------------------------------------------
2019-09-30T01:17:52.1828713Z stderr:
---
2019-09-30T01:17:52.1832966Z normalized stderr:
2019-09-30T01:17:52.1832994Z 
2019-09-30T01:17:52.1833020Z 
2019-09-30T01:17:52.1833079Z expected stderr:
2019-09-30T01:17:52.1833125Z error: this range is empty so this for loop will never run
2019-09-30T01:17:52.1833459Z    |
2019-09-30T01:17:52.1833500Z LL |     for i in 10..0 {
2019-09-30T01:17:52.1833543Z    |              ^^^^^
2019-09-30T01:17:52.1833597Z    |
2019-09-30T01:17:52.1833597Z    |
2019-09-30T01:17:52.1833833Z    = note: `-D clippy::reverse-range-loop` implied by `-D warnings`
2019-09-30T01:17:52.1833899Z help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1833963Z    |
2019-09-30T01:17:52.1834005Z LL |     for i in (0..10).rev() {
2019-09-30T01:17:52.1834079Z 
2019-09-30T01:17:52.1834079Z 
2019-09-30T01:17:52.1834140Z error: this range is empty so this for loop will never run
2019-09-30T01:17:52.1834700Z    |
2019-09-30T01:17:52.1834761Z LL |     for i in 10..=0 {
2019-09-30T01:17:52.1834803Z    |              ^^^^^^
2019-09-30T01:17:52.1834853Z help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1834853Z help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1837570Z    |
2019-09-30T01:17:52.1837657Z LL |     for i in (0..=10).rev() {
2019-09-30T01:17:52.1837729Z 
2019-09-30T01:17:52.1837729Z 
2019-09-30T01:17:52.1837789Z error: this range is empty so this for loop will never run
2019-09-30T01:17:52.1838204Z    |
2019-09-30T01:17:52.1838244Z LL |     for i in MAX_LEN..0 {
2019-09-30T01:17:52.1838420Z    |              ^^^^^^^^^^
2019-09-30T01:17:52.1838471Z help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1838471Z help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1838516Z    |
2019-09-30T01:17:52.1838577Z LL |     for i in (0..MAX_LEN).rev() {
2019-09-30T01:17:52.1838650Z 
2019-09-30T01:17:52.1838650Z 
2019-09-30T01:17:52.1838694Z error: this range is empty so this for loop will never run
2019-09-30T01:17:52.1838982Z    |
2019-09-30T01:17:52.1839032Z LL |     for i in 10..5 + 4 {
2019-09-30T01:17:52.1839093Z    |              ^^^^^^^^^
2019-09-30T01:17:52.1839145Z help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1839145Z help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1839190Z    |
2019-09-30T01:17:52.1839399Z LL |     for i in (5 + 4..10).rev() {
2019-09-30T01:17:52.1839482Z 
2019-09-30T01:17:52.1839482Z 
2019-09-30T01:17:52.1839525Z error: this range is empty so this for loop will never run
2019-09-30T01:17:52.1839836Z    |
2019-09-30T01:17:52.1839836Z    |
2019-09-30T01:17:52.1840040Z LL |     for i in (5 + 2)..(3 - 1) {
2019-09-30T01:17:52.1840157Z help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1840203Z    |
2019-09-30T01:17:52.1840203Z    |
2019-09-30T01:17:52.1840435Z LL |     for i in ((3 - 1)..(5 + 2)).rev() {
2019-09-30T01:17:52.1840617Z 
2019-09-30T01:17:52.1840668Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1840927Z   --> $DIR/for_loop_fixable.rs:98:15
2019-09-30T01:17:52.1840970Z    |
2019-09-30T01:17:52.1840970Z    |
2019-09-30T01:17:52.1841021Z LL |     for _v in vec.iter() {}
2019-09-30T01:17:52.1841089Z    |               ^^^^^^^^^^ help: to write this more concisely, try: `&vec`
2019-09-30T01:17:52.1841370Z    = note: `-D clippy::explicit-iter-loop` implied by `-D warnings`
2019-09-30T01:17:52.1841402Z 
2019-09-30T01:17:52.1841470Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1841685Z   --> $DIR/for_loop_fixable.rs:100:15
2019-09-30T01:17:52.1841685Z   --> $DIR/for_loop_fixable.rs:100:15
2019-09-30T01:17:52.1841729Z    |
2019-09-30T01:17:52.1841788Z LL |     for _v in vec.iter_mut() {}
2019-09-30T01:17:52.1841839Z    |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&mut vec`
2019-09-30T01:17:52.1842036Z error: it is more concise to loop over containers instead of using explicit iteration methods`
2019-09-30T01:17:52.1842262Z   --> $DIR/for_loop_fixable.rs:103:15
2019-09-30T01:17:52.1842304Z    |
2019-09-30T01:17:52.1842304Z    |
2019-09-30T01:17:52.1842352Z LL |     for _v in out_vec.into_iter() {}
2019-09-30T01:17:52.1842422Z    |               ^^^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `out_vec`
2019-09-30T01:17:52.1843216Z    = note: `-D clippy::explicit-into-iter-loop` implied by `-D warnings`
2019-09-30T01:17:52.1843269Z 
2019-09-30T01:17:52.1843321Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1843538Z   --> $DIR/for_loop_fixable.rs:106:15
2019-09-30T01:17:52.1843538Z   --> $DIR/for_loop_fixable.rs:106:15
2019-09-30T01:17:52.1843598Z    |
2019-09-30T01:17:52.1843640Z LL |     for _v in array.into_iter() {}
2019-09-30T01:17:52.1843693Z    |               ^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&array`
2019-09-30T01:17:52.1843803Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1844019Z   --> $DIR/for_loop_fixable.rs:111:15
2019-09-30T01:17:52.1844062Z    |
2019-09-30T01:17:52.1844062Z    |
2019-09-30T01:17:52.1844127Z LL |     for _v in [1, 2, 3].iter() {}
2019-09-30T01:17:52.1844180Z    |               ^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[1, 2, 3]`
2019-09-30T01:17:52.1844262Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1844495Z   --> $DIR/for_loop_fixable.rs:115:15
2019-09-30T01:17:52.1844539Z    |
2019-09-30T01:17:52.1844539Z    |
2019-09-30T01:17:52.1844580Z LL |     for _v in [0; 32].iter() {}
2019-09-30T01:17:52.1844649Z    |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[0; 32]`
2019-09-30T01:17:52.1844730Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1844971Z   --> $DIR/for_loop_fixable.rs:120:15
2019-09-30T01:17:52.1845016Z    |
2019-09-30T01:17:52.1845016Z    |
2019-09-30T01:17:52.1845057Z LL |     for _v in ll.iter() {}
2019-09-30T01:17:52.1845240Z    |               ^^^^^^^^^ help: to write this more concisely, try: `&ll`
2019-09-30T01:17:52.1845357Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1845639Z   --> $DIR/for_loop_fixable.rs:123:15
2019-09-30T01:17:52.1845703Z    |
2019-09-30T01:17:52.1845703Z    |
2019-09-30T01:17:52.1845747Z LL |     for _v in vd.iter() {}
2019-09-30T01:17:52.1845799Z    |               ^^^^^^^^^ help: to write this more concisely, try: `&vd`
2019-09-30T01:17:52.1845901Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1846221Z   --> $DIR/for_loop_fixable.rs:126:15
2019-09-30T01:17:52.1846354Z    |
2019-09-30T01:17:52.1846354Z    |
2019-09-30T01:17:52.1846412Z LL |     for _v in bh.iter() {}
2019-09-30T01:17:52.1846460Z    |               ^^^^^^^^^ help: to write this more concisely, try: `&bh`
2019-09-30T01:17:52.1846544Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1846798Z   --> $DIR/for_loop_fixable.rs:129:15
2019-09-30T01:17:52.1846840Z    |
2019-09-30T01:17:52.1846840Z    |
2019-09-30T01:17:52.1846879Z LL |     for _v in hm.iter() {}
2019-09-30T01:17:52.1846943Z    |               ^^^^^^^^^ help: to write this more concisely, try: `&hm`
2019-09-30T01:17:52.1847025Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1847271Z   --> $DIR/for_loop_fixable.rs:132:15
2019-09-30T01:17:52.1847316Z    |
2019-09-30T01:17:52.1847316Z    |
2019-09-30T01:17:52.1847359Z LL |     for _v in bt.iter() {}
2019-09-30T01:17:52.1847408Z    |               ^^^^^^^^^ help: to write this more concisely, try: `&bt`
2019-09-30T01:17:52.1847516Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1847744Z   --> $DIR/for_loop_fixable.rs:135:15
2019-09-30T01:17:52.1847805Z    |
2019-09-30T01:17:52.1847805Z    |
2019-09-30T01:17:52.1847854Z LL |     for _v in hs.iter() {}
2019-09-30T01:17:52.1847903Z    |               ^^^^^^^^^ help: to write this more concisely, try: `&hs`
2019-09-30T01:17:52.1848001Z error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1848229Z   --> $DIR/for_loop_fixable.rs:138:15
2019-09-30T01:17:52.1848273Z    |
2019-09-30T01:17:52.1848273Z    |
2019-09-30T01:17:52.1848332Z LL |     for _v in bs.iter() {}
2019-09-30T01:17:52.1848382Z    |               ^^^^^^^^^ help: to write this more concisely, try: `&bs`
2019-09-30T01:17:52.1848457Z error: aborting due to 18 previous errors
2019-09-30T01:17:52.1848509Z 
2019-09-30T01:17:52.1848535Z 
2019-09-30T01:17:52.1848560Z 
2019-09-30T01:17:52.1848560Z 
2019-09-30T01:17:52.1848601Z diff of stderr:
2019-09-30T01:17:52.1848645Z 
2019-09-30T01:17:52.1848887Z -error: this range is empty so this for loop will never run
2019-09-30T01:17:52.1853470Z -   |
2019-09-30T01:17:52.1853711Z -LL |     for i in 10..0 {
2019-09-30T01:17:52.1853903Z -   |              ^^^^^
2019-09-30T01:17:52.1854381Z -   |
2019-09-30T01:17:52.1854381Z -   |
2019-09-30T01:17:52.1854688Z -   = note: `-D clippy::reverse-range-loop` implied by `-D warnings`
2019-09-30T01:17:52.1854957Z -help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1855136Z -   |
2019-09-30T01:17:52.1855358Z -LL |     for i in (0..10).rev() {
2019-09-30T01:17:52.1855733Z -
2019-09-30T01:17:52.1855733Z -
2019-09-30T01:17:52.1855955Z -error: this range is empty so this for loop will never run
2019-09-30T01:17:52.1856476Z -   |
2019-09-30T01:17:52.1856664Z -LL |     for i in 10..=0 {
2019-09-30T01:17:52.1856871Z -   |              ^^^^^^
2019-09-30T01:17:52.1857120Z -help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1857120Z -help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1857467Z -   |
2019-09-30T01:17:52.1857739Z -LL |     for i in (0..=10).rev() {
2019-09-30T01:17:52.1858102Z -
2019-09-30T01:17:52.1858102Z -
2019-09-30T01:17:52.1858317Z -error: this range is empty so this for loop will never run
2019-09-30T01:17:52.1858710Z -   |
2019-09-30T01:17:52.1858903Z -LL |     for i in MAX_LEN..0 {
2019-09-30T01:17:52.1859117Z -   |              ^^^^^^^^^^
2019-09-30T01:17:52.1859369Z -help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1859369Z -help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1859543Z -   |
2019-09-30T01:17:52.1859760Z -LL |     for i in (0..MAX_LEN).rev() {
2019-09-30T01:17:52.1860256Z -
2019-09-30T01:17:52.1860256Z -
2019-09-30T01:17:52.1860492Z -error: this range is empty so this for loop will never run
2019-09-30T01:17:52.1860867Z -   |
2019-09-30T01:17:52.1861069Z -LL |     for i in 10..5 + 4 {
2019-09-30T01:17:52.1861282Z -   |              ^^^^^^^^^
2019-09-30T01:17:52.1861530Z -help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1861530Z -help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1861702Z -   |
2019-09-30T01:17:52.1861923Z -LL |     for i in (5 + 4..10).rev() {
2019-09-30T01:17:52.1862284Z -
2019-09-30T01:17:52.1862284Z -
2019-09-30T01:17:52.1862910Z -error: this range is empty so this for loop will never run
2019-09-30T01:17:52.1863332Z -   |
2019-09-30T01:17:52.1863332Z -   |
2019-09-30T01:17:52.1863539Z -LL |     for i in (5 + 2)..(3 - 1) {
2019-09-30T01:17:52.1864045Z -help: consider using the following if you are attempting to iterate over this range in reverse
2019-09-30T01:17:52.1864223Z -   |
2019-09-30T01:17:52.1864223Z -   |
2019-09-30T01:17:52.1864455Z -LL |     for i in ((3 - 1)..(5 + 2)).rev() {
2019-09-30T01:17:52.1864849Z -
2019-09-30T01:17:52.1865131Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1865348Z -  --> $DIR/for_loop_fixable.rs:98:15
2019-09-30T01:17:52.1865524Z -   |
2019-09-30T01:17:52.1865524Z -   |
2019-09-30T01:17:52.1865742Z -LL |     for _v in vec.iter() {}
2019-09-30T01:17:52.1866095Z -   |               ^^^^^^^^^^ help: to write this more concisely, try: `&vec`
2019-09-30T01:17:52.1866493Z -   = note: `-D clippy::explicit-iter-loop` implied by `-D warnings`
2019-09-30T01:17:52.1866680Z -
2019-09-30T01:17:52.1866937Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1867155Z -  --> $DIR/for_loop_fixable.rs:100:15
2019-09-30T01:17:52.1867155Z -  --> $DIR/for_loop_fixable.rs:100:15
2019-09-30T01:17:52.1867343Z -   |
2019-09-30T01:17:52.1867539Z -LL |     for _v in vec.iter_mut() {}
2019-09-30T01:17:52.1867789Z -   |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&mut vec`
2019-09-30T01:17:52.1868224Z -error: it is more concise to loop over containers instead of using explicit iteration methods`
2019-09-30T01:17:52.1868428Z -  --> $DIR/for_loop_fixable.rs:103:15
2019-09-30T01:17:52.1868613Z -   |
2019-09-30T01:17:52.1868613Z -   |
2019-09-30T01:17:52.1868815Z -LL |     for _v in out_vec.into_iter() {}
2019-09-30T01:17:52.1869059Z -   |               ^^^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `out_vec`
2019-09-30T01:17:52.1869477Z -   = note: `-D clippy::explicit-into-iter-loop` implied by `-D warnings`
2019-09-30T01:17:52.1869643Z -
2019-09-30T01:17:52.1869899Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1870133Z -  --> $DIR/for_loop_fixable.rs:106:15
2019-09-30T01:17:52.1870133Z -  --> $DIR/for_loop_fixable.rs:106:15
2019-09-30T01:17:52.1870300Z -   |
2019-09-30T01:17:52.1870497Z -LL |     for _v in array.into_iter() {}
2019-09-30T01:17:52.1870878Z -   |               ^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&array`
2019-09-30T01:17:52.1871354Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1871581Z -  --> $DIR/for_loop_fixable.rs:111:15
2019-09-30T01:17:52.1872041Z -   |
2019-09-30T01:17:52.1872041Z -   |
2019-09-30T01:17:52.1872254Z -LL |     for _v in [1, 2, 3].iter() {}
2019-09-30T01:17:52.1872917Z -   |               ^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[1, 2, 3]`
2019-09-30T01:17:52.1873410Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1873847Z -  --> $DIR/for_loop_fixable.rs:115:15
2019-09-30T01:17:52.1874073Z -   |
2019-09-30T01:17:52.1874073Z -   |
2019-09-30T01:17:52.1874281Z -LL |     for _v in [0; 32].iter() {}
2019-09-30T01:17:52.1874554Z -   |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[0; 32]`
2019-09-30T01:17:52.1875010Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1875225Z -  --> $DIR/for_loop_fixable.rs:120:15
2019-09-30T01:17:52.1875420Z -   |
2019-09-30T01:17:52.1875420Z -   |
2019-09-30T01:17:52.1875619Z -LL |     for _v in ll.iter() {}
2019-09-30T01:17:52.1875862Z -   |               ^^^^^^^^^ help: to write this more concisely, try: `&ll`
2019-09-30T01:17:52.1876418Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1876624Z -  --> $DIR/for_loop_fixable.rs:123:15
2019-09-30T01:17:52.1876810Z -   |
2019-09-30T01:17:52.1876810Z -   |
2019-09-30T01:17:52.1877001Z -LL |     for _v in vd.iter() {}
2019-09-30T01:17:52.1877242Z -   |               ^^^^^^^^^ help: to write this more concisely, try: `&vd`
2019-09-30T01:17:52.1877684Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1877900Z -  --> $DIR/for_loop_fixable.rs:126:15
2019-09-30T01:17:52.1878087Z -   |
2019-09-30T01:17:52.1878087Z -   |
2019-09-30T01:17:52.1878281Z -LL |     for _v in bh.iter() {}
2019-09-30T01:17:52.1878512Z -   |               ^^^^^^^^^ help: to write this more concisely, try: `&bh`
2019-09-30T01:17:52.1878952Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1879160Z -  --> $DIR/for_loop_fixable.rs:129:15
2019-09-30T01:17:52.1879328Z -   |
2019-09-30T01:17:52.1879328Z -   |
2019-09-30T01:17:52.1879536Z -LL |     for _v in hm.iter() {}
2019-09-30T01:17:52.1879768Z -   |               ^^^^^^^^^ help: to write this more concisely, try: `&hm`
2019-09-30T01:17:52.1880217Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1880424Z -  --> $DIR/for_loop_fixable.rs:132:15
2019-09-30T01:17:52.1880595Z -   |
2019-09-30T01:17:52.1880595Z -   |
2019-09-30T01:17:52.1880812Z -LL |     for _v in bt.iter() {}
2019-09-30T01:17:52.1881047Z -   |               ^^^^^^^^^ help: to write this more concisely, try: `&bt`
2019-09-30T01:17:52.1881593Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1881825Z -  --> $DIR/for_loop_fixable.rs:135:15
2019-09-30T01:17:52.1881999Z -   |
2019-09-30T01:17:52.1881999Z -   |
2019-09-30T01:17:52.1882197Z -LL |     for _v in hs.iter() {}
2019-09-30T01:17:52.1882749Z -   |               ^^^^^^^^^ help: to write this more concisely, try: `&hs`
2019-09-30T01:17:52.1883242Z -error: it is more concise to loop over references to containers instead of using explicit iteration methods
2019-09-30T01:17:52.1883492Z -  --> $DIR/for_loop_fixable.rs:138:15
2019-09-30T01:17:52.1883667Z -   |
2019-09-30T01:17:52.1883667Z -   |
2019-09-30T01:17:52.1883865Z -LL |     for _v in bs.iter() {}
2019-09-30T01:17:52.1884121Z -   |               ^^^^^^^^^ help: to write this more concisely, try: `&bs`
2019-09-30T01:17:52.1884683Z -error: aborting due to 18 previous errors
2019-09-30T01:17:52.1884855Z -
2019-09-30T01:17:52.1885043Z -
2019-09-30T01:17:52.1885073Z 
2019-09-30T01:17:52.1885073Z 
2019-09-30T01:17:52.1885120Z The actual stderr differed from the expected stderr.
2019-09-30T01:17:52.1885500Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/for_loop_fixable.stderr
2019-09-30T01:17:52.1885564Z normalized fixed:
2019-09-30T01:17:52.1885753Z // run-rustfix
2019-09-30T01:17:52.1885805Z 
2019-09-30T01:17:52.1885848Z #![allow(dead_code, unused)]
2019-09-30T01:17:52.1885918Z use std::collections::*;
2019-09-30T01:17:52.1886060Z 
2019-09-30T01:17:52.1886102Z #[warn(clippy::all)]
2019-09-30T01:17:52.1886102Z #[warn(clippy::all)]
2019-09-30T01:17:52.1886146Z struct Unrelated(Vec<u8>);
2019-09-30T01:17:52.1886187Z impl Unrelated {
2019-09-30T01:17:52.1886453Z     fn next(&self) -> std::slice::Iter<u8> {
2019-09-30T01:17:52.1886512Z         self.0.iter()
2019-09-30T01:17:52.1886598Z 
2019-09-30T01:17:52.1886814Z     fn iter(&self) -> std::slice::Iter<u8> {
2019-09-30T01:17:52.1886860Z         self.0.iter()
2019-09-30T01:17:52.1886899Z     }
2019-09-30T01:17:52.1886899Z     }
2019-09-30T01:17:52.1886954Z }
2019-09-30T01:17:52.1886980Z 
2019-09-30T01:17:52.1887018Z #[warn(
2019-09-30T01:17:52.1887059Z     clippy::needless_range_loop,
2019-09-30T01:17:52.1887118Z     clippy::explicit_iter_loop,
2019-09-30T01:17:52.1887163Z     clippy::explicit_into_iter_loop,
2019-09-30T01:17:52.1887207Z     clippy::iter_next_loop,
2019-09-30T01:17:52.1887265Z     clippy::reverse_range_loop,
2019-09-30T01:17:52.1887308Z     clippy::for_kv_map
2019-09-30T01:17:52.1887356Z )]
2019-09-30T01:17:52.1887412Z #[allow(
2019-09-30T01:17:52.1887452Z     clippy::linkedlist,
2019-09-30T01:17:52.1887494Z     clippy::shadow_unrelated,
2019-09-30T01:17:52.1887537Z     clippy::unnecessary_mut_passed,
2019-09-30T01:17:52.1887596Z     clippy::cognitive_complexity,
2019-09-30T01:17:52.1887645Z     clippy::similar_names
2019-09-30T01:17:52.1887685Z )]
2019-09-30T01:17:52.1887747Z #[allow(clippy::many_single_char_names, unused_variables, clippy::into_iter_on_array)]
2019-09-30T01:17:52.1887948Z     const MAX_LEN: usize = 42;
2019-09-30T01:17:52.1888004Z     let mut vec = vec![1, 2, 3, 4];
2019-09-30T01:17:52.1888031Z 
2019-09-30T01:17:52.1888070Z     for i in 10..0 {
---
2019-09-30T01:17:52.1889387Z         // not an error
2019-09-30T01:17:52.1889444Z         println!("{}", i);
2019-09-30T01:17:52.1889481Z     }
2019-09-30T01:17:52.1889505Z 
2019-09-30T01:17:52.1889544Z     for i in (10..0).map(|x| x * 2) {
2019-09-30T01:17:52.1889812Z         // not an error, it can't be known what arbitrary methods do to a range
2019-09-30T01:17:52.1889898Z     }
2019-09-30T01:17:52.1889923Z 
2019-09-30T01:17:52.1889980Z     // testing that the empty range lint folds constants
2019-09-30T01:17:52.1890109Z     for i in 10..5 + 4 {
2019-09-30T01:17:52.1890109Z     for i in 10..5 + 4 {
2019-09-30T01:17:52.1890159Z         println!("{}", i);
2019-09-30T01:17:52.1890213Z     }
2019-09-30T01:17:52.1890236Z 
2019-09-30T01:17:52.1890460Z     for i in (5 + 2)..(3 - 1) {
2019-09-30T01:17:52.1890559Z     }
2019-09-30T01:17:52.1890583Z 
2019-09-30T01:17:52.1890583Z 
2019-09-30T01:17:52.1890621Z     for i in (2 * 2)..(2 * 3) {
2019-09-30T01:17:52.1890663Z         // no error, 4..6 is fine
2019-09-30T01:17:52.1890758Z     }
2019-09-30T01:17:52.1890782Z 
2019-09-30T01:17:52.1890837Z     let x = 42;
2019-09-30T01:17:52.1890878Z     for i in x..10 {
2019-09-30T01:17:52.1890878Z     for i in x..10 {
2019-09-30T01:17:52.1891194Z         // no error, not constant-foldable
2019-09-30T01:17:52.1891239Z         println!("{}", i);
2019-09-30T01:17:52.1891296Z     }
2019-09-30T01:17:52.1891321Z 
2019-09-30T01:17:52.1891358Z     // See #601
2019-09-30T01:17:52.1891397Z     for i in 0..10 {
2019-09-30T01:17:52.1891467Z         // no error, id_col does not exist outside the loop
2019-09-30T01:17:52.1891513Z         let mut id_col = vec![0f64; 10];
2019-09-30T01:17:52.1891554Z         id_col[i] = 1f64;
2019-09-30T01:17:52.1891632Z 
2019-09-30T01:17:52.1891632Z 
2019-09-30T01:17:52.1891672Z     for _v in vec.iter() {}
2019-09-30T01:17:52.1891698Z 
2019-09-30T01:17:52.1891754Z     for _v in vec.iter_mut() {}
2019-09-30T01:17:52.1891820Z     let out_vec = vec![1, 2, 3];
2019-09-30T01:17:52.1891820Z     let out_vec = vec![1, 2, 3];
2019-09-30T01:17:52.1891862Z     for _v in out_vec.into_iter() {}
2019-09-30T01:17:52.1891905Z 
2019-09-30T01:17:52.1891943Z     let array = [1, 2, 3];
2019-09-30T01:17:52.1891985Z     for _v in array.into_iter() {}
2019-09-30T01:17:52.1892019Z 
2019-09-30T01:17:52.1892075Z     for _v in &vec {} // these are fine
2019-09-30T01:17:52.1892118Z     for _v in &mut vec {} // these are fine
2019-09-30T01:17:52.1892146Z 
2019-09-30T01:17:52.1892184Z     for _v in [1, 2, 3].iter() {}
2019-09-30T01:17:52.1892225Z 
2019-09-30T01:17:52.1892273Z     for _v in (&mut [1, 2, 3]).iter() {} // no error
2019-09-30T01:17:52.1892301Z 
2019-09-30T01:17:52.1892340Z     for _v in [0; 32].iter() {}
2019-09-30T01:17:52.1892381Z 
2019-09-30T01:17:52.1892734Z     for _v in [0; 33].iter() {} // no error
2019-09-30T01:17:52.1892781Z 
2019-09-30T01:17:52.1892824Z     let ll: LinkedList<()> = LinkedList::new();
2019-09-30T01:17:52.1892888Z     for _v in ll.iter() {}
2019-09-30T01:17:52.1893062Z 
2019-09-30T01:17:52.1893207Z     let vd: VecDeque<()> = VecDeque::new();
2019-09-30T01:17:52.1893272Z     for _v in vd.iter() {}
2019-09-30T01:17:52.1893299Z 
2019-09-30T01:17:52.1893342Z     let bh: BinaryHeap<()> = BinaryHeap::new();
2019-09-30T01:17:52.1893386Z     for _v in bh.iter() {}
2019-09-30T01:17:52.1893422Z 
2019-09-30T01:17:52.1893483Z     let hm: HashMap<(), ()> = HashMap::new();
2019-09-30T01:17:52.1893527Z     for _v in hm.iter() {}
2019-09-30T01:17:52.1893553Z 
2019-09-30T01:17:52.1893612Z     let bt: BTreeMap<(), ()> = BTreeMap::new();
2019-09-30T01:17:52.1893662Z     for _v in bt.iter() {}
2019-09-30T01:17:52.1893690Z 
2019-09-30T01:17:52.1893731Z     let hs: HashSet<()> = HashSet::new();
2019-09-30T01:17:52.1893790Z     for _v in hs.iter() {}
2019-09-30T01:17:52.1893817Z 
2019-09-30T01:17:52.1893858Z     let bs: BTreeSet<()> = BTreeSet::new();
2019-09-30T01:17:52.1893901Z     for _v in bs.iter() {}
2019-09-30T01:17:52.1893943Z 
2019-09-30T01:17:52.1893984Z     let u = Unrelated(vec![]);
2019-09-30T01:17:52.1894026Z     for _v in u.next() {} // no error
2019-09-30T01:17:52.1894084Z     for _v in u.iter() {} // no error
2019-09-30T01:17:52.1894151Z     let mut out = vec![];
2019-09-30T01:17:52.1894151Z     let mut out = vec![];
2019-09-30T01:17:52.1894198Z     vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();
2019-09-30T01:17:52.1894275Z     let _y = vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>(); // this is fine
2019-09-30T01:17:52.1894349Z     // Loop with explicit counter variable
2019-09-30T01:17:52.1894377Z 
2019-09-30T01:17:52.1894554Z     // Potential false positives
2019-09-30T01:17:52.1894610Z     let mut _index = 0;
2019-09-30T01:17:52.1894610Z     let mut _index = 0;
2019-09-30T01:17:52.1894652Z     _index = 1;
2019-09-30T01:17:52.1894710Z     for _v in &vec {
2019-09-30T01:17:52.1894752Z         _index += 1
2019-09-30T01:17:52.1894817Z 
2019-09-30T01:17:52.1894875Z     let mut _index = 0;
2019-09-30T01:17:52.1894916Z     _index += 1;
2019-09-30T01:17:52.1894916Z     _index += 1;
2019-09-30T01:17:52.1894957Z     for _v in &vec {
2019-09-30T01:17:52.1894998Z         _index += 1
2019-09-30T01:17:52.1895079Z 
2019-09-30T01:17:52.1895119Z     let mut _index = 0;
2019-09-30T01:17:52.1895159Z     if true {
2019-09-30T01:17:52.1895215Z         _index = 1
2019-09-30T01:17:52.1895215Z         _index = 1
2019-09-30T01:17:52.1895321Z     }
2019-09-30T01:17:52.1895361Z     for _v in &vec {
2019-09-30T01:17:52.1895418Z         _index += 1
2019-09-30T01:17:52.1895483Z 
2019-09-30T01:17:52.1895522Z     let mut _index = 0;
2019-09-30T01:17:52.1895579Z     let mut _index = 1;
2019-09-30T01:17:52.1895579Z     let mut _index = 1;
2019-09-30T01:17:52.1895627Z     for _v in &vec {
2019-09-30T01:17:52.1895669Z         _index += 1
2019-09-30T01:17:52.1895748Z 
2019-09-30T01:17:52.1895788Z     let mut _index = 0;
2019-09-30T01:17:52.1895788Z     let mut _index = 0;
2019-09-30T01:17:52.1895829Z     for _v in &vec {
2019-09-30T01:17:52.1895887Z         _index += 1;
2019-09-30T01:17:52.1895927Z         _index += 1
2019-09-30T01:17:52.1896098Z 
2019-09-30T01:17:52.1896153Z     let mut _index = 0;
2019-09-30T01:17:52.1896153Z     let mut _index = 0;
2019-09-30T01:17:52.1896192Z     for _v in &vec {
2019-09-30T01:17:52.1896231Z         _index *= 2;
2019-09-30T01:17:52.1896286Z         _index += 1
2019-09-30T01:17:52.1896347Z 
2019-09-30T01:17:52.1896392Z     let mut _index = 0;
2019-09-30T01:17:52.1896392Z     let mut _index = 0;
2019-09-30T01:17:52.1896447Z     for _v in &vec {
2019-09-30T01:17:52.1896486Z         _index = 1;
2019-09-30T01:17:52.1896525Z         _index += 1
2019-09-30T01:17:52.1896602Z 
2019-09-30T01:17:52.1896639Z     let mut _index = 0;
2019-09-30T01:17:52.1896672Z 
2019-09-30T01:17:52.1896672Z 
2019-09-30T01:17:52.1896710Z     for _v in &vec {
2019-09-30T01:17:52.1896765Z         let mut _index = 0;
2019-09-30T01:17:52.1896805Z         _index += 1
2019-09-30T01:17:52.1896866Z 
2019-09-30T01:17:52.1896922Z     let mut _index = 0;
2019-09-30T01:17:52.1896922Z     let mut _index = 0;
2019-09-30T01:17:52.1896960Z     for _v in &vec {
2019-09-30T01:17:52.1897000Z         _index += 1;
2019-09-30T01:17:52.1897054Z         _index = 0;
2019-09-30T01:17:52.1897115Z 
2019-09-30T01:17:52.1897153Z     let mut _index = 0;
2019-09-30T01:17:52.1897153Z     let mut _index = 0;
2019-09-30T01:17:52.1897209Z     for _v in &vec {
2019-09-30T01:17:52.1897250Z         for _x in 0..1 {
2019-09-30T01:17:52.1897290Z             _index += 1;
2019-09-30T01:17:52.1897391Z         _index += 1
2019-09-30T01:17:52.1897428Z     }
2019-09-30T01:17:52.1897452Z 
2019-09-30T01:17:52.1897507Z     let mut _index = 0;
---
2019-09-30T01:17:52.1897802Z     let mut _index = 0;
2019-09-30T01:17:52.1897841Z     if true {
2019-09-30T01:17:52.1897880Z         _index = 1
2019-09-30T01:17:52.1897919Z     };
2019-09-30T01:17:52.1897975Z     for _v in &vec {
2019-09-30T01:17:52.1898013Z         _index += 1
2019-09-30T01:17:52.1898075Z 
2019-09-30T01:17:52.1898130Z     let mut _index = 1;
2019-09-30T01:17:52.1898169Z     if false {
2019-09-30T01:17:52.1898209Z         _index = 0
2019-09-30T01:17:52.1898209Z         _index = 0
2019-09-30T01:17:52.1898264Z     };
2019-09-30T01:17:52.1898303Z     for _v in &vec {
2019-09-30T01:17:52.1898348Z         _index += 1
2019-09-30T01:17:52.1898427Z 
2019-09-30T01:17:52.1898465Z     let mut index = 0;
2019-09-30T01:17:52.1898503Z     {
2019-09-30T01:17:52.1898557Z         let mut _x = &mut index;
2019-09-30T01:17:52.1898557Z         let mut _x = &mut index;
2019-09-30T01:17:52.1898597Z     }
2019-09-30T01:17:52.1898695Z     for _v in &vec {
2019-09-30T01:17:52.1898743Z         _index += 1
2019-09-30T01:17:52.1898823Z 
2019-09-30T01:17:52.1898862Z     let mut index = 0;
2019-09-30T01:17:52.1898862Z     let mut index = 0;
2019-09-30T01:17:52.1898902Z     for _v in &vec {
2019-09-30T01:17:52.1898959Z         index += 1
2019-09-30T01:17:52.1898996Z     }
2019-09-30T01:17:52.1899035Z     println!("index: {}", index);
2019-09-30T01:17:52.1899082Z 
2019-09-30T01:17:52.1899381Z     fn f<T>(_: &T, _: &T) -> bool {
2019-09-30T01:17:52.1899463Z     }
2019-09-30T01:17:52.1899463Z     }
2019-09-30T01:17:52.1899522Z     fn g<T>(_: &mut [T], _: usize, _: usize) {
2019-09-30T01:17:52.1899726Z     }
2019-09-30T01:17:52.1899726Z     }
2019-09-30T01:17:52.1899783Z     for i in 1..vec.len() {
2019-09-30T01:17:52.1900014Z         if f(&vec[i - 1], &vec[i]) {
2019-09-30T01:17:52.1900210Z             g(&mut vec, i - 1, i);
2019-09-30T01:17:52.1900310Z     }
2019-09-30T01:17:52.1900344Z 
2019-09-30T01:17:52.1900344Z 
2019-09-30T01:17:52.1900384Z     for mid in 1..vec.len() {
2019-09-30T01:17:52.1900443Z         let (_, _) = vec.split_at(mid);
2019-09-30T01:17:52.1900518Z }
2019-09-30T01:17:52.1900542Z 
2019-09-30T01:17:52.1900542Z 
2019-09-30T01:17:52.1900782Z fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
2019-09-30T01:17:52.1900979Z     let pivot = v.len() - 1;
2019-09-30T01:17:52.1901023Z     let mut i = 0;
2019-09-30T01:17:52.1901079Z     for j in 0..pivot {
2019-09-30T01:17:52.1901122Z         if v[j] <= v[pivot] {
2019-09-30T01:17:52.1901163Z             v.swap(i, j);
2019-09-30T01:17:52.1901202Z             i += 1;
2019-09-30T01:17:52.1901302Z     }
2019-09-30T01:17:52.1901302Z     }
2019-09-30T01:17:52.1901340Z     v.swap(i, pivot);
2019-09-30T01:17:52.1901429Z }
2019-09-30T01:17:52.1901453Z 
2019-09-30T01:17:52.1901493Z #[warn(clippy::needless_range_loop)]
2019-09-30T01:17:52.1901493Z #[warn(clippy::needless_range_loop)]
2019-09-30T01:17:52.1901564Z pub fn manual_copy_same_destination(dst: &mut [i32], d: usize, s: usize) {
2019-09-30T01:17:52.1901789Z     // Same source and destination - don't trigger lint
2019-09-30T01:17:52.1901834Z     for i in 0..dst.len() {
2019-09-30T01:17:52.1901894Z         dst[d + i] = dst[s + i];
2019-09-30T01:17:52.1901968Z }
2019-09-30T01:17:52.1901992Z 
2019-09-30T01:17:52.1902048Z mod issue_2496 {
2019-09-30T01:17:52.1902088Z     pub trait Handle {
2019-09-30T01:17:52.1902088Z     pub trait Handle {
2019-09-30T01:17:52.1902298Z         fn new_for_index(index: usize) -> Self;
2019-09-30T01:17:52.1902888Z         fn index(&self) -> usize;
2019-09-30T01:17:52.1902969Z 
2019-09-30T01:17:52.1902969Z 
2019-09-30T01:17:52.1903184Z     pub fn test<H: Handle>() -> H {
2019-09-30T01:17:52.1903258Z         for x in 0..5 {
2019-09-30T01:17:52.1903302Z             let next_handle = H::new_for_index(x);
2019-09-30T01:17:52.1903350Z             println!("{}", next_handle.index());
2019-09-30T01:17:52.1903449Z         unimplemented!()
2019-09-30T01:17:52.1903598Z     }
2019-09-30T01:17:52.1903636Z }
2019-09-30T01:17:52.1903677Z 
2019-09-30T01:17:52.1903677Z 
2019-09-30T01:17:52.1903702Z 
2019-09-30T01:17:52.1903741Z expected fixed:
2019-09-30T01:17:52.1903942Z // run-rustfix
2019-09-30T01:17:52.1903988Z 
2019-09-30T01:17:52.1904031Z #![allow(dead_code, unused)]
2019-09-30T01:17:52.1904098Z use std::collections::*;
2019-09-30T01:17:52.1904125Z 
2019-09-30T01:17:52.1904180Z #[warn(clippy::all)]
2019-09-30T01:17:52.1904180Z #[warn(clippy::all)]
2019-09-30T01:17:52.1904222Z struct Unrelated(Vec<u8>);
2019-09-30T01:17:52.1904262Z impl Unrelated {
2019-09-30T01:17:52.1904495Z     fn next(&self) -> std::slice::Iter<u8> {
2019-09-30T01:17:52.1904542Z         self.0.iter()
2019-09-30T01:17:52.1904615Z 
2019-09-30T01:17:52.1904844Z     fn iter(&self) -> std::slice::Iter<u8> {
2019-09-30T01:17:52.1904890Z         self.0.iter()
2019-09-30T01:17:52.1904929Z     }
2019-09-30T01:17:52.1904929Z     }
2019-09-30T01:17:52.1904982Z }
2019-09-30T01:17:52.1905007Z 
2019-09-30T01:17:52.1905045Z #[warn(
2019-09-30T01:17:52.1905205Z     clippy::needless_range_loop,
2019-09-30T01:17:52.1905278Z     clippy::explicit_iter_loop,
2019-09-30T01:17:52.1905322Z     clippy::explicit_into_iter_loop,
2019-09-30T01:17:52.1905365Z     clippy::iter_next_loop,
2019-09-30T01:17:52.1905424Z     clippy::reverse_range_loop,
2019-09-30T01:17:52.1905467Z     clippy::for_kv_map
2019-09-30T01:17:52.1905506Z )]
2019-09-30T01:17:52.1905546Z #[allow(
2019-09-30T01:17:52.1905602Z     clippy::linkedlist,
2019-09-30T01:17:52.1905644Z     clippy::shadow_unrelated,
2019-09-30T01:17:52.1905687Z     clippy::unnecessary_mut_passed,
2019-09-30T01:17:52.1905748Z     clippy::cognitive_complexity,
2019-09-30T01:17:52.1905791Z     clippy::similar_names
2019-09-30T01:17:52.1906003Z )]
2019-09-30T01:17:52.1906065Z #[allow(clippy::many_single_char_names, unused_variables, clippy::into_iter_on_array)]
2019-09-30T01:17:52.1906151Z     const MAX_LEN: usize = 42;
2019-09-30T01:17:52.1906192Z     let mut vec = vec![1, 2, 3, 4];
2019-09-30T01:17:52.1906242Z 
2019-09-30T01:17:52.1906242Z 
2019-09-30T01:17:52.1906280Z     for i in (0..10).rev() {
2019-09-30T01:17:52.1906360Z     }
2019-09-30T01:17:52.1906401Z 
2019-09-30T01:17:52.1906401Z 
2019-09-30T01:17:52.1906440Z     for i in (0..=10).rev() {
2019-09-30T01:17:52.1906530Z     }
2019-09-30T01:17:52.1906554Z 
2019-09-30T01:17:52.1906554Z 
2019-09-30T01:17:52.1906593Z     for i in (0..MAX_LEN).rev() {
2019-09-30T01:17:52.1923827Z     }
2019-09-30T01:17:52.1923857Z 
2019-09-30T01:17:52.1923900Z     for i in 5..=5 {
2019-09-30T01:17:52.1924372Z         // not an error, this is the range with only one element “5”
---
2019-09-30T01:17:52.1924995Z         // not an error
2019-09-30T01:17:52.1925046Z         println!("{}", i);
2019-09-30T01:17:52.1925085Z     }
2019-09-30T01:17:52.1925111Z 
2019-09-30T01:17:52.1925153Z     for i in (10..0).map(|x| x * 2) {
2019-09-30T01:17:52.1925660Z         // not an error, it can't be known what arbitrary methods do to a range
2019-09-30T01:17:52.1925756Z     }
2019-09-30T01:17:52.1925796Z 
2019-09-30T01:17:52.1925841Z     // testing that the empty range lint folds constants
2019-09-30T01:17:52.1925841Z     // testing that the empty range lint folds constants
2019-09-30T01:17:52.1925996Z     for i in (5 + 4..10).rev() {
2019-09-30T01:17:52.1926096Z     }
2019-09-30T01:17:52.1926120Z 
2019-09-30T01:17:52.1926120Z 
2019-09-30T01:17:52.1926348Z     for i in ((3 - 1)..(5 + 2)).rev() {
2019-09-30T01:17:52.1926448Z     }
2019-09-30T01:17:52.1926480Z 
2019-09-30T01:17:52.1926480Z 
2019-09-30T01:17:52.1926520Z     for i in (2 * 2)..(2 * 3) {
2019-09-30T01:17:52.1926569Z         // no error, 4..6 is fine
2019-09-30T01:17:52.1926649Z     }
2019-09-30T01:17:52.1926673Z 
2019-09-30T01:17:52.1926719Z     let x = 42;
2019-09-30T01:17:52.1926759Z     for i in x..10 {
2019-09-30T01:17:52.1926759Z     for i in x..10 {
2019-09-30T01:17:52.1926984Z         // no error, not constant-foldable
2019-09-30T01:17:52.1927035Z         println!("{}", i);
2019-09-30T01:17:52.1927073Z     }
2019-09-30T01:17:52.1927097Z 
2019-09-30T01:17:52.1927135Z     // See #601
2019-09-30T01:17:52.1927179Z     for i in 0..10 {
2019-09-30T01:17:52.1927223Z         // no error, id_col does not exist outside the loop
2019-09-30T01:17:52.1927276Z         let mut id_col = vec![0f64; 10];
2019-09-30T01:17:52.1927443Z         id_col[i] = 1f64;
2019-09-30T01:17:52.1927509Z 
2019-09-30T01:17:52.1927509Z 
2019-09-30T01:17:52.1927549Z     for _v in &vec {}
2019-09-30T01:17:52.1927576Z 
2019-09-30T01:17:52.1927800Z     for _v in &mut vec {}
2019-09-30T01:17:52.1927881Z     let out_vec = vec![1, 2, 3];
2019-09-30T01:17:52.1927881Z     let out_vec = vec![1, 2, 3];
2019-09-30T01:17:52.1927932Z     for _v in out_vec {}
2019-09-30T01:17:52.1927959Z 
2019-09-30T01:17:52.1927999Z     let array = [1, 2, 3];
2019-09-30T01:17:52.1928041Z     for _v in &array {}
2019-09-30T01:17:52.1928067Z 
2019-09-30T01:17:52.1928122Z     for _v in &vec {} // these are fine
2019-09-30T01:17:52.1928166Z     for _v in &mut vec {} // these are fine
2019-09-30T01:17:52.1928195Z 
2019-09-30T01:17:52.1928246Z     for _v in &[1, 2, 3] {}
2019-09-30T01:17:52.1928273Z 
2019-09-30T01:17:52.1928316Z     for _v in (&mut [1, 2, 3]).iter() {} // no error
2019-09-30T01:17:52.1928419Z 
2019-09-30T01:17:52.1928468Z     for _v in &[0; 32] {}
2019-09-30T01:17:52.1928495Z 
2019-09-30T01:17:52.1928537Z     for _v in [0; 33].iter() {} // no error
2019-09-30T01:17:52.1928566Z 
2019-09-30T01:17:52.1928609Z     let ll: LinkedList<()> = LinkedList::new();
2019-09-30T01:17:52.1928674Z     for _v in &ll {}
2019-09-30T01:17:52.1928701Z 
2019-09-30T01:17:52.1928743Z     let vd: VecDeque<()> = VecDeque::new();
2019-09-30T01:17:52.1928792Z     for _v in &vd {}
2019-09-30T01:17:52.1928819Z 
2019-09-30T01:17:52.1928862Z     let bh: BinaryHeap<()> = BinaryHeap::new();
2019-09-30T01:17:52.1928906Z     for _v in &bh {}
2019-09-30T01:17:52.1928941Z 
2019-09-30T01:17:52.1928983Z     let hm: HashMap<(), ()> = HashMap::new();
2019-09-30T01:17:52.1929026Z     for _v in &hm {}
2019-09-30T01:17:52.1929053Z 
2019-09-30T01:17:52.1929101Z     let bt: BTreeMap<(), ()> = BTreeMap::new();
2019-09-30T01:17:52.1929144Z     for _v in &bt {}
2019-09-30T01:17:52.1929170Z 
2019-09-30T01:17:52.1929212Z     let hs: HashSet<()> = HashSet::new();
2019-09-30T01:17:52.1929267Z     for _v in &hs {}
2019-09-30T01:17:52.1929293Z 
2019-09-30T01:17:52.1929334Z     let bs: BTreeSet<()> = BTreeSet::new();
2019-09-30T01:17:52.1929377Z     for _v in &bs {}
2019-09-30T01:17:52.1929408Z 
2019-09-30T01:17:52.1929455Z     let u = Unrelated(vec![]);
2019-09-30T01:17:52.1929500Z     for _v in u.next() {} // no error
2019-09-30T01:17:52.1929549Z     for _v in u.iter() {} // no error
2019-09-30T01:17:52.1929616Z     let mut out = vec![];
2019-09-30T01:17:52.1929616Z     let mut out = vec![];
2019-09-30T01:17:52.1929663Z     vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();
2019-09-30T01:17:52.1929722Z     let _y = vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>(); // this is fine
2019-09-30T01:17:52.1929797Z     // Loop with explicit counter variable
2019-09-30T01:17:52.1929824Z 
2019-09-30T01:17:52.1929876Z     // Potential false positives
2019-09-30T01:17:52.1929919Z     let mut _index = 0;
2019-09-30T01:17:52.1929919Z     let mut _index = 0;
2019-09-30T01:17:52.1929967Z     _index = 1;
2019-09-30T01:17:52.1930023Z     for _v in &vec {
2019-09-30T01:17:52.1930065Z         _index += 1
2019-09-30T01:17:52.1930129Z 
2019-09-30T01:17:52.1930177Z     let mut _index = 0;
2019-09-30T01:17:52.1930219Z     _index += 1;
2019-09-30T01:17:52.1930219Z     _index += 1;
2019-09-30T01:17:52.1930267Z     for _v in &vec {
2019-09-30T01:17:52.1930308Z         _index += 1
2019-09-30T01:17:52.1930378Z 
2019-09-30T01:17:52.1930417Z     let mut _index = 0;
2019-09-30T01:17:52.1930468Z     if true {
2019-09-30T01:17:52.1930509Z         _index = 1
2019-09-30T01:17:52.1930509Z         _index = 1
2019-09-30T01:17:52.1930548Z     }
2019-09-30T01:17:52.1930587Z     for _v in &vec {
2019-09-30T01:17:52.1930634Z         _index += 1
2019-09-30T01:17:52.1930697Z 
2019-09-30T01:17:52.1930736Z     let mut _index = 0;
2019-09-30T01:17:52.1930784Z     let mut _index = 1;
2019-09-30T01:17:52.1930784Z     let mut _index = 1;
2019-09-30T01:17:52.1930824Z     for _v in &vec {
2019-09-30T01:17:52.1930865Z         _index += 1
2019-09-30T01:17:52.1930941Z 
2019-09-30T01:17:52.1930981Z     let mut _index = 0;
2019-09-30T01:17:52.1930981Z     let mut _index = 0;
2019-09-30T01:17:52.1931129Z     for _v in &vec {
2019-09-30T01:17:52.1931174Z         _index += 1;
2019-09-30T01:17:52.1931213Z         _index += 1
2019-09-30T01:17:52.1931275Z 
2019-09-30T01:17:52.1931379Z     let mut _index = 0;
2019-09-30T01:17:52.1931379Z     let mut _index = 0;
2019-09-30T01:17:52.1931428Z     for _v in &vec {
2019-09-30T01:17:52.1931467Z         _index *= 2;
2019-09-30T01:17:52.1931513Z         _index += 1
2019-09-30T01:17:52.1931575Z 
2019-09-30T01:17:52.1931613Z     let mut _index = 0;
2019-09-30T01:17:52.1931613Z     let mut _index = 0;
2019-09-30T01:17:52.1931657Z     for _v in &vec {
2019-09-30T01:17:52.1931696Z         _index = 1;
2019-09-30T01:17:52.1931735Z         _index += 1
2019-09-30T01:17:52.1931804Z 
2019-09-30T01:17:52.1931843Z     let mut _index = 0;
2019-09-30T01:17:52.1931868Z 
2019-09-30T01:17:52.1931868Z 
2019-09-30T01:17:52.1931905Z     for _v in &vec {
2019-09-30T01:17:52.1931958Z         let mut _index = 0;
2019-09-30T01:17:52.1932060Z         _index += 1
2019-09-30T01:17:52.1932131Z 
2019-09-30T01:17:52.1932169Z     let mut _index = 0;
2019-09-30T01:17:52.1932169Z     let mut _index = 0;
2019-09-30T01:17:52.1932209Z     for _v in &vec {
2019-09-30T01:17:52.1932248Z         _index += 1;
2019-09-30T01:17:52.1932300Z         _index = 0;
2019-09-30T01:17:52.1932362Z 
2019-09-30T01:17:52.1932400Z     let mut _index = 0;
2019-09-30T01:17:52.1932400Z     let mut _index = 0;
2019-09-30T01:17:52.1932806Z     for _v in &vec {
2019-09-30T01:17:52.1932879Z         for _x in 0..1 {
2019-09-30T01:17:52.1932921Z             _index += 1;
2019-09-30T01:17:52.1933011Z         _index += 1
2019-09-30T01:17:52.1933050Z     }
2019-09-30T01:17:52.1933075Z 
2019-09-30T01:17:52.1933122Z     let mut _index = 0;
---
2019-09-30T01:17:52.1933411Z     let mut _index = 0;
2019-09-30T01:17:52.1933451Z     if true {
2019-09-30T01:17:52.1933492Z         _index = 1
2019-09-30T01:17:52.1933531Z     };
2019-09-30T01:17:52.1933577Z     for _v in &vec {
2019-09-30T01:17:52.1933618Z         _index += 1
2019-09-30T01:17:52.1933694Z 
2019-09-30T01:17:52.1933733Z     let mut _index = 1;
2019-09-30T01:17:52.1933774Z     if false {
2019-09-30T01:17:52.1933814Z         _index = 0
2019-09-30T01:17:52.1933814Z         _index = 0
2019-09-30T01:17:52.1933858Z     };
2019-09-30T01:17:52.1933898Z     for _v in &vec {
2019-09-30T01:17:52.1933938Z         _index += 1
2019-09-30T01:17:52.1934015Z 
2019-09-30T01:17:52.1934054Z     let mut index = 0;
2019-09-30T01:17:52.1934093Z     {
2019-09-30T01:17:52.1934145Z         let mut _x = &mut index;
2019-09-30T01:17:52.1934145Z         let mut _x = &mut index;
2019-09-30T01:17:52.1934185Z     }
2019-09-30T01:17:52.1934224Z     for _v in &vec {
2019-09-30T01:17:52.1934264Z         _index += 1
2019-09-30T01:17:52.1934340Z 
2019-09-30T01:17:52.1934379Z     let mut index = 0;
2019-09-30T01:17:52.1934379Z     let mut index = 0;
2019-09-30T01:17:52.1934426Z     for _v in &vec {
2019-09-30T01:17:52.1934466Z         index += 1
2019-09-30T01:17:52.1934504Z     }
2019-09-30T01:17:52.1934546Z     println!("index: {}", index);
2019-09-30T01:17:52.1934593Z 
2019-09-30T01:17:52.1934914Z     fn f<T>(_: &T, _: &T) -> bool {
2019-09-30T01:17:52.1935002Z     }
2019-09-30T01:17:52.1935002Z     }
2019-09-30T01:17:52.1935059Z     fn g<T>(_: &mut [T], _: usize, _: usize) {
2019-09-30T01:17:52.1935142Z     }
2019-09-30T01:17:52.1935142Z     }
2019-09-30T01:17:52.1935193Z     for i in 1..vec.len() {
2019-09-30T01:17:52.1935409Z         if f(&vec[i - 1], &vec[i]) {
2019-09-30T01:17:52.1935614Z             g(&mut vec, i - 1, i);
2019-09-30T01:17:52.1935703Z     }
2019-09-30T01:17:52.1935751Z 
2019-09-30T01:17:52.1935751Z 
2019-09-30T01:17:52.1935793Z     for mid in 1..vec.len() {
2019-09-30T01:17:52.1935848Z         let (_, _) = vec.split_at(mid);
2019-09-30T01:17:52.1935945Z }
2019-09-30T01:17:52.1936078Z 
2019-09-30T01:17:52.1936078Z 
2019-09-30T01:17:52.1936307Z fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
2019-09-30T01:17:52.1936505Z     let pivot = v.len() - 1;
2019-09-30T01:17:52.1936691Z     let mut i = 0;
2019-09-30T01:17:52.1936742Z     for j in 0..pivot {
2019-09-30T01:17:52.1936783Z         if v[j] <= v[pivot] {
2019-09-30T01:17:52.1936842Z             v.swap(i, j);
2019-09-30T01:17:52.1936881Z             i += 1;
2019-09-30T01:17:52.1936972Z     }
2019-09-30T01:17:52.1936972Z     }
2019-09-30T01:17:52.1937010Z     v.swap(i, pivot);
2019-09-30T01:17:52.1937083Z }
2019-09-30T01:17:52.1937126Z 
2019-09-30T01:17:52.1937165Z #[warn(clippy::needless_range_loop)]
2019-09-30T01:17:52.1937165Z #[warn(clippy::needless_range_loop)]
2019-09-30T01:17:52.1937212Z pub fn manual_copy_same_destination(dst: &mut [i32], d: usize, s: usize) {
2019-09-30T01:17:52.1937487Z     // Same source and destination - don't trigger lint
2019-09-30T01:17:52.1937625Z     for i in 0..dst.len() {
2019-09-30T01:17:52.1937667Z         dst[d + i] = dst[s + i];
2019-09-30T01:17:52.1937761Z }
2019-09-30T01:17:52.1937786Z 
2019-09-30T01:17:52.1937824Z mod issue_2496 {
2019-09-30T01:17:52.1937864Z     pub trait Handle {
2019-09-30T01:17:52.1937864Z     pub trait Handle {
2019-09-30T01:17:52.1938127Z         fn new_for_index(index: usize) -> Self;
2019-09-30T01:17:52.1938327Z         fn index(&self) -> usize;
2019-09-30T01:17:52.1938411Z 
2019-09-30T01:17:52.1938411Z 
2019-09-30T01:17:52.1938606Z     pub fn test<H: Handle>() -> H {
2019-09-30T01:17:52.1938648Z         for x in 0..5 {
2019-09-30T01:17:52.1938691Z             let next_handle = H::new_for_index(x);
2019-09-30T01:17:52.1938754Z             println!("{}", next_handle.index());
2019-09-30T01:17:52.1938834Z         unimplemented!()
2019-09-30T01:17:52.1938890Z     }
2019-09-30T01:17:52.1938925Z }
2019-09-30T01:17:52.1938948Z 
2019-09-30T01:17:52.1938948Z 
2019-09-30T01:17:52.1938973Z 
2019-09-30T01:17:52.1939035Z diff of fixed:
2019-09-30T01:17:52.1939061Z 
2019-09-30T01:17:52.1939241Z  // run-rustfix
2019-09-30T01:17:52.1939281Z  
2019-09-30T01:17:52.1939338Z  #![allow(dead_code, unused)]
2019-09-30T01:17:52.1939415Z  use std::collections::*;
2019-09-30T01:17:52.1939452Z  
2019-09-30T01:17:52.1939515Z  #[warn(clippy::all)]
2019-09-30T01:17:52.1939515Z  #[warn(clippy::all)]
2019-09-30T01:17:52.1939556Z  struct Unrelated(Vec<u8>);
2019-09-30T01:17:52.1939597Z  impl Unrelated {
2019-09-30T01:17:52.1939827Z      fn next(&self) -> std::slice::Iter<u8> {
2019-09-30T01:17:52.1939872Z          self.0.iter()
2019-09-30T01:17:52.1939964Z  
2019-09-30T01:17:52.1940170Z      fn iter(&self) -> std::slice::Iter<u8> {
2019-09-30T01:17:52.1940213Z          self.0.iter()
2019-09-30T01:17:52.1940251Z      }
2019-09-30T01:17:52.1940251Z      }
2019-09-30T01:17:52.1940306Z  }
2019-09-30T01:17:52.1940342Z  
2019-09-30T01:17:52.1940379Z  #[warn(
2019-09-30T01:17:52.1940436Z      clippy::needless_range_loop,
2019-09-30T01:17:52.1940478Z      clippy::explicit_iter_loop,
2019-09-30T01:17:52.1940529Z      clippy::explicit_into_iter_loop,
2019-09-30T01:17:52.1940570Z      clippy::iter_next_loop,
2019-09-30T01:17:52.1940627Z      clippy::reverse_range_loop,
2019-09-30T01:17:52.1940668Z      clippy::for_kv_map
2019-09-30T01:17:52.1940706Z  )]
2019-09-30T01:17:52.1940766Z  #[allow(
2019-09-30T01:17:52.1940805Z      clippy::linkedlist,
2019-09-30T01:17:52.1940846Z      clippy::shadow_unrelated,
2019-09-30T01:17:52.1940899Z      clippy::unnecessary_mut_passed,
2019-09-30T01:17:52.1940942Z      clippy::cognitive_complexity,
2019-09-30T01:17:52.1940983Z      clippy::similar_names
2019-09-30T01:17:52.1941021Z  )]
2019-09-30T01:17:52.1941083Z  #[allow(clippy::many_single_char_names, unused_variables, clippy::into_iter_on_array)]
2019-09-30T01:17:52.1941169Z      const MAX_LEN: usize = 42;
2019-09-30T01:17:52.1941226Z      let mut vec = vec![1, 2, 3, 4];
2019-09-30T01:17:52.1941265Z  
2019-09-30T01:17:52.1941265Z  
2019-09-30T01:17:52.1941467Z -    for i in (0..10).rev() {
2019-09-30T01:17:52.1941576Z          println!("{}", i);
2019-09-30T01:17:52.1941614Z      }
2019-09-30T01:17:52.1941650Z  
2019-09-30T01:17:52.1941650Z  
2019-09-30T01:17:52.1941860Z -    for i in (0..=10).rev() {
2019-09-30T01:17:52.1941904Z +    for i in 10..=0 {
2019-09-30T01:17:52.1942092Z      }
2019-09-30T01:17:52.1942128Z  
2019-09-30T01:17:52.1942128Z  
2019-09-30T01:17:52.1942782Z -    for i in (0..MAX_LEN).rev() {
2019-09-30T01:17:52.1942838Z +    for i in MAX_LEN..0 {
2019-09-30T01:17:52.1942940Z      }
2019-09-30T01:17:52.1942977Z  
2019-09-30T01:17:52.1943033Z      for i in 5..=5 {
2019-09-30T01:17:52.1943283Z          // not an error, this is the range with only one element “5”
---
2019-09-30T01:17:52.1944077Z          // not an error
2019-09-30T01:17:52.1944129Z          println!("{}", i);
2019-09-30T01:17:52.1944169Z      }
2019-09-30T01:17:52.1944222Z  
2019-09-30T01:17:52.1944264Z      for i in (10..0).map(|x| x * 2) {
2019-09-30T01:17:52.1944515Z          // not an error, it can't be known what arbitrary methods do to a range
2019-09-30T01:17:52.1944621Z      }
2019-09-30T01:17:52.1944659Z  
2019-09-30T01:17:52.1944720Z      // testing that the empty range lint folds constants
2019-09-30T01:17:52.1944720Z      // testing that the empty range lint folds constants
2019-09-30T01:17:52.1944928Z -    for i in (5 + 4..10).rev() {
2019-09-30T01:17:52.1944973Z +    for i in 10..5 + 4 {
2019-09-30T01:17:52.1945081Z      }
2019-09-30T01:17:52.1945118Z  
2019-09-30T01:17:52.1945118Z  
2019-09-30T01:17:52.1945328Z -    for i in ((3 - 1)..(5 + 2)).rev() {
2019-09-30T01:17:52.1945551Z +    for i in (5 + 2)..(3 - 1) {
2019-09-30T01:17:52.1945637Z      }
2019-09-30T01:17:52.1945692Z  
2019-09-30T01:17:52.1945692Z  
2019-09-30T01:17:52.1945742Z      for i in (2 * 2)..(2 * 3) {
2019-09-30T01:17:52.1945897Z          // no error, 4..6 is fine
2019-09-30T01:17:52.1945996Z      }
2019-09-30T01:17:52.1946031Z  
2019-09-30T01:17:52.1946069Z      let x = 42;
2019-09-30T01:17:52.1946126Z      for i in x..10 {
2019-09-30T01:17:52.1946126Z      for i in x..10 {
2019-09-30T01:17:52.1946334Z          // no error, not constant-foldable
2019-09-30T01:17:52.1946378Z          println!("{}", i);
2019-09-30T01:17:52.1946417Z      }
2019-09-30T01:17:52.1946470Z  
2019-09-30T01:17:52.1946507Z      // See #601
2019-09-30T01:17:52.1946546Z      for i in 0..10 {
2019-09-30T01:17:52.1946607Z          // no error, id_col does not exist outside the loop
2019-09-30T01:17:52.1946660Z          let mut id_col = vec![0f64; 10];
2019-09-30T01:17:52.1946702Z          id_col[i] = 1f64;
2019-09-30T01:17:52.1946793Z  
2019-09-30T01:17:52.1946793Z  
2019-09-30T01:17:52.1946978Z -    for _v in &vec {}
2019-09-30T01:17:52.1947021Z +    for _v in vec.iter() {}
2019-09-30T01:17:52.1947085Z  
2019-09-30T01:17:52.1947269Z -    for _v in &mut vec {}
2019-09-30T01:17:52.1947312Z +    for _v in vec.iter_mut() {}
2019-09-30T01:17:52.1947406Z      let out_vec = vec![1, 2, 3];
2019-09-30T01:17:52.1947406Z      let out_vec = vec![1, 2, 3];
2019-09-30T01:17:52.1947591Z -    for _v in out_vec {}
2019-09-30T01:17:52.1947650Z +    for _v in out_vec.into_iter() {}
2019-09-30T01:17:52.1947689Z  
2019-09-30T01:17:52.1947727Z      let array = [1, 2, 3];
2019-09-30T01:17:52.1947910Z -    for _v in &array {}
2019-09-30T01:17:52.1947970Z +    for _v in array.into_iter() {}
2019-09-30T01:17:52.1948008Z  
2019-09-30T01:17:52.1948049Z      for _v in &vec {} // these are fine
2019-09-30T01:17:52.1948106Z      for _v in &mut vec {} // these are fine
2019-09-30T01:17:52.1948153Z  
2019-09-30T01:17:52.1948344Z -    for _v in &[1, 2, 3] {}
2019-09-30T01:17:52.1948388Z +    for _v in [1, 2, 3].iter() {}
2019-09-30T01:17:52.1948445Z  
2019-09-30T01:17:52.1948486Z      for _v in (&mut [1, 2, 3]).iter() {} // no error
2019-09-30T01:17:52.1948609Z  
2019-09-30T01:17:52.1948850Z -    for _v in &[0; 32] {}
2019-09-30T01:17:52.1948895Z +    for _v in [0; 32].iter() {}
2019-09-30T01:17:52.1948933Z  
2019-09-30T01:17:52.1948992Z      for _v in [0; 33].iter() {} // no error
2019-09-30T01:17:52.1949030Z  
2019-09-30T01:17:52.1949071Z      let ll: LinkedList<()> = LinkedList::new();
2019-09-30T01:17:52.1949254Z -    for _v in &ll {}
2019-09-30T01:17:52.1949315Z +    for _v in ll.iter() {}
2019-09-30T01:17:52.1949352Z  
2019-09-30T01:17:52.1949392Z      let vd: VecDeque<()> = VecDeque::new();
2019-09-30T01:17:52.1949593Z -    for _v in &vd {}
2019-09-30T01:17:52.1949636Z +    for _v in vd.iter() {}
2019-09-30T01:17:52.1949673Z  
2019-09-30T01:17:52.1949811Z      let bh: BinaryHeap<()> = BinaryHeap::new();
2019-09-30T01:17:52.1950019Z -    for _v in &bh {}
2019-09-30T01:17:52.1950061Z +    for _v in bh.iter() {}
2019-09-30T01:17:52.1950098Z  
2019-09-30T01:17:52.1950156Z      let hm: HashMap<(), ()> = HashMap::new();
2019-09-30T01:17:52.1950346Z -    for _v in &hm {}
2019-09-30T01:17:52.1950388Z +    for _v in hm.iter() {}
2019-09-30T01:17:52.1950442Z  
2019-09-30T01:17:52.1950482Z      let bt: BTreeMap<(), ()> = BTreeMap::new();
2019-09-30T01:17:52.1950665Z -    for _v in &bt {}
2019-09-30T01:17:52.1950707Z +    for _v in bt.iter() {}
2019-09-30T01:17:52.1950762Z  
2019-09-30T01:17:52.1950802Z      let hs: HashSet<()> = HashSet::new();
2019-09-30T01:17:52.1950986Z -    for _v in &hs {}
2019-09-30T01:17:52.1951045Z +    for _v in hs.iter() {}
2019-09-30T01:17:52.1951082Z  
2019-09-30T01:17:52.1951122Z      let bs: BTreeSet<()> = BTreeSet::new();
2019-09-30T01:17:52.1951324Z -    for _v in &bs {}
2019-09-30T01:17:52.1951366Z +    for _v in bs.iter() {}
2019-09-30T01:17:52.1951412Z  
2019-09-30T01:17:52.1951450Z      let u = Unrelated(vec![]);
2019-09-30T01:17:52.1951510Z      for _v in u.next() {} // no error
2019-09-30T01:17:52.1951552Z      for _v in u.iter() {} // no error
2019-09-30T01:17:52.1951653Z      let mut out = vec![];
2019-09-30T01:17:52.1951653Z      let mut out = vec![];
2019-09-30T01:17:52.1951853Z      vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();
2019-09-30T01:17:52.1951906Z      let _y = vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>(); // this is fine
2019-09-30T01:17:52.1952007Z      // Loop with explicit counter variable
2019-09-30T01:17:52.1952046Z  
2019-09-30T01:17:52.1952085Z      // Potential false positives
2019-09-30T01:17:52.1952144Z      let mut _index = 0;
2019-09-30T01:17:52.1952144Z      let mut _index = 0;
2019-09-30T01:17:52.1952183Z      _index = 1;
2019-09-30T01:17:52.1952224Z      for _v in &vec {
2019-09-30T01:17:52.1952282Z          _index += 1
2019-09-30T01:17:52.1952355Z  
2019-09-30T01:17:52.1952417Z      let mut _index = 0;
2019-09-30T01:17:52.1952836Z      _index += 1;
2019-09-30T01:17:52.1952836Z      _index += 1;
2019-09-30T01:17:52.1952882Z      for _v in &vec {
2019-09-30T01:17:52.1952923Z          _index += 1
2019-09-30T01:17:52.1953019Z  
2019-09-30T01:17:52.1953059Z      let mut _index = 0;
2019-09-30T01:17:52.1953124Z      if true {
2019-09-30T01:17:52.1953165Z          _index = 1
2019-09-30T01:17:52.1953165Z          _index = 1
2019-09-30T01:17:52.1953205Z      }
2019-09-30T01:17:52.1953245Z      for _v in &vec {
2019-09-30T01:17:52.1953304Z          _index += 1
2019-09-30T01:17:52.1953381Z  
2019-09-30T01:17:52.1953437Z      let mut _index = 0;
2019-09-30T01:17:52.1953478Z      let mut _index = 1;
2019-09-30T01:17:52.1953478Z      let mut _index = 1;
2019-09-30T01:17:52.1953519Z      for _v in &vec {
2019-09-30T01:17:52.1953560Z          _index += 1
2019-09-30T01:17:52.1953652Z  
2019-09-30T01:17:52.1953691Z      let mut _index = 0;
2019-09-30T01:17:52.1953691Z      let mut _index = 0;
2019-09-30T01:17:52.1953749Z      for _v in &vec {
2019-09-30T01:17:52.1953790Z          _index += 1;
2019-09-30T01:17:52.1953837Z          _index += 1
2019-09-30T01:17:52.1953931Z  
2019-09-30T01:17:52.1953970Z      let mut _index = 0;
2019-09-30T01:17:52.1953970Z      let mut _index = 0;
2019-09-30T01:17:52.1954010Z      for _v in &vec {
2019-09-30T01:17:52.1954067Z          _index *= 2;
2019-09-30T01:17:52.1954242Z          _index += 1
2019-09-30T01:17:52.1954347Z  
2019-09-30T01:17:52.1954387Z      let mut _index = 0;
2019-09-30T01:17:52.1954387Z      let mut _index = 0;
2019-09-30T01:17:52.1954428Z      for _v in &vec {
2019-09-30T01:17:52.1954468Z          _index = 1;
2019-09-30T01:17:52.1954525Z          _index += 1
2019-09-30T01:17:52.1954601Z  
2019-09-30T01:17:52.1954657Z      let mut _index = 0;
2019-09-30T01:17:52.1954695Z  
2019-09-30T01:17:52.1954695Z  
2019-09-30T01:17:52.1954735Z      for _v in &vec {
2019-09-30T01:17:52.1954776Z          let mut _index = 0;
2019-09-30T01:17:52.1954836Z          _index += 1
2019-09-30T01:17:52.1954911Z  
2019-09-30T01:17:52.1954968Z      let mut _index = 0;
2019-09-30T01:17:52.1954968Z      let mut _index = 0;
2019-09-30T01:17:52.1955081Z      for _v in &vec {
2019-09-30T01:17:52.1955121Z          _index += 1;
2019-09-30T01:17:52.1955179Z          _index = 0;
2019-09-30T01:17:52.1955256Z  
2019-09-30T01:17:52.1955295Z      let mut _index = 0;
2019-09-30T01:17:52.1955295Z      let mut _index = 0;
2019-09-30T01:17:52.1955359Z      for _v in &vec {
2019-09-30T01:17:52.1955401Z          for _x in 0..1 {
2019-09-30T01:17:52.1955443Z              _index += 1;
2019-09-30T01:17:52.1955541Z          _index += 1
2019-09-30T01:17:52.1955581Z      }
2019-09-30T01:17:52.1955618Z  
2019-09-30T01:17:52.1955673Z      let mut _index = 0;
---
2019-09-30T01:17:52.1955985Z      let mut _index = 0;
2019-09-30T01:17:52.1956026Z      if true {
2019-09-30T01:17:52.1956180Z          _index = 1
2019-09-30T01:17:52.1956237Z      };
2019-09-30T01:17:52.1956276Z      for _v in &vec {
2019-09-30T01:17:52.1956315Z          _index += 1
2019-09-30T01:17:52.1956406Z  
2019-09-30T01:17:52.1956443Z      let mut _index = 1;
2019-09-30T01:17:52.1956488Z      if false {
2019-09-30T01:17:52.1956544Z          _index = 0
2019-09-30T01:17:52.1956544Z          _index = 0
2019-09-30T01:17:52.1956582Z      };
2019-09-30T01:17:52.1956621Z      for _v in &vec {
2019-09-30T01:17:52.1956674Z          _index += 1
2019-09-30T01:17:52.1956747Z  
2019-09-30T01:17:52.1956785Z      let mut index = 0;
2019-09-30T01:17:52.1956838Z      {
2019-09-30T01:17:52.1956877Z          let mut _x = &mut index;
2019-09-30T01:17:52.1956877Z          let mut _x = &mut index;
2019-09-30T01:17:52.1956916Z      }
2019-09-30T01:17:52.1956968Z      for _v in &vec {
2019-09-30T01:17:52.1957007Z          _index += 1
2019-09-30T01:17:52.1957079Z  
2019-09-30T01:17:52.1957130Z      let mut index = 0;
2019-09-30T01:17:52.1957130Z      let mut index = 0;
2019-09-30T01:17:52.1957176Z      for _v in &vec {
2019-09-30T01:17:52.1957214Z          index += 1
2019-09-30T01:17:52.1957266Z      }
2019-09-30T01:17:52.1957306Z      println!("index: {}", index);
2019-09-30T01:17:52.1957344Z  
2019-09-30T01:17:52.1957647Z      fn f<T>(_: &T, _: &T) -> bool {
2019-09-30T01:17:52.1957758Z      }
2019-09-30T01:17:52.1957758Z      }
2019-09-30T01:17:52.1957800Z      fn g<T>(_: &mut [T], _: usize, _: usize) {
2019-09-30T01:17:52.1957899Z      }
2019-09-30T01:17:52.1957899Z      }
2019-09-30T01:17:52.1957938Z      for i in 1..vec.len() {
2019-09-30T01:17:52.1958161Z          if f(&vec[i - 1], &vec[i]) {
2019-09-30T01:17:52.1958362Z              g(&mut vec, i - 1, i);
2019-09-30T01:17:52.1958442Z      }
2019-09-30T01:17:52.1958497Z  
2019-09-30T01:17:52.1958497Z  
2019-09-30T01:17:52.1958536Z      for mid in 1..vec.len() {
2019-09-30T01:17:52.1958578Z          let (_, _) = vec.split_at(mid);
2019-09-30T01:17:52.1958680Z  }
2019-09-30T01:17:52.1958716Z  
2019-09-30T01:17:52.1958716Z  
2019-09-30T01:17:52.1958939Z  fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
2019-09-30T01:17:52.1959153Z      let pivot = v.len() - 1;
2019-09-30T01:17:52.1959196Z      let mut i = 0;
2019-09-30T01:17:52.1959236Z      for j in 0..pivot {
2019-09-30T01:17:52.1959382Z          if v[j] <= v[pivot] {
2019-09-30T01:17:52.1959432Z              v.swap(i, j);
2019-09-30T01:17:52.1959473Z              i += 1;
2019-09-30T01:17:52.1959566Z      }
2019-09-30T01:17:52.1959566Z      }
2019-09-30T01:17:52.1959603Z      v.swap(i, pivot);
2019-09-30T01:17:52.1959694Z  }
2019-09-30T01:17:52.1959729Z  
2019-09-30T01:17:52.1959768Z  #[warn(clippy::needless_range_loop)]
2019-09-30T01:17:52.1959768Z  #[warn(clippy::needless_range_loop)]
2019-09-30T01:17:52.1959831Z  pub fn manual_copy_same_destination(dst: &mut [i32], d: usize, s: usize) {
2019-09-30T01:17:52.1960083Z      // Same source and destination - don't trigger lint
2019-09-30T01:17:52.1960129Z      for i in 0..dst.len() {
2019-09-30T01:17:52.1960295Z          dst[d + i] = dst[s + i];
2019-09-30T01:17:52.1960370Z  }
2019-09-30T01:17:52.1960406Z  
2019-09-30T01:17:52.1960460Z  mod issue_2496 {
2019-09-30T01:17:52.1960500Z      pub trait Handle {
2019-09-30T01:17:52.1960500Z      pub trait Handle {
2019-09-30T01:17:52.1960747Z          fn new_for_index(index: usize) -> Self;
2019-09-30T01:17:52.1960964Z          fn index(&self) -> usize;
2019-09-30T01:17:52.1961042Z  
2019-09-30T01:17:52.1961042Z  
2019-09-30T01:17:52.1961237Z      pub fn test<H: Handle>() -> H {
2019-09-30T01:17:52.1961298Z          for x in 0..5 {
2019-09-30T01:17:52.1961341Z              let next_handle = H::new_for_index(x);
2019-09-30T01:17:52.1961387Z              println!("{}", next_handle.index());
2019-09-30T01:17:52.1961485Z          unimplemented!()
2019-09-30T01:17:52.1961523Z      }
2019-09-30T01:17:52.1961575Z  }
2019-09-30T01:17:52.1961611Z  
2019-09-30T01:17:52.1961611Z  
2019-09-30T01:17:52.1961637Z 
2019-09-30T01:17:52.1961679Z The actual fixed differed from the expected fixed.
2019-09-30T01:17:52.1962051Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/for_loop_fixable.fixed
2019-09-30T01:17:52.1962810Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'for_loop_fixable.rs'
2019-09-30T01:17:52.1962883Z 
2019-09-30T01:17:52.1962926Z error: 2 errors occurred comparing output.
2019-09-30T01:17:52.1962970Z status: exit code: 0
2019-09-30T01:17:52.1962970Z status: exit code: 0
2019-09-30T01:17:52.1964464Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/for_loop_fixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/for_loop_fixable.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/for_loop_fixable.stage-id.aux" "-A" "unused"
2019-09-30T01:17:52.1964894Z ------------------------------------------
2019-09-30T01:17:52.1964946Z 
2019-09-30T01:17:52.1965160Z ------------------------------------------
2019-09-30T01:17:52.1965204Z stderr:
---
2019-09-30T01:17:52.1966367Z expected stderr:
2019-09-30T01:17:52.1966587Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1966794Z   --> $DIR/manual_memcpy.rs:7:14
2019-09-30T01:17:52.1966857Z    |
2019-09-30T01:17:52.1966898Z LL |     for i in 0..src.len() {
2019-09-30T01:17:52.1966990Z    |              ^^^^^^^^^^^^ help: try replacing the loop by: `dst[..src.len()].clone_from_slice(&src[..])`
2019-09-30T01:17:52.1967161Z    |
2019-09-30T01:17:52.1967425Z    = note: `-D clippy::manual-memcpy` implied by `-D warnings`
2019-09-30T01:17:52.1967670Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1967899Z   --> $DIR/manual_memcpy.rs:12:14
2019-09-30T01:17:52.1967942Z    |
2019-09-30T01:17:52.1967942Z    |
2019-09-30T01:17:52.1967982Z LL |     for i in 0..src.len() {
2019-09-30T01:17:52.1968054Z    |              ^^^^^^^^^^^^ help: try replacing the loop by: `dst[10..(src.len() + 10)].clone_from_slice(&src[..])`
2019-09-30T01:17:52.1968309Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1968526Z   --> $DIR/manual_memcpy.rs:17:14
2019-09-30T01:17:52.1968568Z    |
2019-09-30T01:17:52.1968568Z    |
2019-09-30T01:17:52.1968608Z LL |     for i in 0..src.len() {
2019-09-30T01:17:52.1968661Z    |              ^^^^^^^^^^^^ help: try replacing the loop by: `dst[..src.len()].clone_from_slice(&src[10..])`
2019-09-30T01:17:52.1968936Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1969134Z   --> $DIR/manual_memcpy.rs:22:14
2019-09-30T01:17:52.1969194Z    |
2019-09-30T01:17:52.1969194Z    |
2019-09-30T01:17:52.1969235Z LL |     for i in 11..src.len() {
2019-09-30T01:17:52.1969537Z    |              ^^^^^^^^^^^^^ help: try replacing the loop by: `dst[11..src.len()].clone_from_slice(&src[(11 - 10)..(src.len() - 10)])`
2019-09-30T01:17:52.1969811Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1970009Z   --> $DIR/manual_memcpy.rs:27:14
2019-09-30T01:17:52.1970050Z    |
2019-09-30T01:17:52.1970050Z    |
2019-09-30T01:17:52.1970108Z LL |     for i in 0..dst.len() {
2019-09-30T01:17:52.1970159Z    |              ^^^^^^^^^^^^ help: try replacing the loop by: `dst.clone_from_slice(&src[..dst.len()])`
2019-09-30T01:17:52.1970410Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1970626Z   --> $DIR/manual_memcpy.rs:40:14
2019-09-30T01:17:52.1970676Z    |
2019-09-30T01:17:52.1970716Z LL |     for i in 10..256 {
2019-09-30T01:17:52.1970716Z LL |     for i in 10..256 {
2019-09-30T01:17:52.1970775Z    |              ^^^^^^^
2019-09-30T01:17:52.1970816Z help: try replacing the loop by
2019-09-30T01:17:52.1970855Z    |
2019-09-30T01:17:52.1971116Z LL |     for i in dst[10..256].clone_from_slice(&src[(10 - 5)..(256 - 5)])
2019-09-30T01:17:52.1971171Z LL |     dst2[(10 + 500)..(256 + 500)].clone_from_slice(&src[10..256]) {
2019-09-30T01:17:52.1971238Z 
2019-09-30T01:17:52.1971471Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1971670Z   --> $DIR/manual_memcpy.rs:52:14
2019-09-30T01:17:52.1971711Z    |
2019-09-30T01:17:52.1971711Z    |
2019-09-30T01:17:52.1971773Z LL |     for i in 10..LOOP_OFFSET {
2019-09-30T01:17:52.1972112Z    |              ^^^^^^^^^^^^^^^ help: try replacing the loop by: `dst[(10 + LOOP_OFFSET)..(LOOP_OFFSET + LOOP_OFFSET)].clone_from_slice(&src[(10 - some_var)..(LOOP_OFFSET - some_var)])`
2019-09-30T01:17:52.1972396Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1977452Z   --> $DIR/manual_memcpy.rs:65:14
2019-09-30T01:17:52.1977517Z    |
2019-09-30T01:17:52.1977517Z    |
2019-09-30T01:17:52.1977558Z LL |     for i in 0..src_vec.len() {
2019-09-30T01:17:52.1977798Z    |              ^^^^^^^^^^^^^^^^ help: try replacing the loop by: `dst_vec[..src_vec.len()].clone_from_slice(&src_vec[..])`
2019-09-30T01:17:52.1978114Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1978339Z   --> $DIR/manual_memcpy.rs:94:14
2019-09-30T01:17:52.1978382Z    |
2019-09-30T01:17:52.1978382Z    |
2019-09-30T01:17:52.1978425Z LL |     for i in from..from + src.len() {
2019-09-30T01:17:52.1979028Z    |              ^^^^^^^^^^^^^^^^^^^^^^ help: try replacing the loop by: `dst[from..from + src.len()].clone_from_slice(&src[0..(from + src.len() - from)])`
2019-09-30T01:17:52.1979303Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1983066Z   --> $DIR/manual_memcpy.rs:98:14
2019-09-30T01:17:52.1983324Z    |
2019-09-30T01:17:52.1983324Z    |
2019-09-30T01:17:52.1983366Z LL |     for i in from..from + 3 {
2019-09-30T01:17:52.1983738Z    |              ^^^^^^^^^^^^^^ help: try replacing the loop by: `dst[from..from + 3].clone_from_slice(&src[0..(from + 3 - from)])`
2019-09-30T01:17:52.1984033Z error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1984241Z   --> $DIR/manual_memcpy.rs:105:14
2019-09-30T01:17:52.1984301Z    |
2019-09-30T01:17:52.1984301Z    |
2019-09-30T01:17:52.1984343Z LL |     for i in 0..src.len() {
2019-09-30T01:17:52.1984396Z    |              ^^^^^^^^^^^^ help: try replacing the loop by: `dst[..src.len()].clone_from_slice(&src[..])`
2019-09-30T01:17:52.1984489Z error: aborting due to 11 previous errors
2019-09-30T01:17:52.1984517Z 
2019-09-30T01:17:52.1984542Z 
2019-09-30T01:17:52.1984567Z 
2019-09-30T01:17:52.1984567Z 
2019-09-30T01:17:52.1984606Z diff of stderr:
2019-09-30T01:17:52.1984649Z 
2019-09-30T01:17:52.1984877Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1985092Z -  --> $DIR/manual_memcpy.rs:7:14
2019-09-30T01:17:52.1985288Z -   |
2019-09-30T01:17:52.1985493Z -LL |     for i in 0..src.len() {
2019-09-30T01:17:52.1985768Z -   |              ^^^^^^^^^^^^ help: try replacing the loop by: `dst[..src.len()].clone_from_slice(&src[..])`
2019-09-30T01:17:52.1985973Z -   |
2019-09-30T01:17:52.1986314Z -   = note: `-D clippy::manual-memcpy` implied by `-D warnings`
2019-09-30T01:17:52.1986701Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1986919Z -  --> $DIR/manual_memcpy.rs:12:14
2019-09-30T01:17:52.1987089Z -   |
2019-09-30T01:17:52.1987089Z -   |
2019-09-30T01:17:52.1987283Z -LL |     for i in 0..src.len() {
2019-09-30T01:17:52.1987574Z -   |              ^^^^^^^^^^^^ help: try replacing the loop by: `dst[10..(src.len() + 10)].clone_from_slice(&src[..])`
2019-09-30T01:17:52.1987966Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1988181Z -  --> $DIR/manual_memcpy.rs:17:14
2019-09-30T01:17:52.1988363Z -   |
2019-09-30T01:17:52.1988363Z -   |
2019-09-30T01:17:52.1988557Z -LL |     for i in 0..src.len() {
2019-09-30T01:17:52.1988825Z -   |              ^^^^^^^^^^^^ help: try replacing the loop by: `dst[..src.len()].clone_from_slice(&src[10..])`
2019-09-30T01:17:52.1989241Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1989441Z -  --> $DIR/manual_memcpy.rs:22:14
2019-09-30T01:17:52.1989630Z -   |
2019-09-30T01:17:52.1989630Z -   |
2019-09-30T01:17:52.1989822Z -LL |     for i in 11..src.len() {
2019-09-30T01:17:52.1990115Z -   |              ^^^^^^^^^^^^^ help: try replacing the loop by: `dst[11..src.len()].clone_from_slice(&src[(11 - 10)..(src.len() - 10)])`
2019-09-30T01:17:52.1990529Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1990727Z -  --> $DIR/manual_memcpy.rs:27:14
2019-09-30T01:17:52.1990912Z -   |
2019-09-30T01:17:52.1990912Z -   |
2019-09-30T01:17:52.1991108Z -LL |     for i in 0..dst.len() {
2019-09-30T01:17:52.1991379Z -   |              ^^^^^^^^^^^^ help: try replacing the loop by: `dst.clone_from_slice(&src[..dst.len()])`
2019-09-30T01:17:52.1992098Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1992311Z -  --> $DIR/manual_memcpy.rs:40:14
2019-09-30T01:17:52.1993122Z -   |
2019-09-30T01:17:52.1993419Z -LL |     for i in 10..256 {
2019-09-30T01:17:52.1993419Z -LL |     for i in 10..256 {
2019-09-30T01:17:52.1993616Z -   |              ^^^^^^^
2019-09-30T01:17:52.1993817Z -help: try replacing the loop by
2019-09-30T01:17:52.1994012Z -   |
2019-09-30T01:17:52.1994258Z -LL |     for i in dst[10..256].clone_from_slice(&src[(10 - 5)..(256 - 5)])
2019-09-30T01:17:52.1994511Z -LL |     dst2[(10 + 500)..(256 + 500)].clone_from_slice(&src[10..256]) {
2019-09-30T01:17:52.1994875Z -
2019-09-30T01:17:52.1995099Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1995306Z -  --> $DIR/manual_memcpy.rs:52:14
2019-09-30T01:17:52.1995499Z -   |
2019-09-30T01:17:52.1995499Z -   |
2019-09-30T01:17:52.1995834Z -LL |     for i in 10..LOOP_OFFSET {
2019-09-30T01:17:52.1996187Z -   |              ^^^^^^^^^^^^^^^ help: try replacing the loop by: `dst[(10 + LOOP_OFFSET)..(LOOP_OFFSET + LOOP_OFFSET)].clone_from_slice(&src[(10 - some_var)..(LOOP_OFFSET - some_var)])`
2019-09-30T01:17:52.1996635Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1996949Z -  --> $DIR/manual_memcpy.rs:65:14
2019-09-30T01:17:52.1997139Z -   |
2019-09-30T01:17:52.1997139Z -   |
2019-09-30T01:17:52.1997336Z -LL |     for i in 0..src_vec.len() {
2019-09-30T01:17:52.1997614Z -   |              ^^^^^^^^^^^^^^^^ help: try replacing the loop by: `dst_vec[..src_vec.len()].clone_from_slice(&src_vec[..])`
2019-09-30T01:17:52.1998023Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1998223Z -  --> $DIR/manual_memcpy.rs:94:14
2019-09-30T01:17:52.1998410Z -   |
2019-09-30T01:17:52.1998410Z -   |
2019-09-30T01:17:52.1998616Z -LL |     for i in from..from + src.len() {
2019-09-30T01:17:52.1998930Z -   |              ^^^^^^^^^^^^^^^^^^^^^^ help: try replacing the loop by: `dst[from..from + src.len()].clone_from_slice(&src[0..(from + src.len() - from)])`
2019-09-30T01:17:52.1999343Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.1999550Z -  --> $DIR/manual_memcpy.rs:98:14
2019-09-30T01:17:52.1999720Z -   |
2019-09-30T01:17:52.1999720Z -   |
2019-09-30T01:17:52.1999935Z -LL |     for i in from..from + 3 {
2019-09-30T01:17:52.2000222Z -   |              ^^^^^^^^^^^^^^ help: try replacing the loop by: `dst[from..from + 3].clone_from_slice(&src[0..(from + 3 - from)])`
2019-09-30T01:17:52.2000631Z -error: it looks like you're manually copying between slices
2019-09-30T01:17:52.2000831Z -  --> $DIR/manual_memcpy.rs:105:14
2019-09-30T01:17:52.2001002Z -   |
2019-09-30T01:17:52.2001002Z -   |
2019-09-30T01:17:52.2001214Z -LL |     for i in 0..src.len() {
2019-09-30T01:17:52.2001478Z -   |              ^^^^^^^^^^^^ help: try replacing the loop by: `dst[..src.len()].clone_from_slice(&src[..])`
2019-09-30T01:17:52.2001881Z -error: aborting due to 11 previous errors
2019-09-30T01:17:52.2002050Z -
2019-09-30T01:17:52.2002210Z -
2019-09-30T01:17:52.2002239Z 
2019-09-30T01:17:52.2002239Z 
2019-09-30T01:17:52.2002301Z The actual stderr differed from the expected stderr.
2019-09-30T01:17:52.2003025Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/manual_memcpy.stderr
2019-09-30T01:17:52.2003095Z To update references, run this command from build directory:
2019-09-30T01:17:52.2003481Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'manual_memcpy.rs'
2019-09-30T01:17:52.2003523Z 
2019-09-30T01:17:52.2003566Z error: 1 errors occurred comparing output.
2019-09-30T01:17:52.2003625Z status: exit code: 0
2019-09-30T01:17:52.2005149Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/manual_memcpy.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/manual_memcpy.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/manual_memcpy.stage-id.aux" "-A" "unused"
2019-09-30T01:17:52.2005697Z ------------------------------------------
2019-09-30T01:17:52.2005732Z 
2019-09-30T01:17:52.2006055Z ------------------------------------------
2019-09-30T01:17:52.2006097Z stderr:
---
2019-09-30T01:17:52.2006859Z normalized stderr:
2019-09-30T01:17:52.2006885Z 
2019-09-30T01:17:52.2006927Z 
2019-09-30T01:17:52.2006966Z expected stderr:
2019-09-30T01:17:52.2007014Z error: attempt to mutate range bound within loop; note that the range of the loop is unchanged
2019-09-30T01:17:52.2007299Z    |
2019-09-30T01:17:52.2007339Z LL |         m = 5;
2019-09-30T01:17:52.2007379Z    |         ^^^^^
2019-09-30T01:17:52.2007434Z    |
2019-09-30T01:17:52.2007434Z    |
2019-09-30T01:17:52.2007666Z    = note: `-D clippy::mut-range-bound` implied by `-D warnings`
2019-09-30T01:17:52.2007699Z 
2019-09-30T01:17:52.2007763Z error: attempt to mutate range bound within loop; note that the range of the loop is unchanged
2019-09-30T01:17:52.2008012Z    |
2019-09-30T01:17:52.2008051Z LL |         m *= 2;
2019-09-30T01:17:52.2008108Z    |         ^^^^^^
2019-09-30T01:17:52.2008134Z 
2019-09-30T01:17:52.2008134Z 
2019-09-30T01:17:52.2008180Z error: attempt to mutate range bound within loop; note that the range of the loop is unchanged
2019-09-30T01:17:52.2008446Z    |
2019-09-30T01:17:52.2008484Z LL |         m = 5;
2019-09-30T01:17:52.2008532Z    |         ^^^^^
2019-09-30T01:17:52.2008574Z 
2019-09-30T01:17:52.2008574Z 
2019-09-30T01:17:52.2008620Z error: attempt to mutate range bound within loop; note that the range of the loop is unchanged
2019-09-30T01:17:52.2008875Z    |
2019-09-30T01:17:52.2008922Z LL |         n = 7;
2019-09-30T01:17:52.2008962Z    |         ^^^^^
2019-09-30T01:17:52.2008988Z 
2019-09-30T01:17:52.2008988Z 
2019-09-30T01:17:52.2009049Z error: attempt to mutate range bound within loop; note that the range of the loop is unchanged
2019-09-30T01:17:52.2009297Z    |
2019-09-30T01:17:52.2009297Z    |
2019-09-30T01:17:52.2009356Z LL |         let n = &mut m; // warning
2019-09-30T01:17:52.2009426Z 
2019-09-30T01:17:52.2009466Z error: aborting due to 5 previous errors
2019-09-30T01:17:52.2009510Z 
2019-09-30T01:17:52.2009533Z 
2019-09-30T01:17:52.2009533Z 
2019-09-30T01:17:52.2009557Z 
2019-09-30T01:17:52.2009595Z diff of stderr:
2019-09-30T01:17:52.2009621Z 
2019-09-30T01:17:52.2009903Z -error: attempt to mutate range bound within loop; note that the range of the loop is unchanged
2019-09-30T01:17:52.2010275Z -   |
2019-09-30T01:17:52.2010476Z -LL |         m = 5;
2019-09-30T01:17:52.2010658Z -   |         ^^^^^
2019-09-30T01:17:52.2010910Z -   |
2019-09-30T01:17:52.2010910Z -   |
2019-09-30T01:17:52.2011167Z -   = note: `-D clippy::mut-range-bound` implied by `-D warnings`
2019-09-30T01:17:52.2011358Z -
2019-09-30T01:17:52.2011606Z -error: attempt to mutate range bound within loop; note that the range of the loop is unchanged
2019-09-30T01:17:52.2011994Z -   |
2019-09-30T01:17:52.2012174Z -LL |         m *= 2;
2019-09-30T01:17:52.2012355Z -   |         ^^^^^^
2019-09-30T01:17:52.2012899Z -
2019-09-30T01:17:52.2012899Z -
2019-09-30T01:17:52.2013497Z -error: attempt to mutate range bound within loop; note that the range of the loop is unchanged
2019-09-30T01:17:52.2014073Z -   |
2019-09-30T01:17:52.2014266Z -LL |         m = 5;
2019-09-30T01:17:52.2014451Z -   |         ^^^^^
2019-09-30T01:17:52.2014619Z -
2019-09-30T01:17:52.2014619Z -
2019-09-30T01:17:52.2014899Z -error: attempt to mutate range bound within loop; note that the range of the loop is unchanged
2019-09-30T01:17:52.2015294Z -   |
2019-09-30T01:17:52.2015500Z -LL |         n = 7;
2019-09-30T01:17:52.2015689Z -   |         ^^^^^
2019-09-30T01:17:52.2015964Z -
2019-09-30T01:17:52.2015964Z -
2019-09-30T01:17:52.2016213Z -error: attempt to mutate range bound within loop; note that the range of the loop is unchanged
2019-09-30T01:17:52.2016608Z -   |
2019-09-30T01:17:52.2016608Z -   |
2019-09-30T01:17:52.2016810Z -LL |         let n = &mut m; // warning
2019-09-30T01:17:52.2017187Z -
2019-09-30T01:17:52.2017389Z -error: aborting due to 5 previous errors
2019-09-30T01:17:52.2017572Z -
2019-09-30T01:17:52.2017746Z -
---
2019-09-30T01:17:52.2018630Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'mut_range_bound.rs'
2019-09-30T01:17:52.2018673Z 
2019-09-30T01:17:52.2018714Z error: 1 errors occurred comparing output.
2019-09-30T01:17:52.2018756Z status: exit code: 0
2019-09-30T01:17:52.2020150Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/mut_range_bound.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/mut_range_bound.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/mut_range_bound.stage-id.aux" "-A" "unused"
2019-09-30T01:17:52.2020554Z ------------------------------------------
2019-09-30T01:17:52.2020603Z 
2019-09-30T01:17:52.2020807Z ------------------------------------------
2019-09-30T01:17:52.2020850Z stderr:
---
2019-09-30T01:17:52.2021757Z normalized stderr:
2019-09-30T01:17:52.2021783Z 
2019-09-30T01:17:52.2021807Z 
2019-09-30T01:17:52.2021863Z expected stderr:
2019-09-30T01:17:52.2021907Z error: the loop variable `i` is only used to index `ns`.
2019-09-30T01:17:52.2022118Z   --> $DIR/needless_range_loop.rs:15:14
2019-09-30T01:17:52.2022223Z LL |     for i in 3..10 {
2019-09-30T01:17:52.2022355Z    |              ^^^^^
2019-09-30T01:17:52.2022410Z    |
2019-09-30T01:17:52.2022410Z    |
2019-09-30T01:17:52.2023039Z    = note: `-D clippy::needless-range-loop` implied by `-D warnings`
2019-09-30T01:17:52.2023134Z    |
2019-09-30T01:17:52.2023134Z    |
2019-09-30T01:17:52.2023208Z LL |     for <item> in ns.iter().take(10).skip(3) {
2019-09-30T01:17:52.2023286Z 
2019-09-30T01:17:52.2023286Z 
2019-09-30T01:17:52.2023346Z error: the loop variable `i` is only used to index `ms`.
2019-09-30T01:17:52.2023565Z   --> $DIR/needless_range_loop.rs:36:14
2019-09-30T01:17:52.2023649Z LL |     for i in 0..ms.len() {
2019-09-30T01:17:52.2023710Z    |              ^^^^^^^^^^^
2019-09-30T01:17:52.2023753Z help: consider using an iterator
2019-09-30T01:17:52.2023794Z    |
2019-09-30T01:17:52.2023794Z    |
2019-09-30T01:17:52.2023853Z LL |     for <item> in &mut ms {
2019-09-30T01:17:52.2023923Z 
2019-09-30T01:17:52.2023923Z 
2019-09-30T01:17:52.2023976Z error: the loop variable `i` is only used to index `ms`.
2019-09-30T01:17:52.2024210Z   --> $DIR/needless_range_loop.rs:42:14
2019-09-30T01:17:52.2024294Z LL |     for i in 0..ms.len() {
2019-09-30T01:17:52.2024357Z    |              ^^^^^^^^^^^
2019-09-30T01:17:52.2024407Z help: consider using an iterator
2019-09-30T01:17:52.2024447Z    |
2019-09-30T01:17:52.2024447Z    |
2019-09-30T01:17:52.2024505Z LL |     for <item> in &mut ms {
2019-09-30T01:17:52.2024575Z 
2019-09-30T01:17:52.2024575Z 
2019-09-30T01:17:52.2024620Z error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2024854Z   --> $DIR/needless_range_loop.rs:66:14
2019-09-30T01:17:52.2024898Z    |
2019-09-30T01:17:52.2024939Z LL |     for i in x..x + 4 {
2019-09-30T01:17:52.2025042Z help: consider using an iterator
2019-09-30T01:17:52.2025083Z    |
2019-09-30T01:17:52.2025083Z    |
2019-09-30T01:17:52.2025126Z LL |     for <item> in vec.iter_mut().skip(x).take(4) {
2019-09-30T01:17:52.2025227Z 
2019-09-30T01:17:52.2025227Z 
2019-09-30T01:17:52.2025271Z error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2025506Z   --> $DIR/needless_range_loop.rs:73:14
2019-09-30T01:17:52.2025551Z    |
2019-09-30T01:17:52.2025599Z LL |     for i in x..=x + 4 {
2019-09-30T01:17:52.2025705Z help: consider using an iterator
2019-09-30T01:17:52.2025745Z    |
2019-09-30T01:17:52.2025745Z    |
2019-09-30T01:17:52.2025790Z LL |     for <item> in vec.iter_mut().skip(x).take(4 + 1) {
2019-09-30T01:17:52.2025995Z 
2019-09-30T01:17:52.2025995Z 
2019-09-30T01:17:52.2026037Z error: the loop variable `i` is only used to index `arr`.
2019-09-30T01:17:52.2026247Z   --> $DIR/needless_range_loop.rs:79:14
2019-09-30T01:17:52.2026347Z LL |     for i in 0..3 {
2019-09-30T01:17:52.2026389Z    |              ^^^^
2019-09-30T01:17:52.2026456Z help: consider using an iterator
2019-09-30T01:17:52.2026494Z    |
2019-09-30T01:17:52.2026494Z    |
2019-09-30T01:17:52.2026533Z LL |     for <item> in &arr {
2019-09-30T01:17:52.2026619Z 
2019-09-30T01:17:52.2026619Z 
2019-09-30T01:17:52.2026776Z error: the loop variable `i` is only used to index `arr`.
2019-09-30T01:17:52.2027024Z   --> $DIR/needless_range_loop.rs:83:14
2019-09-30T01:17:52.2027123Z LL |     for i in 0..2 {
2019-09-30T01:17:52.2027164Z    |              ^^^^
2019-09-30T01:17:52.2027221Z help: consider using an iterator
2019-09-30T01:17:52.2027259Z    |
2019-09-30T01:17:52.2027259Z    |
2019-09-30T01:17:52.2027300Z LL |     for <item> in arr.iter().take(2) {
2019-09-30T01:17:52.2027389Z 
2019-09-30T01:17:52.2027389Z 
2019-09-30T01:17:52.2027430Z error: the loop variable `i` is only used to index `arr`.
2019-09-30T01:17:52.2027637Z   --> $DIR/needless_range_loop.rs:87:14
2019-09-30T01:17:52.2027841Z LL |     for i in 1..3 {
2019-09-30T01:17:52.2027881Z    |              ^^^^
2019-09-30T01:17:52.2027922Z help: consider using an iterator
2019-09-30T01:17:52.2027978Z    |
2019-09-30T01:17:52.2027978Z    |
2019-09-30T01:17:52.2028019Z LL |     for <item> in arr.iter().skip(1) {
2019-09-30T01:17:52.2028113Z 
2019-09-30T01:17:52.2028113Z 
2019-09-30T01:17:52.2028155Z error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2028391Z   --> $DIR/needless_range_loop.rs:93:14
2019-09-30T01:17:52.2028432Z    |
2019-09-30T01:17:52.2028491Z LL |     for i in 0..vec.len() {
2019-09-30T01:17:52.2028574Z help: consider using an iterator
2019-09-30T01:17:52.2028630Z    |
2019-09-30T01:17:52.2028630Z    |
2019-09-30T01:17:52.2028669Z LL |     for <item> in &vec {
2019-09-30T01:17:52.2028736Z 
2019-09-30T01:17:52.2028736Z 
2019-09-30T01:17:52.2028796Z error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2029004Z   --> $DIR/needless_range_loop.rs:102:14
2019-09-30T01:17:52.2029054Z    |
2019-09-30T01:17:52.2029112Z LL |     for i in 0..vec.len() {
2019-09-30T01:17:52.2029195Z help: consider using an iterator
2019-09-30T01:17:52.2029250Z    |
2019-09-30T01:17:52.2029250Z    |
2019-09-30T01:17:52.2029296Z LL |     for <item> in &vec {
2019-09-30T01:17:52.2029365Z 
2019-09-30T01:17:52.2029365Z 
2019-09-30T01:17:52.2029427Z error: the loop variable `j` is only used to index `STATIC`.
2019-09-30T01:17:52.2029637Z   --> $DIR/needless_range_loop.rs:107:14
2019-09-30T01:17:52.2029679Z    |
2019-09-30T01:17:52.2029718Z LL |     for j in 0..4 {
2019-09-30T01:17:52.2029818Z help: consider using an iterator
2019-09-30T01:17:52.2029857Z    |
2019-09-30T01:17:52.2029857Z    |
2019-09-30T01:17:52.2029914Z LL |     for <item> in &STATIC {
2019-09-30T01:17:52.2029981Z 
2019-09-30T01:17:52.2029981Z 
2019-09-30T01:17:52.2030025Z error: the loop variable `j` is only used to index `CONST`.
2019-09-30T01:17:52.2030257Z   --> $DIR/needless_range_loop.rs:111:14
2019-09-30T01:17:52.2030300Z    |
2019-09-30T01:17:52.2030339Z LL |     for j in 0..4 {
2019-09-30T01:17:52.2030446Z help: consider using an iterator
2019-09-30T01:17:52.2030484Z    |
2019-09-30T01:17:52.2030484Z    |
2019-09-30T01:17:52.2030539Z LL |     for <item> in &CONST {
2019-09-30T01:17:52.2030607Z 
2019-09-30T01:17:52.2030607Z 
2019-09-30T01:17:52.2030649Z error: the loop variable `i` is used to index `vec`
2019-09-30T01:17:52.2030873Z   --> $DIR/needless_range_loop.rs:115:14
2019-09-30T01:17:52.2030915Z    |
2019-09-30T01:17:52.2030954Z LL |     for i in 0..vec.len() {
2019-09-30T01:17:52.2031054Z help: consider using an iterator
2019-09-30T01:17:52.2031093Z    |
2019-09-30T01:17:52.2031093Z    |
2019-09-30T01:17:52.2031134Z LL |     for (i, <item>) in vec.iter().enumerate() {
2019-09-30T01:17:52.2031233Z 
2019-09-30T01:17:52.2031233Z 
2019-09-30T01:17:52.2031276Z error: the loop variable `i` is only used to index `vec2`.
2019-09-30T01:17:52.2031501Z   --> $DIR/needless_range_loop.rs:123:14
2019-09-30T01:17:52.2031544Z    |
2019-09-30T01:17:52.2031875Z LL |     for i in 0..vec.len() {
2019-09-30T01:17:52.2032010Z help: consider using an iterator
2019-09-30T01:17:52.2032048Z    |
2019-09-30T01:17:52.2032048Z    |
2019-09-30T01:17:52.2032089Z LL |     for <item> in vec2.iter().take(vec.len()) {
2019-09-30T01:17:52.2032181Z 
2019-09-30T01:17:52.2032181Z 
2019-09-30T01:17:52.2032223Z error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2032963Z   --> $DIR/needless_range_loop.rs:127:14
2019-09-30T01:17:52.2033039Z    |
2019-09-30T01:17:52.2033079Z LL |     for i in 5..vec.len() {
2019-09-30T01:17:52.2033182Z help: consider using an iterator
2019-09-30T01:17:52.2033373Z    |
2019-09-30T01:17:52.2033373Z    |
2019-09-30T01:17:52.2033414Z LL |     for <item> in vec.iter().skip(5) {
2019-09-30T01:17:52.2033509Z 
2019-09-30T01:17:52.2033509Z 
2019-09-30T01:17:52.2033561Z error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2033832Z   --> $DIR/needless_range_loop.rs:131:14
2019-09-30T01:17:52.2033935Z LL |     for i in 0..MAX_LEN {
2019-09-30T01:17:52.2033978Z    |              ^^^^^^^^^^
2019-09-30T01:17:52.2034037Z help: consider using an iterator
2019-09-30T01:17:52.2034077Z    |
2019-09-30T01:17:52.2034077Z    |
2019-09-30T01:17:52.2034119Z LL |     for <item> in vec.iter().take(MAX_LEN) {
2019-09-30T01:17:52.2034211Z 
2019-09-30T01:17:52.2034211Z 
2019-09-30T01:17:52.2034255Z error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2034962Z   --> $DIR/needless_range_loop.rs:135:14
2019-09-30T01:17:52.2035037Z    |
2019-09-30T01:17:52.2035092Z LL |     for i in 0..=MAX_LEN {
2019-09-30T01:17:52.2035177Z help: consider using an iterator
2019-09-30T01:17:52.2035237Z    |
2019-09-30T01:17:52.2035237Z    |
2019-09-30T01:17:52.2035281Z LL |     for <item> in vec.iter().take(MAX_LEN + 1) {
2019-09-30T01:17:52.2035382Z 
2019-09-30T01:17:52.2035382Z 
2019-09-30T01:17:52.2035426Z error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2035655Z   --> $DIR/needless_range_loop.rs:139:14
2019-09-30T01:17:52.2035757Z LL |     for i in 5..10 {
2019-09-30T01:17:52.2035799Z    |              ^^^^^
2019-09-30T01:17:52.2035843Z help: consider using an iterator
2019-09-30T01:17:52.2035901Z    |
2019-09-30T01:17:52.2035901Z    |
2019-09-30T01:17:52.2036053Z LL |     for <item> in vec.iter().take(10).skip(5) {
2019-09-30T01:17:52.2036125Z 
2019-09-30T01:17:52.2036125Z 
2019-09-30T01:17:52.2036184Z error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2036525Z   --> $DIR/needless_range_loop.rs:143:14
2019-09-30T01:17:52.2036628Z LL |     for i in 5..=10 {
2019-09-30T01:17:52.2036671Z    |              ^^^^^^
2019-09-30T01:17:52.2036721Z help: consider using an iterator
2019-09-30T01:17:52.2036779Z    |
2019-09-30T01:17:52.2036779Z    |
2019-09-30T01:17:52.2036822Z LL |     for <item> in vec.iter().take(10 + 1).skip(5) {
2019-09-30T01:17:52.2036898Z 
2019-09-30T01:17:52.2036898Z 
2019-09-30T01:17:52.2036959Z error: the loop variable `i` is used to index `vec`
2019-09-30T01:17:52.2037176Z   --> $DIR/needless_range_loop.rs:147:14
2019-09-30T01:17:52.2037220Z    |
2019-09-30T01:17:52.2037278Z LL |     for i in 5..vec.len() {
2019-09-30T01:17:52.2037363Z help: consider using an iterator
2019-09-30T01:17:52.2037403Z    |
2019-09-30T01:17:52.2037403Z    |
2019-09-30T01:17:52.2037463Z LL |     for (i, <item>) in vec.iter().enumerate().skip(5) {
2019-09-30T01:17:52.2037550Z 
2019-09-30T01:17:52.2037550Z 
2019-09-30T01:17:52.2037608Z error: the loop variable `i` is used to index `vec`
2019-09-30T01:17:52.2037824Z   --> $DIR/needless_range_loop.rs:151:14
2019-09-30T01:17:52.2038064Z LL |     for i in 5..10 {
2019-09-30T01:17:52.2038107Z    |              ^^^^^
2019-09-30T01:17:52.2038150Z help: consider using an iterator
2019-09-30T01:17:52.2038190Z    |
2019-09-30T01:17:52.2038190Z    |
2019-09-30T01:17:52.2038255Z LL |     for (i, <item>) in vec.iter().enumerate().take(10).skip(5) {
2019-09-30T01:17:52.2038337Z 
2019-09-30T01:17:52.2038337Z 
2019-09-30T01:17:52.2038393Z error: the loop variable `i` is used to index `vec`
2019-09-30T01:17:52.2038647Z   --> $DIR/needless_range_loop.rs:156:14
2019-09-30T01:17:52.2038691Z    |
2019-09-30T01:17:52.2038731Z LL |     for i in 0..vec.len() {
2019-09-30T01:17:52.2038922Z help: consider using an iterator
2019-09-30T01:17:52.2038962Z    |
2019-09-30T01:17:52.2038962Z    |
2019-09-30T01:17:52.2039011Z LL |     for (i, <item>) in vec.iter_mut().enumerate() {
2019-09-30T01:17:52.2039094Z 
2019-09-30T01:17:52.2039136Z error: aborting due to 22 previous errors
2019-09-30T01:17:52.2039171Z 
2019-09-30T01:17:52.2039196Z 
2019-09-30T01:17:52.2039196Z 
2019-09-30T01:17:52.2039221Z 
2019-09-30T01:17:52.2039260Z diff of stderr:
2019-09-30T01:17:52.2039288Z 
2019-09-30T01:17:52.2039552Z -error: the loop variable `i` is only used to index `ns`.
2019-09-30T01:17:52.2039765Z -  --> $DIR/needless_range_loop.rs:15:14
2019-09-30T01:17:52.2040154Z -LL |     for i in 3..10 {
2019-09-30T01:17:52.2040345Z -   |              ^^^^^
2019-09-30T01:17:52.2040518Z -   |
2019-09-30T01:17:52.2040518Z -   |
2019-09-30T01:17:52.2040759Z -   = note: `-D clippy::needless-range-loop` implied by `-D warnings`
2019-09-30T01:17:52.2041152Z -   |
2019-09-30T01:17:52.2041152Z -   |
2019-09-30T01:17:52.2041377Z -LL |     for <item> in ns.iter().take(10).skip(3) {
2019-09-30T01:17:52.2041880Z -
2019-09-30T01:17:52.2041880Z -
2019-09-30T01:17:52.2042104Z -error: the loop variable `i` is only used to index `ms`.
2019-09-30T01:17:52.2042353Z -  --> $DIR/needless_range_loop.rs:36:14
2019-09-30T01:17:52.2043480Z -LL |     for i in 0..ms.len() {
2019-09-30T01:17:52.2043694Z -   |              ^^^^^^^^^^^
2019-09-30T01:17:52.2043896Z -help: consider using an iterator
2019-09-30T01:17:52.2044071Z -   |
2019-09-30T01:17:52.2044071Z -   |
2019-09-30T01:17:52.2044268Z -LL |     for <item> in &mut ms {
2019-09-30T01:17:52.2044646Z -
2019-09-30T01:17:52.2044646Z -
2019-09-30T01:17:52.2044868Z -error: the loop variable `i` is only used to index `ms`.
2019-09-30T01:17:52.2045093Z -  --> $DIR/needless_range_loop.rs:42:14
2019-09-30T01:17:52.2045482Z -LL |     for i in 0..ms.len() {
2019-09-30T01:17:52.2045687Z -   |              ^^^^^^^^^^^
2019-09-30T01:17:52.2045889Z -help: consider using an iterator
2019-09-30T01:17:52.2046062Z -   |
2019-09-30T01:17:52.2046062Z -   |
2019-09-30T01:17:52.2046260Z -LL |     for <item> in &mut ms {
2019-09-30T01:17:52.2046646Z -
2019-09-30T01:17:52.2046646Z -
2019-09-30T01:17:52.2046867Z -error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2047091Z -  --> $DIR/needless_range_loop.rs:66:14
2019-09-30T01:17:52.2047265Z -   |
2019-09-30T01:17:52.2047463Z -LL |     for i in x..x + 4 {
2019-09-30T01:17:52.2047866Z -help: consider using an iterator
2019-09-30T01:17:52.2048039Z -   |
2019-09-30T01:17:52.2048039Z -   |
2019-09-30T01:17:52.2048262Z -LL |     for <item> in vec.iter_mut().skip(x).take(4) {
2019-09-30T01:17:52.2048666Z -
2019-09-30T01:17:52.2048666Z -
2019-09-30T01:17:52.2048899Z -error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2049121Z -  --> $DIR/needless_range_loop.rs:73:14
2019-09-30T01:17:52.2049296Z -   |
2019-09-30T01:17:52.2049493Z -LL |     for i in x..=x + 4 {
2019-09-30T01:17:52.2050022Z -help: consider using an iterator
2019-09-30T01:17:52.2050247Z -   |
2019-09-30T01:17:52.2050247Z -   |
2019-09-30T01:17:52.2050474Z -LL |     for <item> in vec.iter_mut().skip(x).take(4 + 1) {
2019-09-30T01:17:52.2050886Z -
2019-09-30T01:17:52.2050886Z -
2019-09-30T01:17:52.2051109Z -error: the loop variable `i` is only used to index `arr`.
2019-09-30T01:17:52.2051332Z -  --> $DIR/needless_range_loop.rs:79:14
2019-09-30T01:17:52.2051702Z -LL |     for i in 0..3 {
2019-09-30T01:17:52.2051903Z -   |              ^^^^
2019-09-30T01:17:52.2052107Z -help: consider using an iterator
2019-09-30T01:17:52.2052565Z -   |
2019-09-30T01:17:52.2052565Z -   |
2019-09-30T01:17:52.2052961Z -LL |     for <item> in &arr {
2019-09-30T01:17:52.2053344Z -
2019-09-30T01:17:52.2053344Z -
2019-09-30T01:17:52.2053567Z -error: the loop variable `i` is only used to index `arr`.
2019-09-30T01:17:52.2053793Z -  --> $DIR/needless_range_loop.rs:83:14
2019-09-30T01:17:52.2054173Z -LL |     for i in 0..2 {
2019-09-30T01:17:52.2054375Z -   |              ^^^^
2019-09-30T01:17:52.2054580Z -help: consider using an iterator
2019-09-30T01:17:52.2054755Z -   |
2019-09-30T01:17:52.2054755Z -   |
2019-09-30T01:17:52.2054969Z -LL |     for <item> in arr.iter().take(2) {
2019-09-30T01:17:52.2055366Z -
2019-09-30T01:17:52.2055366Z -
2019-09-30T01:17:52.2055590Z -error: the loop variable `i` is only used to index `arr`.
2019-09-30T01:17:52.2055812Z -  --> $DIR/needless_range_loop.rs:87:14
2019-09-30T01:17:52.2056184Z -LL |     for i in 1..3 {
2019-09-30T01:17:52.2056382Z -   |              ^^^^
2019-09-30T01:17:52.2056595Z -help: consider using an iterator
2019-09-30T01:17:52.2056771Z -   |
2019-09-30T01:17:52.2056771Z -   |
2019-09-30T01:17:52.2056985Z -LL |     for <item> in arr.iter().skip(1) {
2019-09-30T01:17:52.2057381Z -
2019-09-30T01:17:52.2057381Z -
2019-09-30T01:17:52.2057614Z -error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2057839Z -  --> $DIR/needless_range_loop.rs:93:14
2019-09-30T01:17:52.2058017Z -   |
2019-09-30T01:17:52.2058220Z -LL |     for i in 0..vec.len() {
2019-09-30T01:17:52.2058634Z -help: consider using an iterator
2019-09-30T01:17:52.2058808Z -   |
2019-09-30T01:17:52.2058808Z -   |
2019-09-30T01:17:52.2059005Z -LL |     for <item> in &vec {
2019-09-30T01:17:52.2059381Z -
2019-09-30T01:17:52.2059381Z -
2019-09-30T01:17:52.2059605Z -error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2059831Z -  --> $DIR/needless_range_loop.rs:102:14
2019-09-30T01:17:52.2060009Z -   |
2019-09-30T01:17:52.2060219Z -LL |     for i in 0..vec.len() {
2019-09-30T01:17:52.2060631Z -help: consider using an iterator
2019-09-30T01:17:52.2060806Z -   |
2019-09-30T01:17:52.2060806Z -   |
2019-09-30T01:17:52.2061002Z -LL |     for <item> in &vec {
2019-09-30T01:17:52.2061389Z -
2019-09-30T01:17:52.2061389Z -
2019-09-30T01:17:52.2061616Z -error: the loop variable `j` is only used to index `STATIC`.
2019-09-30T01:17:52.2061841Z -  --> $DIR/needless_range_loop.rs:107:14
2019-09-30T01:17:52.2062016Z -   |
2019-09-30T01:17:52.2062211Z -LL |     for j in 0..4 {
2019-09-30T01:17:52.2063467Z -help: consider using an iterator
2019-09-30T01:17:52.2063641Z -   |
2019-09-30T01:17:52.2063641Z -   |
2019-09-30T01:17:52.2063838Z -LL |     for <item> in &STATIC {
2019-09-30T01:17:52.2064218Z -
2019-09-30T01:17:52.2064218Z -
2019-09-30T01:17:52.2064443Z -error: the loop variable `j` is only used to index `CONST`.
2019-09-30T01:17:52.2064683Z -  --> $DIR/needless_range_loop.rs:111:14
2019-09-30T01:17:52.2064858Z -   |
2019-09-30T01:17:52.2065049Z -LL |     for j in 0..4 {
2019-09-30T01:17:52.2065496Z -help: consider using an iterator
2019-09-30T01:17:52.2065671Z -   |
2019-09-30T01:17:52.2065671Z -   |
2019-09-30T01:17:52.2066006Z -LL |     for <item> in &CONST {
2019-09-30T01:17:52.2066438Z -
2019-09-30T01:17:52.2066438Z -
2019-09-30T01:17:52.2066659Z -error: the loop variable `i` is used to index `vec`
2019-09-30T01:17:52.2066884Z -  --> $DIR/needless_range_loop.rs:115:14
2019-09-30T01:17:52.2067064Z -   |
2019-09-30T01:17:52.2067263Z -LL |     for i in 0..vec.len() {
2019-09-30T01:17:52.2067669Z -help: consider using an iterator
2019-09-30T01:17:52.2067844Z -   |
2019-09-30T01:17:52.2067844Z -   |
2019-09-30T01:17:52.2068064Z -LL |     for (i, <item>) in vec.iter().enumerate() {
2019-09-30T01:17:52.2068593Z -
2019-09-30T01:17:52.2068593Z -
2019-09-30T01:17:52.2068819Z -error: the loop variable `i` is only used to index `vec2`.
2019-09-30T01:17:52.2069046Z -  --> $DIR/needless_range_loop.rs:123:14
2019-09-30T01:17:52.2069224Z -   |
2019-09-30T01:17:52.2069424Z -LL |     for i in 0..vec.len() {
2019-09-30T01:17:52.2069845Z -help: consider using an iterator
2019-09-30T01:17:52.2070020Z -   |
2019-09-30T01:17:52.2070020Z -   |
2019-09-30T01:17:52.2070240Z -LL |     for <item> in vec2.iter().take(vec.len()) {
2019-09-30T01:17:52.2070646Z -
2019-09-30T01:17:52.2070646Z -
2019-09-30T01:17:52.2070869Z -error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2071091Z -  --> $DIR/needless_range_loop.rs:127:14
2019-09-30T01:17:52.2071268Z -   |
2019-09-30T01:17:52.2071469Z -LL |     for i in 5..vec.len() {
2019-09-30T01:17:52.2072060Z -help: consider using an iterator
2019-09-30T01:17:52.2072247Z -   |
2019-09-30T01:17:52.2072247Z -   |
2019-09-30T01:17:52.2072665Z -LL |     for <item> in vec.iter().skip(5) {
2019-09-30T01:17:52.2073115Z -
2019-09-30T01:17:52.2073115Z -
2019-09-30T01:17:52.2073338Z -error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2073573Z -  --> $DIR/needless_range_loop.rs:131:14
2019-09-30T01:17:52.2073948Z -LL |     for i in 0..MAX_LEN {
2019-09-30T01:17:52.2074154Z -   |              ^^^^^^^^^^
2019-09-30T01:17:52.2074355Z -help: consider using an iterator
2019-09-30T01:17:52.2074529Z -   |
2019-09-30T01:17:52.2074529Z -   |
2019-09-30T01:17:52.2074745Z -LL |     for <item> in vec.iter().take(MAX_LEN) {
2019-09-30T01:17:52.2075142Z -
2019-09-30T01:17:52.2075142Z -
2019-09-30T01:17:52.2075365Z -error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2075587Z -  --> $DIR/needless_range_loop.rs:135:14
2019-09-30T01:17:52.2075761Z -   |
2019-09-30T01:17:52.2075969Z -LL |     for i in 0..=MAX_LEN {
2019-09-30T01:17:52.2076378Z -help: consider using an iterator
2019-09-30T01:17:52.2076551Z -   |
2019-09-30T01:17:52.2076551Z -   |
2019-09-30T01:17:52.2076771Z -LL |     for <item> in vec.iter().take(MAX_LEN + 1) {
2019-09-30T01:17:52.2077180Z -
2019-09-30T01:17:52.2077180Z -
2019-09-30T01:17:52.2077402Z -error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2077625Z -  --> $DIR/needless_range_loop.rs:139:14
2019-09-30T01:17:52.2077995Z -LL |     for i in 5..10 {
2019-09-30T01:17:52.2078194Z -   |              ^^^^^
2019-09-30T01:17:52.2078397Z -help: consider using an iterator
2019-09-30T01:17:52.2078569Z -   |
2019-09-30T01:17:52.2078569Z -   |
2019-09-30T01:17:52.2078795Z -LL |     for <item> in vec.iter().take(10).skip(5) {
2019-09-30T01:17:52.2079190Z -
2019-09-30T01:17:52.2079190Z -
2019-09-30T01:17:52.2079411Z -error: the loop variable `i` is only used to index `vec`.
2019-09-30T01:17:52.2079644Z -  --> $DIR/needless_range_loop.rs:143:14
2019-09-30T01:17:52.2080014Z -LL |     for i in 5..=10 {
2019-09-30T01:17:52.2080216Z -   |              ^^^^^^
2019-09-30T01:17:52.2080551Z -help: consider using an iterator
2019-09-30T01:17:52.2080771Z -   |
2019-09-30T01:17:52.2080771Z -   |
2019-09-30T01:17:52.2081005Z -LL |     for <item> in vec.iter().take(10 + 1).skip(5) {
2019-09-30T01:17:52.2081406Z -
2019-09-30T01:17:52.2081406Z -
2019-09-30T01:17:52.2081624Z -error: the loop variable `i` is used to index `vec`
2019-09-30T01:17:52.2081847Z -  --> $DIR/needless_range_loop.rs:147:14
2019-09-30T01:17:52.2082024Z -   |
2019-09-30T01:17:52.2082225Z -LL |     for i in 5..vec.len() {
2019-09-30T01:17:52.2082868Z -help: consider using an iterator
2019-09-30T01:17:52.2083045Z -   |
2019-09-30T01:17:52.2083045Z -   |
2019-09-30T01:17:52.2083284Z -LL |     for (i, <item>) in vec.iter().enumerate().skip(5) {
2019-09-30T01:17:52.2083855Z -
2019-09-30T01:17:52.2083855Z -
2019-09-30T01:17:52.2084072Z -error: the loop variable `i` is used to index `vec`
2019-09-30T01:17:52.2084307Z -  --> $DIR/needless_range_loop.rs:151:14
2019-09-30T01:17:52.2084676Z -LL |     for i in 5..10 {
2019-09-30T01:17:52.2084877Z -   |              ^^^^^
2019-09-30T01:17:52.2085076Z -help: consider using an iterator
2019-09-30T01:17:52.2085249Z -   |
2019-09-30T01:17:52.2085249Z -   |
2019-09-30T01:17:52.2085495Z -LL |     for (i, <item>) in vec.iter().enumerate().take(10).skip(5) {
2019-09-30T01:17:52.2085909Z -
2019-09-30T01:17:52.2085909Z -
2019-09-30T01:17:52.2086131Z -error: the loop variable `i` is used to index `vec`
2019-09-30T01:17:52.2086347Z -  --> $DIR/needless_range_loop.rs:156:14
2019-09-30T01:17:52.2086522Z -   |
2019-09-30T01:17:52.2086722Z -LL |     for i in 0..vec.len() {
2019-09-30T01:17:52.2087144Z -help: consider using an iterator
2019-09-30T01:17:52.2087315Z -   |
2019-09-30T01:17:52.2087315Z -   |
2019-09-30T01:17:52.2087548Z -LL |     for (i, <item>) in vec.iter_mut().enumerate() {
2019-09-30T01:17:52.2087954Z -
2019-09-30T01:17:52.2088174Z -error: aborting due to 22 previous errors
2019-09-30T01:17:52.2088347Z -
2019-09-30T01:17:52.2088512Z -
---
2019-09-30T01:17:52.2089400Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'needless_range_loop.rs'
2019-09-30T01:17:52.2089453Z 
2019-09-30T01:17:52.2089497Z error: 1 errors occurred comparing output.
2019-09-30T01:17:52.2089549Z status: exit code: 0
2019-09-30T01:17:52.2091064Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/needless_range_loop.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/needless_range_loop.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/needless_range_loop.stage-id.aux" "-A" "unused"
2019-09-30T01:17:52.2091510Z ------------------------------------------
2019-09-30T01:17:52.2091543Z 
2019-09-30T01:17:52.2091754Z ------------------------------------------
2019-09-30T01:17:52.2091810Z stderr:
---
2019-09-30T01:17:52.2094060Z    |
2019-09-30T01:17:52.2094116Z    = note: `#[deny(const_err)]` on by default
2019-09-30T01:17:52.2094145Z 
2019-09-30T01:17:52.2094379Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-09-30T01:17:52.2094428Z   left: `Size { raw: 0 }`,
2019-09-30T01:17:52.2094712Z  right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-09-30T01:17:52.2094807Z 
2019-09-30T01:17:52.2094874Z error: internal compiler error: unexpected panic
2019-09-30T01:17:52.2094904Z 
2019-09-30T01:17:52.2094948Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-30T01:17:52.2094948Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-30T01:17:52.2094978Z 
2019-09-30T01:17:52.2095396Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-30T01:17:52.2095678Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-30T01:17:52.2095724Z 
2019-09-30T01:17:52.2095943Z note: compiler flags: -Z ui-testing -C prefer-dynamic
2019-09-30T01:17:52.2095975Z 
---
2019-09-30T01:17:52.2101751Z -LL |     &empty[1..5];
2019-09-30T01:17:52.2101951Z -   |            ^
2019-09-30T01:17:52.2102124Z -   |
2019-09-30T01:17:52.2102360Z -   = note: `-D clippy::out-of-bounds-indexing` implied by `-D warnings`
2019-09-30T01:17:52.2102864Z +thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-09-30T01:17:52.2102925Z +  left: `Size { raw: 0 }`,
2019-09-30T01:17:52.2103205Z + right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-09-30T01:17:52.2103278Z +note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-30T01:17:52.2103522Z -error: range is out of bounds
2019-09-30T01:17:52.2103736Z -  --> $DIR/empty_array.rs:8:16
2019-09-30T01:17:52.2103913Z -   |
2019-09-30T01:17:52.2104104Z -LL |     &empty[0..=4];
---
2019-09-30T01:17:52.2105905Z -  --> $DIR/empty_array.rs:10:12
2019-09-30T01:17:52.2106081Z -   |
2019-09-30T01:17:52.2106271Z -LL |     &empty[1..];
2019-09-30T01:17:52.2106472Z -   |            ^
2019-09-30T01:17:52.2106806Z +note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-30T01:17:52.2107084Z -error: range is out of bounds
2019-09-30T01:17:52.2107287Z -  --> $DIR/empty_array.rs:11:14
2019-09-30T01:17:52.2107464Z -   |
2019-09-30T01:17:52.2107678Z -LL |     &empty[..4];
---
2019-09-30T01:17:52.2108681Z -  --> $DIR/empty_array.rs:12:16
2019-09-30T01:17:52.2108902Z -   |
2019-09-30T01:17:52.2109112Z -LL |     &empty[0..=0];
2019-09-30T01:17:52.2109302Z -   |                ^
2019-09-30T01:17:52.2109523Z +note: compiler flags: -Z ui-testing -C prefer-dynamic
2019-09-30T01:17:52.2109777Z -error: range is out of bounds
2019-09-30T01:17:52.2109978Z -  --> $DIR/empty_array.rs:13:15
2019-09-30T01:17:52.2110154Z -   |
2019-09-30T01:17:52.2110360Z -LL |     &empty[..=0];
---
2019-09-30T01:17:52.2112618Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base' 'out_of_bounds_indexing/empty_array.rs'
2019-09-30T01:17:52.2112676Z 
2019-09-30T01:17:52.2112737Z error: 1 errors occurred comparing output.
2019-09-30T01:17:52.2112779Z status: exit code: 101
2019-09-30T01:17:52.2114317Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/out_of_bounds_indexing/empty_array.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/out_of_bounds_indexing/empty_array.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-23d814d56e4a27a6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex-28e39b75b54d56f5.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libclippy_lints-00261d9ef13abc66.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-7ccc90d9ed54ca9b/out/test_build_base/out_of_bounds_indexing/empty_array.stage-id.aux" "-A" "unused"
2019-09-30T01:17:52.2114783Z ------------------------------------------
2019-09-30T01:17:52.2114826Z 
2019-09-30T01:17:52.2115085Z ------------------------------------------
2019-09-30T01:17:52.2115131Z stderr:
2019-09-30T01:17:52.2115131Z stderr:
2019-09-30T01:17:52.2115361Z ------------------------------------------
2019-09-30T01:17:52.2116509Z {"message":"index out of bounds: the len is 0 but the index is 0","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/out_of_bounds_indexing/empty_array.rs","byte_start":147,"byte_end":155,"line_start":6,"line_end":6,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    empty[0];","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(const_err)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: index out of bounds: the len is 0 but the index is 0\n  --> tests/ui/out_of_bounds_indexing/empty_array.rs:6:5\n   |\nLL |     empty[0];\n   |     ^^^^^^^^\n   |\n   = note: `#[deny(const_err)]` on by default\n\n"}
2019-09-30T01:17:52.2116897Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2019-09-30T01:17:52.2116951Z   left: `Size { raw: 0 }`,
2019-09-30T01:17:52.2117250Z  right: `Size { raw: 1 }`: Size mismatch when writing bits', src/librustc_mir/interpret/place.rs:736:21
2019-09-30T01:17:52.2117365Z 
2019-09-30T01:17:52.2117411Z error: internal compiler error: unexpected panic
2019-09-30T01:17:52.2117453Z 
2019-09-30T01:17:52.2117500Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-30T01:17:52.2117500Z note: the compiler unexpectedly panicked. this is a bug.
2019-09-30T01:17:52.2117531Z 
2019-09-30T01:17:52.2118042Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-09-30T01:17:52.2118348Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-30T01:17:52.2118381Z 
2019-09-30T01:17:52.2118634Z note: compiler flags: -Z ui-testing -C prefer-dynamic
2019-09-30T01:17:52.2118678Z 
---
2019-09-30T02:00:48.1506830Z Verifying status of rustfmt...
2019-09-30T02:00:48.1524363Z Verifying status of clippy-driver...
2019-09-30T02:00:48.1539168Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-09-30T02:00:48.1554662Z 
2019-09-30T02:00:48.1555294Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-09-30T02:00:48.1555340Z 
2019-09-30T02:00:48.1555684Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-09-30T02:00:48.1555747Z commit another update.
2019-09-30T02:00:48.1555779Z 
2019-09-30T02:00:48.1556083Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-09-30T02:00:48.1557578Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-09-30T02:00:48.1557942Z proper steps.
2019-09-30T02:00:48.1577698Z   local time: Mon Sep 30 02:00:48 UTC 2019
2019-09-30T02:00:48.5225973Z   network time: Mon, 30 Sep 2019 02:00:48 GMT
2019-09-30T02:00:48.5231039Z == end clock drift check ==
2019-09-30T02:00:48.5231039Z == end clock drift check ==
2019-09-30T02:00:49.6622311Z ##[error]Bash exited with code '3'.
2019-09-30T02:00:49.6687221Z ##[section]Starting: Checkout
2019-09-30T02:00:49.6689606Z ==============================================================================
2019-09-30T02:00:49.6689670Z Task         : Get sources
2019-09-30T02:00:49.6689722Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
