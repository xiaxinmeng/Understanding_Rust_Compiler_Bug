plain
travis_time:end:018f64c8:start=1549628228673701586,finish=1549628232019192019,duration=3345490433
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:55:47]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:56:03] warning: `[std::str]` cannot be resolved, ignoring it...
[00:56:03]  --> src/libcore/str/mod.rs:3:32
[00:56:03]   |
[00:56:03] 3 | //! For more details, see the [`std::str`] module.
[00:56:03]   |
[00:56:03] note: lint level defined here
[00:56:03]  --> src/libcore/lib.rs:63:9
[00:56:03]   |
---
[00:59:55] .................................................................................................... 3800/5376
[00:59:57] .............i...................................................................................... 3900/5376
[00:59:59] ......................................................................i............................. 4000/5376
[01:00:01] .................................................................................................... 4100/5376
[01:00:08] .............................................F...................................................... 4200/5376
[01:00:18] .................................................................................................... 4400/5376
[01:00:21] .................................................................................................... 4500/5376
[01:00:26] ..........................i......................................................................... 4600/5376
[01:00:31] .................................................................................................... 4700/5376
---
[01:00:50] .................................................................................................... 5300/5376
ce
[01:00:52] -    |         ^^^^^^^^^^^^^^^^^^ can't leak private type
[01:00:52] - 
[01:00:52] 309 error: private trait `aliases_priv::PrivTr1` in public interface (error E0445)
[01:00:52] +   --> $DIR/private-in-public-warn.rs:240:5
[01:00:52] 311    |
[01:00:52] 311    |
[01:00:52] 312 LL |     pub trait Tr1: PrivUseAliasTr {}
[01:00:52] 
[01:00:52] 316    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[01:00:52] 317 
[01:00:52] 317 
[01:00:52] 318 error: private trait `aliases_priv::PrivTr1<aliases_priv::Priv2>` in public interface (error E0445)
[01:00:52] +   --> $DIR/private-in-public-warn.rs:243:5
[01:00:52] 320    |
[01:00:52] 320    |
[01:00:52] 321 LL |     pub trait Tr2: PrivUseAliasTr<PrivAlias> {}
[01:00:52] 
[01:00:52] 325    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
[01:00:52] 326 
[01:00:52] 326 
[01:00:52] 327 error: private type `aliases_priv::Priv2` in public interface (error E0446)
[01:00:52] +   --> $DIR/private-in-public-warn.rs:243:5
[01:00:52] 329    |
[01:00:52] 329    |
[01:00:52] 330 LL |     pub trait Tr2: PrivUseAliasTr<PrivAlias> {}
[01:00:52] 
[01:00:52] 342    = note: #[warn(type_alias_bounds)] on by default
[01:00:52] 342    = note: #[warn(type_alias_bounds)] on by default
[01:00:52] 343    = help: the bound will not be checked when the type alias is used, and should be removed
[01:00:52] - warning: where clauses are not enforced in type aliases
[01:00:52] - warning: where clauses are not enforced in type aliases
[01:00:52] + warning: where-clauses are not enforced in type aliases
[01:00:52] 347    |
[01:00:52] 347    |
[01:00:52] 348 LL |     pub type Alias<T> where T: PrivTr = T;
[01:00:52] 350    |
[01:00:52] 350    |
[01:00:52] 351    = help: the clause will not be checked when the type alias is used, and should be removed
[01:00:52] - error: aborting due to 36 previous errors
[01:00:52] + error: aborting due to 35 previous errors
[01:00:52] 354 
[01:00:52] 355 For more information about this error, try `rustc --explain E0446`.
[01:00:52] 355 For more information about this error, try `rustc --explain E0446`.
[01:00:52] 356 
[01:00:52] 
[01:00:52] 
[01:00:52] The actual stderr differed from the expected stderr.
[01:00:52] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-warn/private-in-public-warn.stderr
[01:00:52] To update references, rerun the tests and pass the `--bless` flag
[01:00:52] To only update this specific test, also pass `--test-args privacy/private-in-public-warn.rs`
[01:00:52] error: 1 errors occurred comparing output.
[01:00:52] status: exit code: 1
[01:00:52] status: exit code: 1
[01:00:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-in-public-warn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-warn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-in-public-warn/auxiliary" "-A" "unused"
[01:00:52] ------------------------------------------
[01:00:52] 
[01:00:52] ------------------------------------------
[01:00:52] stderr:
[01:00:52] stderr:
[01:00:52] ------------------------------------------
[01:00:52] {"message":"private type `types::Priv` in public interface (error E0446)","code":{"code":"private_in_public","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/privacy/private-in-public-warn.rs","byte_start":348,"byte_end":370,"line_start":15,"line_end":15,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    pub type Alias = Priv; //~ ERROR private type `types::Priv` in public interface","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/privacy/private-in-public-warn.rs","byte_start":198,"byte_end":215,"line_start":5,"line_end":5,"column_start":9,"column_end":26,"is_primary":true,"text":[{"text":"#![deny(private_in_public)]","highlight_start":9,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: private type `types::Priv` in public interface (error E0446)\n  --> /checkout/src/test/ui/privacy/private-in-public-warn.rs:15:5\n   |\nLL |     pub type Alias = Priv; //~ ERROR private type `types::Priv` in public interface\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/privacy/private-in-public-warn.rs:5:9\n   |\nLL | #![deny(private_in_public)]\n   |         ^^^^^^^^^^^^^^^^^\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>\n\n"}
[01:00:52] {"message":"private type `types::Priv` in public interface (error E0446)","code":{"code":"private_in_public","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/privacy/private-in-public-warn.rs","byte_start":484,"byte_end":488,"line_start":18,"line_end":18,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"        V1(Priv), //~ ERROR private type `types::Priv` in public interface","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: private type `types::Priv` in public interface (error E0446)\n  --> /checkout/src/test/ui/privacy/private-in-public-warn.rs:18:12\n   |\nLL |         V1(Priv), //~ ERROR private type `types::Priv` in public interface\n   |            ^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>\n\n"}
[01:00:52] {"message":"private type `types::Priv` in public interface (error E0446)","code":{"code":"private_in_public","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/privacy/private-in-public-warn.rs","byte_start":593,"byte_end":604,"line_start":20,"line_end":20,"column_start":14,"column_end":25,"is_primary":true,"text":[{"text":"        V2 { field: Priv }, //~ ERROR private type `types::Priv` in public interface","highlight_start":14,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: private type `types::Priv` in public interface (error E0446)\n  --> /checkout/src/test/ui/privacy/private-in-public-warn.rs:20:14\n   |\nLL |         V2 { field: Priv }, //~ ERROR private type `types::Priv` in public interface\n   |              ^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>\n\n"}
[01:00:52] {"message":"private type `types::Priv` in public interface (error E0446)","code":{"code":"private_in_public","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/privacy/private-in-public-warn.rs","byte_start":730,"byte_end":751,"line_start":24,"line_end":24,"column_start":9,"column_end":30,"is_primary":true,"text":[{"text":"        const C: Priv = Priv; //~ ERROR private type `types::Priv` in public interface","highlight_start":9,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: private type `types::Priv` in public interface (error E0446)\n  --> /checkout/src/test/ui/privacy/private-in-public-warn.rs:24:9\n   |\nLL |         const C: Priv = Priv; //~ ERROR private type `types::Priv` in public interface\n   |         ^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>\n\n"}
[01:00:52] {"message":"private type `types::Priv` in public interface","code":{"code":"E0446","explanation":"\nA private type was used in a public type signature. Erroneous code example:\n\n