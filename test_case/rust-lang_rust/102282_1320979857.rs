plain
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error: unused variable: `file`
   --> min_config.rs:237:13
    |
237 | fn get_toml(file: &Path) -> TomlConfig {
    |             ^^^^ help: if this is intentional, prefix it with an underscore: `_file`
    |
    = note: `-D unused-variables` implied by `-D warnings`

error[E0451]: field `minimal_config` of struct `config::Config` is private
   |
   |
21 |         ..Config::parse(&["check".to_owned()])
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `minimal_config` is private

error[E0451]: field `minimal_config` of struct `config::Config` is private
    |
    |
177 |         let config = Config { stage: 0, ..configure("build", &["A"], &["A"]) };
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `minimal_config` is private

error[E0451]: field `minimal_config` of struct `config::Config` is private
    |
    |
194 |         let config = Config { stage: 1, ..configure("build", &["A", "B"], &["A", "B"]) };
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `minimal_config` is private

error[E0451]: field `minimal_config` of struct `config::Config` is private
    |
    |
266 |         Config { stage: 2, ..super::configure("dist", host, target) }
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ field `minimal_config` is private
For more information about this error, try `rustc --explain E0451`.
error: could not compile `bootstrap` due to 5 previous errors
Build completed unsuccessfully in 0:32:36
