plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_driver/src/lib.rs at line 1189:
     // Choose error text
     //let msg = if !msg.is_empty() { msg } else { "aborted" };
     //let hash = msg.chars().fold(0, |accum, val| accum + (val as uint) );
-    let hash = info.location()
-        .map(|l| l.line() + l.column())
-        .unwrap_or(5);
+    let hash = info.location().map(|l| l.line() + l.column()).unwrap_or(5);
     let quote = match hash % 10 {
-        0 => "
+        0 => {
+            "
 It was from the artists and poets that the pertinent answers came, and I
 know that panic would have broken loose had they been able to compare notes.
 As it was, lacking their original letters, I half suspected the compiler of
Diff in /checkout/compiler/rustc_driver/src/lib.rs at line 1200:
 having asked leading questions, or of having edited the correspondence in
-corroboration of what he had latently resolved to see.",
-        1 => "
+corroboration of what he had latently resolved to see."
+        1 => {
+            "
+            "
 There are not many persons who know what wonders are opened to them in the
 stories and visions of their youth; for when as children we listen and dream,
 we think but half-formed thoughts, and when as men we try to remember, we are
Diff in /checkout/compiler/rustc_driver/src/lib.rs at line 1209:
 down to sleeping cities of bronze and stone, and of shadowy companies of heroes
 that ride caparisoned white horses along the edges of thick forests; and then
 we know that we have looked back through the ivory gates into that world of
-wonder which was ours before we were wise and unhappy.",
-        2 => "
+wonder which was ours before we were wise and unhappy."
+        2 => {
+            "
+            "
 Instead of the poems I had hoped for, there came only a shuddering blackness
 and ineffable loneliness; and I saw at last a fearful truth which no one had
 ever dared to breathe before — the unwhisperable secret of secrets — The fact
Diff in /checkout/compiler/rustc_driver/src/lib.rs at line 1217:
 that this city of stone and stridor is not a sentient perpetuation of Old New
 York as London is of Old London and Paris of Old Paris, but that it is in fact
 quite dead, its sprawling body imperfectly embalmed and infested with queer
-animate things which have nothing to do with it as it was in life.",
-        3 => "
+animate things which have nothing to do with it as it was in life."
+        3 => {
+            "
+            "
 The ocean ate the last of the land and poured into the smoking gulf, thereby
 giving up all it had ever conquered. From the new-flooded lands it flowed
 again, uncovering death and decay; and from its ancient and immemorial bed it
Diff in /checkout/compiler/rustc_driver/src/lib.rs at line 1227:
 moon laid pale lilies of light on dead London, and Paris stood up from its damp
 grave to be sanctified with star-dust. Then rose spires and monoliths that were
 weedy but not remembered; terrible spires and monoliths of lands that men never
-knew were lands...",
-        4 => "
+knew were lands..."
+        4 => {
+            "
+            "
 There was a night when winds from unknown spaces whirled us irresistibly into
 limitless vacuum beyond all thought and entity. Perceptions of the most
 maddeningly untransmissible sort thronged upon us; perceptions of infinity
Diff in /checkout/compiler/rustc_driver/src/lib.rs at line 1235:
 which at the time convulsed us with joy, yet which are now partly lost to my
-memory and partly incapable of presentation to others.",
-        _ => "You've met with a terrible fate, haven't you?"
+memory and partly incapable of presentation to others."
+        }
+        _ => "You've met with a terrible fate, haven't you?",
 
 
     let mut xs: Vec<Cow<'static, str>> = vec![
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_driver/src/pretty.rs" "/checkout/compiler/rustc_serialize/src/collection_impls.rs" "/checkout/compiler/rustc_driver/src/args.rs" "/checkout/compiler/rustc_driver/src/lib.rs" "/checkout/compiler/rustc_serialize/src/serialize.rs" "/checkout/compiler/rustc_serialize/src/json.rs" "/checkout/compiler/rustc_serialize/src/json/tests.rs" "/checkout/compiler/rustc_serialize/src/leb128.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
