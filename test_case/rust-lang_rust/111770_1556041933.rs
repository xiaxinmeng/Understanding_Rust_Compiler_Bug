plain
   Compiling clap_complete v4.2.2
error[E0425]: cannot find value `version_file` in this scope
    --> lib.rs:1346:46
     |
1346 |             let version = fs::read_to_string(version_file).ok()?;

error[E0061]: this function takes 0 arguments but 1 argument was supplied
    --> lib.rs:1358:21
     |
     |
1358 |         let count = extract_beta_rev_from_file(self.src.join("version")).unwrap_or_else(|| {
     |                                                |
     |                                                unexpected argument of type `PathBuf`
     |                                                help: remove the extra argument
     |
     |
note: function defined here
    --> lib.rs:1345:12
     |
1345 |         fn extract_beta_rev_from_file<P: AsRef<Path>>() -> Option<String> {


error[E0425]: cannot find function `extract_beta_rev` in this scope
     |
     |
1348 |             extract_beta_rev(&version)

Some errors have detailed explanations: E0061, E0425.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `bootstrap` (lib) due to 3 previous errors
---
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0425]: cannot find value `version_file` in this scope
    --> lib.rs:1346:46
     |
1346 |             let version = fs::read_to_string(version_file).ok()?;

error[E0061]: this function takes 0 arguments but 1 argument was supplied
    --> lib.rs:1358:21
     |
     |
1358 |         let count = extract_beta_rev_from_file(self.src.join("version")).unwrap_or_else(|| {
     |                                                |
     |                                                unexpected argument of type `PathBuf`
     |                                                help: remove the extra argument
     |
     |
note: function defined here
    --> lib.rs:1345:12
     |
1345 |         fn extract_beta_rev_from_file<P: AsRef<Path>>() -> Option<String> {


error[E0425]: cannot find function `extract_beta_rev` in this scope
     |
     |
1348 |             extract_beta_rev(&version)

Some errors have detailed explanations: E0061, E0425.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `bootstrap` (lib) due to 3 previous errors
---
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0425]: cannot find value `version_file` in this scope
    --> lib.rs:1346:46
     |
1346 |             let version = fs::read_to_string(version_file).ok()?;

error[E0061]: this function takes 0 arguments but 1 argument was supplied
    --> lib.rs:1358:21
     |
     |
1358 |         let count = extract_beta_rev_from_file(self.src.join("version")).unwrap_or_else(|| {
     |                                                |
     |                                                unexpected argument of type `PathBuf`
     |                                                help: remove the extra argument
     |
     |
note: function defined here
    --> lib.rs:1345:12
     |
1345 |         fn extract_beta_rev_from_file<P: AsRef<Path>>() -> Option<String> {


error[E0425]: cannot find function `extract_beta_rev` in this scope
     |
     |
1348 |             extract_beta_rev(&version)

Some errors have detailed explanations: E0061, E0425.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `bootstrap` (lib) due to 3 previous errors
---
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0425]: cannot find value `version_file` in this scope
    --> lib.rs:1346:46
     |
1346 |             let version = fs::read_to_string(version_file).ok()?;

error[E0061]: this function takes 0 arguments but 1 argument was supplied
    --> lib.rs:1358:21
     |
     |
1358 |         let count = extract_beta_rev_from_file(self.src.join("version")).unwrap_or_else(|| {
     |                                                |
     |                                                unexpected argument of type `PathBuf`
     |                                                help: remove the extra argument
     |
     |
note: function defined here
    --> lib.rs:1345:12
     |
1345 |         fn extract_beta_rev_from_file<P: AsRef<Path>>() -> Option<String> {


error[E0425]: cannot find function `extract_beta_rev` in this scope
     |
     |
1348 |             extract_beta_rev(&version)

Some errors have detailed explanations: E0061, E0425.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `bootstrap` (lib) due to 3 previous errors
---
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0425]: cannot find value `version_file` in this scope
    --> lib.rs:1346:46
     |
1346 |             let version = fs::read_to_string(version_file).ok()?;

error[E0061]: this function takes 0 arguments but 1 argument was supplied
    --> lib.rs:1358:21
     |
     |
1358 |         let count = extract_beta_rev_from_file(self.src.join("version")).unwrap_or_else(|| {
     |                                                |
     |                                                unexpected argument of type `PathBuf`
     |                                                help: remove the extra argument
     |
     |
note: function defined here
    --> lib.rs:1345:12
     |
1345 |         fn extract_beta_rev_from_file<P: AsRef<Path>>() -> Option<String> {


error[E0425]: cannot find function `extract_beta_rev` in this scope
     |
     |
1348 |             extract_beta_rev(&version)

Some errors have detailed explanations: E0061, E0425.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `bootstrap` (lib) due to 3 previous errors
