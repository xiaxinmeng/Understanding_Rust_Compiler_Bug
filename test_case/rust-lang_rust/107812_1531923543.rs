plain
   Compiling toml v0.5.9
   Compiling is-terminal v0.4.6
   Compiling fd-lock v3.0.11
   Compiling xz2 v0.1.6
error[E0592]: duplicate definitions with name `download_bootstrap`
   --> download.rs:363:5
    |
363 |     pub fn download_bootstrap(&self, commit: &str) -> PathBuf {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `download_bootstrap`
...
674 |     pub fn download_bootstrap(&self, commit: &str) -> PathBuf {
    |     --------------------------------------------------------- other definition for `download_bootstrap`
error[E0592]: duplicate definitions with name `download_component`
   --> download.rs:383:5
    |
383 | /     fn download_component(
---
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 2/5:
##[group]Building bootstrap
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0592]: duplicate definitions with name `download_bootstrap`
   --> download.rs:363:5
    |
363 |     pub fn download_bootstrap(&self, commit: &str) -> PathBuf {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `download_bootstrap`
...
674 |     pub fn download_bootstrap(&self, commit: &str) -> PathBuf {
    |     --------------------------------------------------------- other definition for `download_bootstrap`
error[E0592]: duplicate definitions with name `download_component`
   --> download.rs:383:5
    |
383 | /     fn download_component(
---
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 3/5:
##[group]Building bootstrap
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0592]: duplicate definitions with name `download_bootstrap`
   --> download.rs:363:5
    |
363 |     pub fn download_bootstrap(&self, commit: &str) -> PathBuf {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `download_bootstrap`
...
674 |     pub fn download_bootstrap(&self, commit: &str) -> PathBuf {
    |     --------------------------------------------------------- other definition for `download_bootstrap`
error[E0592]: duplicate definitions with name `download_component`
   --> download.rs:383:5
    |
383 | /     fn download_component(
---
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 4/5:
##[group]Building bootstrap
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0592]: duplicate definitions with name `download_bootstrap`
   --> download.rs:363:5
    |
363 |     pub fn download_bootstrap(&self, commit: &str) -> PathBuf {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `download_bootstrap`
...
674 |     pub fn download_bootstrap(&self, commit: &str) -> PathBuf {
    |     --------------------------------------------------------- other definition for `download_bootstrap`
error[E0592]: duplicate definitions with name `download_component`
   --> download.rs:383:5
    |
383 | /     fn download_component(
---
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 5/5:
##[group]Building bootstrap
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0592]: duplicate definitions with name `download_bootstrap`
   --> download.rs:363:5
    |
363 |     pub fn download_bootstrap(&self, commit: &str) -> PathBuf {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `download_bootstrap`
...
674 |     pub fn download_bootstrap(&self, commit: &str) -> PathBuf {
    |     --------------------------------------------------------- other definition for `download_bootstrap`
error[E0592]: duplicate definitions with name `download_component`
   --> download.rs:383:5
    |
383 | /     fn download_component(
