
[01:17:05] error[E0432]: unresolved import `crate::config::Config`
[01:17:05]   --> tools/rls/src/actions/mod.rs:17:5
[01:17:05]    |
[01:17:05] 17 | use crate::config::Config;
[01:17:05]    |     ^^^^^^^^^^^^^^^^^^^^^ no `Config` in `config`
[01:17:05]
[01:17:05] error[E0432]: unresolved import `crate::config::Config`
[01:17:05]   --> tools/rls/src/actions/notifications.rs:15:5
[01:17:05]    |
[01:17:05] 15 | use crate::config::Config;
[01:17:05]    |     ^^^^^^^^^^^^^^^^^^^^^ no `Config` in `config`
[01:17:05]
[01:17:05] error[E0432]: unresolved import `crate::config::Config`
[01:17:05]   --> tools/rls/src/build/mod.rs:18:5
[01:17:05]    |
[01:17:05] 18 | use crate::config::Config;
[01:17:05]    |     ^^^^^^^^^^^^^^^^^^^^^ no `Config` in `config`
[01:17:05]
[01:17:05] error[E0432]: unresolved import `crate::config::Config`
[01:17:05]   --> tools/rls/src/build/cargo.rs:24:5
[01:17:05]    |
[01:17:05] 24 | use crate::config::Config;
[01:17:05]    |     ^^^^^^^^^^^^^^^^^^^^^ no `Config` in `config`
[01:17:05]
[01:17:05] error[E0432]: unresolved import `crate::config::Config`
[01:17:05]   --> tools/rls/src/build/rustc.rs:27:39
[01:17:05]    |
[01:17:05] 27 | use crate::config::{ClippyPreference, Config};
[01:17:05]    |                                       ^^^^^^ no `Config` in `config`
[01:17:05]
[01:17:05] error[E0432]: unresolved import `crate::config::Config`
[01:17:05]   --> tools/rls/src/cmd.rs:17:5
[01:17:05]    |
[01:17:05] 17 | use crate::config::Config;
[01:17:05]    |     ^^^^^^^^^^^^^^^^^^^^^ no `Config` in `config`
[01:17:05]
[01:17:05] error[E0432]: unresolved import `crate::config::Config`
[01:17:05]   --> tools/rls/src/server/mod.rs:17:5
[01:17:05]    |
[01:17:05] 17 | use crate::config::Config;
[01:17:05]    |     ^^^^^^^^^^^^^^^^^^^^^ no `Config` in `config`
[01:17:05]
[01:17:05] error: cannot determine resolution for the attribute macro `serde`
[01:17:05]    --> tools/rls/src/config.rs:122:3
[01:17:05]     |
[01:17:05] 122 | #[serde(default)]
[01:17:05]     |   ^^^^^
[01:17:05]     |
[01:17:05]     = note: import resolution is stuck, try simplifying macro imports
[01:17:05]
[01:17:05] error[E0412]: cannot find type `Config` in this scope
[01:17:05]    --> tools/rls/src/config.rs:163:18
[01:17:05]     |
[01:17:05] 163 | impl Default for Config {
[01:17:05]     |                  ^^^^^^ not found in this scope
[01:17:05]
[01:17:05] error[E0412]: cannot find type `Config` in this scope
[01:17:05]    --> tools/rls/src/config.rs:164:21
[01:17:05]     |
[01:17:05] 164 |     fn default() -> Config {
[01:17:05]     |                     ^^^^^^ not found in this scope
[01:17:05]
[01:17:05] error[E0422]: cannot find struct, variant or union type `Config` in this scope
[01:17:05]    --> tools/rls/src/config.rs:165:26
[01:17:05]     |
[01:17:05] 165 |         let mut result = Config {
[01:17:05]     |                          ^^^^^^ not found in this scope
[01:17:05]
[01:17:05] error[E0412]: cannot find type `Config` in this scope
[01:17:05]    --> tools/rls/src/config.rs:195:6
[01:17:05]     |
[01:17:05] 195 | impl Config {
[01:17:05]     |      ^^^^^^ not found in this scope
[01:17:05]
[01:17:05] error[E0412]: cannot find type `Config` in this scope
[01:17:05]    --> tools/rls/src/config.rs:197:39
[01:17:05]     |
[01:17:05] 197 |     pub fn update(&mut self, mut new: Config) {
[01:17:05]     |                                       ^^^^^^ not found in this scope
[01:17:05]
[01:17:05] error: aborting due to 13 previous errors
[01:17:05]
[01:17:05] Some errors occurred: E0412, E0422, E0432.
[01:17:05] For more information about an error, try `rustc --explain E0412`.
[01:17:05] [RUSTC-TIMING] rls test:false 1.585
[01:17:05] error: Could not compile `rls`.
