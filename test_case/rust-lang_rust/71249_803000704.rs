
warning: the following crates contain code that will be rejected by a future version of Rust: colored v1.1.0, lazy_static v1.1.0

These crates are in your dependency tree because:

   ... some nice cargo-tree like output or something like that ...


To solve this problem, you can try the following things:

- Automatically update your dependencies to the latest compatible version:
    - `cargo update -p colored` 
    - `cargo update -p lazy_static`
 
- If a minor dependency update does not help, you can try updating to a new 
  major version of those dependencies. You have to do this manually.
    - colored could be updated from 1.1.0 to 2.0.0

- If the issue is not solved by updating the dependencies, a fix has to be
  implemented by those dependencies. You can help with that by notifying the
  maintainers of this problem (e.g. by creating a bug report) or by proposing a
  fix to the maintainers (e.g. by creating a pull request).
    - colored
        - Repository: https://github.com/mackwic/colored
        - Detailed warning: cargo describe-future-incompatibilities --id abc
    - lazy_static: 
        - Repository: https://github.com/rust-lang-nursery/lazy-static.rs
        - Detailed warning: cargo describe-future-incompatibilities --id def

- If waiting for an upstream fix is not an option, you can use the `[patch]`
  section in `Cargo.toml` to use your own version of the dependency. For more
  information, see:
    https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section

- Finally, to simply silence this warning, you can ... (something something 
  `future_incompatibility_report.frequency = "weekly"`; link to "Annoyance 
  modulation" docs).
