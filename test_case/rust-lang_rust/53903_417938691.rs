plain
[00:46:01] ....................................................................................................
[00:46:04] ....................................................................................................
[00:46:07] ....................................................................................................
[00:46:11] ........i...........................................................................................
[00:46:15] .......................................................................F............................
[00:46:22] .......................ii..iii......................................................................
[00:46:25] ....................................................................................................
[00:46:27] ....................................................................................................
[00:46:30] ....................................................................................................
---
[00:48:01] 
[00:48:01] ---- [ui] ui/consts/const-eval/union-ice.rs stdout ----
[00:48:01] diff of stderr:
[00:48:01] 
[00:48:01] 15 LL | | };
[00:48:01] 16    | |__^ attempted to read undefined bytes
[00:48:01] 17 
[00:48:01] - error[E0080]: this constant likely exhibits undefined behavior
[00:48:01] -    |
[00:48:01] -    |
[00:48:01] - LL | / const FIELD_PATH2: Struct2 = Struct2 { //~ ERROR this constant likely exhibits undefined behavior
[00:48:01] - LL | |     b: [
[00:48:01] - LL | |         nd":28,"column_start":1,"column_end":3,"is_primary":true,"text":[{"text":"const FIELD_PATH: Struct = Struct { //~ ERROR this constant cannot be used","highlight_start":1,"highlight_end":75},{"text":"    a: 42,","highlight_start":1,"highlight_end":11},{"text":"    b: unsafe { UNION.field3 },","highlight_start":1,"highlight_end":32},{"text":"};","highlight_start":1,"highlight_end":3}],"label":"attempted to read undefined bytes","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this constant cannot be used\n  --> /checkout/src/test/ui/consts/const-eval/union-ice.rs:25:1\n   |\nLL | / const FIELD_PATH: Struct = Struct { //~ ERROR this constant cannot be used\nLL | |     a: 42,\nLL | |     b: unsafe { UNION.field3 },\nLL | | };\n   | |__^ attempted to read undefined bytes\n\n"}
[00:48:01] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:01] ------------------------------------------
[00:48:01] 
[00:48:01] thread '[ui] ui/consts/const-eval/union-ice.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:48:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:01] 
[00:48:01] ---- [ui] ui/union-ub-fat-ptr.rs stdout ----
[00:48:01] diff of stderr:
[00:48:01] 
[00:48:01] 26   --> $DIR/union-ub-fat-ptr.rs:99:1
[00:48:01] 27    |
[00:48:01] 28 LL | const B2: &[u8] = unsafe { SliceTransmute { repr: SliceRepr { ptr: &42, len: 999 } }.slice};
[00:48:01] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/union-ub-fat-ptr.rs","byte_start":2182,"byte_end":2277,"line_start":93,"line_end":93,"column_start":1,"column_end":96,"is_primary":true,"text":[{"text":"const C2: &MyStr = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.my_str};","highlight_start":1,"highlight_end":96}],"label":"type validation failed: encountered non-integer slice length in fat pointer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: this constant likely exhibits undefined behavior\n  --> /checkout/src/test/ui/union-ub-fat-ptr.rs:93:1\n   |\nLL | const C2: &MyStr = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.my_str};\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in fat pointer\n   |\n   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior\n\n"}
[00:48:01] {"message":"this constant likely exhibits undefined behavior","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n