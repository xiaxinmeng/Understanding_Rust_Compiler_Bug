plain
   Compiling globset v0.4.8
   Compiling ignore v0.4.18
   Compiling xz2 v0.1.6
   Compiling toml v0.5.9
error[E0600]: cannot apply unary operator `!` to type `DryRun`
   --> setup.rs:161:52
    |
161 |     } else if stage_dir_exists(&stage_path[..]) && !config.dry_run {
    |
    |
note: an implementation of `Not` might be missing for `DryRun`
    |
    |
36  | pub enum DryRun {
    | ^^^^^^^^^^^^^^^ must implement `Not`


error[E0600]: cannot apply unary operator `!` to type `DryRun`
   --> setup.rs:181:8
    |
181 |     if !config.dry_run {
    |
    |
note: an implementation of `Not` might be missing for `DryRun`
    |
    |
36  | pub enum DryRun {
    | ^^^^^^^^^^^^^^^ must implement `Not`

For more information about this error, try `rustc --explain E0600`.
error: could not compile `bootstrap` due to 2 previous errors
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
