plain
travis_time:end:0e9ac420:start=1556290816020680043,finish=1556290964238484141,duration=148217804098
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:17]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:06:23]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:27]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:28]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:54] error[E0599]: no variant named `Body` found for type `session::config::OutputType` in the current scope
[00:07:54]    --> src/librustc/session/config.rs:162:27
[00:07:54] 142 | pub enum OutputType {
[00:07:54] 142 | pub enum OutputType {
[00:07:54]     | ------------------- variant `Body` not found here
[00:07:54] ...
[00:07:54] 162 |             | OutputType::Body
[00:07:54]     |                           ^^^^ variant not found in `session::config::OutputType`
[00:07:54] 
[00:07:54] error[E0599]: no variant named `Body` found for type `session::config::OutputType` in the current scope
[00:07:54]    --> src/librustc/session/config.rs:173:25
[00:07:54] 142 | pub enum OutputType {
[00:07:54] 142 | pub enum OutputType {
[00:07:54]     | ------------------- variant `Body` not found here
[00:07:54] ...
[00:07:54] 173 |             OutputType::Body => "mir",
[00:07:54]     |                         ^^^^ variant not found in `session::config::OutputType`
[00:07:54] 
[00:07:54] error[E0599]: no variant named `Body` found for type `session::config::OutputType` in the current scope
[00:07:54]    --> src/librustc/session/config.rs:185:34
[00:07:54] 142 | pub enum OutputType {
[00:07:54] 142 | pub enum OutputType {
[00:07:54]     | ------------------- variant `Body` not found here
[00:07:54] ...
[00:07:54] 185 |             "mir" => OutputType::Body,
[00:07:54]     |                                  ^^^^ variant not found in `session::config::OutputType`
[00:07:54] 
[00:07:54] error[E0599]: no variant named `Body` found for type `session::config::OutputType` in the current scope
[00:07:54]    --> src/librustc/session/config.rs:201:25
[00:07:54] 142 | pub enum OutputType {
[00:07:54] 142 | pub enum OutputType {
[00:07:54]     | ------------------- variant `Body` not found here
[00:07:54] ...
[00:07:54] 201 |             OutputType::Body.shorthand(),
[00:07:54]     |                         ^^^^ variant not found in `session::config::OutputType`
[00:07:54] 
[00:07:54] error[E0599]: no variant named `Body` found for type `session::config::OutputType` in the current scope
[00:07:54]    --> src/librustc/session/config.rs:214:25
[00:07:54] 142 | pub enum OutputType {
[00:07:54] 142 | pub enum OutputType {
[00:07:54]     | ------------------- variant `Body` not found here
[00:07:54] ...
[00:07:54] 214 |             OutputType::Body => "mir",
[00:07:54]     |                         ^^^^ variant not found in `session::config::OutputType`
[00:07:54] 
[00:07:54] error[E0599]: no variant named `Body` found for type `session::config::OutputType` in the current scope
[00:07:54]    --> src/librustc/session/config.rs:281:27
[00:07:54] 142 | pub enum OutputType {
[00:07:54] 142 | pub enum OutputType {
[00:07:54]     | ------------------- variant `Body` not found here
[00:07:54] ...
[00:07:54] 281 |             | OutputType::Body
[00:07:54]     |                           ^^^^ variant not found in `session::config::OutputType`
[00:08:01] error: aborting due to 6 previous errors
[00:08:01] 
[00:08:01] For more information about this error, try `rustc --explain E0599`.
[00:08:01] error: Could not compile `rustc`.
