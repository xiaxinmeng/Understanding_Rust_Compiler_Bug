plain
    Checking annotate-snippets v0.9.1
    Checking env_logger v0.9.0
    Checking ignore v0.4.17
    Checking thiserror v1.0.30
error: use of deprecated unit variant `clap::ArgAction::StoreValue`: Replaced with `ArgAction::Set` or `ArgAction::Append`
   |
53 |     skip_prefix: u32,
   |     ^^^^^^^^^^^
   |
   |
note: the lint level is defined here
  --> src/tools/rustfmt/src/format-diff/main.rs:5:9
   |
5  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(deprecated)]` implied by `#[deny(warnings)]`

error: use of deprecated unit variant `clap::ArgAction::StoreValue`: Replaced with `ArgAction::Set` or `ArgAction::Append`
   |
62 |     filter: String,
   |     ^^^^^^


error: use of deprecated unit variant `clap::ArgAction::StoreValue`: Replaced with `ArgAction::Set` or `ArgAction::Append`
   |
49 |     packages: Vec<String>,
   |     ^^^^^^^^
   |
   |
note: the lint level is defined here
  --> src/tools/rustfmt/src/cargo-fmt/main.rs:3:9
   |
3  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(deprecated)]` implied by `#[deny(warnings)]`

error: use of deprecated unit variant `clap::ArgAction::StoreValue`: Replaced with `ArgAction::Set` or `ArgAction::Append`
   |
53 |     manifest_path: Option<String>,
   |     ^^^^^^^^^^^^^


error: use of deprecated unit variant `clap::ArgAction::StoreValue`: Replaced with `ArgAction::Set` or `ArgAction::Append`
   |
57 |     message_format: Option<String>,
   |     ^^^^^^^^^^^^^^


error: use of deprecated unit variant `clap::ArgAction::StoreValue`: Replaced with `ArgAction::Set` or `ArgAction::Append`
   |
62 |     rustfmt_options: Vec<String>,
   |     ^^^^^^^^^^^^^^^


error: use of deprecated associated function `clap::Arg::<'help>::validator`: Replaced with `Arg::value_parser(...)`
   |
53 |     skip_prefix: u32,
   |     ^^^^^^^^^^^


error: use of deprecated associated function `clap::Arg::<'help>::validator`: Replaced with `Arg::value_parser(...)`
   |
62 |     filter: String,
   |     ^^^^^^


error: use of deprecated associated function `clap::ArgMatches::is_present`: Replaced with either `ArgAction::SetTrue` or `ArgMatches::contains_id(...)`
   |
32 |     quiet: bool,
   |            ^^^^


error: use of deprecated associated function `clap::ArgMatches::is_present`: Replaced with either `ArgAction::SetTrue` or `ArgMatches::contains_id(...)`
   |
36 |     verbose: bool,
   |              ^^^^


error: use of deprecated associated function `clap::ArgMatches::is_present`: Replaced with either `ArgAction::SetTrue` or `ArgMatches::contains_id(...)`
   |
40 |     version: bool,
   |              ^^^^


error: use of deprecated associated function `clap::ArgMatches::is_present`: Replaced with either `ArgAction::SetTrue` or `ArgMatches::contains_id(...)`
   |
66 |     format_all: bool,
   |                 ^^^^


error: use of deprecated associated function `clap::ArgMatches::is_present`: Replaced with either `ArgAction::SetTrue` or `ArgMatches::contains_id(...)`
   |
70 |     check: bool,
   |            ^^^^


error: use of deprecated associated function `clap::Arg::<'help>::multiple_occurrences`: Replaced with `Arg::action` (Issue #3772)
   |
49 |     packages: Vec<String>,
   |               ^^^


error: use of deprecated associated function `clap::Arg::<'help>::validator`: Replaced with `Arg::value_parser(...)`
   |
49 |     packages: Vec<String>,
   |     ^^^^^^^^


error: use of deprecated associated function `clap::Arg::<'help>::validator`: Replaced with `Arg::value_parser(...)`
   |
53 |     manifest_path: Option<String>,
   |     ^^^^^^^^^^^^^


error: use of deprecated associated function `clap::Arg::<'help>::validator`: Replaced with `Arg::value_parser(...)`
   |
57 |     message_format: Option<String>,
   |     ^^^^^^^^^^^^^^


error: use of deprecated associated function `clap::Arg::<'help>::multiple_occurrences`: Replaced with `Arg::action` (Issue #3772)
   |
62 |     rustfmt_options: Vec<String>,
   |                      ^^^


error: use of deprecated associated function `clap::Arg::<'help>::validator`: Replaced with `Arg::value_parser(...)`
   |
62 |     rustfmt_options: Vec<String>,
   |     ^^^^^^^^^^^^^^^

