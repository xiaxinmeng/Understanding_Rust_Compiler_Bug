
-error: methods called `as_*` usually take `self` by reference or `self` by mutable reference
-  --> $DIR/def_id_nocore.rs:26:19
+error[E0718]: `start` language item must be applied to a function with 1 generic argument
    |
    |
-LL |     pub fn as_ref(self) -> &'static str {
-   |                   ^^^^
-   |
-   = note: `-D clippy::wrong-self-convention` implied by `-D warnings`
-   = help: consider choosing a less ambiguous name
+LL | #[lang = "start"]
+   | ^^^^^^^^^^^^^^^^^
+LL | #[start]
+LL | fn start(_argc: isize, _argv: *const *const u8) -> isize {
+   |         - this function has 0 generic arguments
 error: aborting due to previous error
 