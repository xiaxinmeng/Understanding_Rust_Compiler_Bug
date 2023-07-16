
% cargo +nightly build
   Compiling proc-macro2 v1.0.51
   Compiling unicode-ident v1.0.6
   Compiling quote v1.0.23
   Compiling syn v1.0.107
   Compiling stable_deref_trait v1.2.0
   Compiling rental-impl v0.5.5
   Compiling rental v0.5.5
   Compiling exercise_issue_106060 v0.1.0 (/tmp/exercise_issue_106060)
    Finished dev [unoptimized + debuginfo] target(s) in 5.76s
warning: the following packages contain code that will be rejected by a future version of Rust: rental v0.5.5
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
% cargo +nightly build --future-incompat-report
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
warning: the following packages contain code that will be rejected by a future version of Rust: rental v0.5.5
note:
To solve this problem, you can try the following approaches:


- Some affected dependencies have newer versions available.
You may want to consider updating them to a newer version to see if the issue has been fixed.

rental v0.5.5 has the following newer versions available: 0.5.6


- If the issue is not solved by updating the dependencies, a fix has to be
implemented by those dependencies. You can help with that by notifying the
maintainers of this problem (e.g. by creating a bug report) or by proposing a
fix to the maintainers (e.g. by creating a pull request):

  - rental@0.5.5
  - Repository: https://github.com/jpernst/rental
  - Detailed warning command: `cargo report future-incompatibilities --id 2 --package rental@0.5.5`

- If waiting for an upstream fix is not an option, you can use the `[patch]`
section in `Cargo.toml` to use your own version of the dependency. For more
information, see:
https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section

note: this report can be shown with `cargo report future-incompatibilities --id 1`
