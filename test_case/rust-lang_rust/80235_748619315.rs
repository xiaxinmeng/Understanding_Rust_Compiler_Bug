
error[E0080]: it is undefined behavior to use this value
  --> /home/r/src/rust/rustc/src/test/ui/consts/const-eval/promote-static.rs:7:1
   |
LL | / static NONE_REF_REF: &&Option<String> = {
LL | |     let x = &&NONE;
LL | |     x
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable at .<deref>
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
