plain
travis_time:end:0ce7c7f2:start=1546020317087783815,finish=1546020374232458875,duration=57144675060
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:01:56] ................................................................i................................... 4400/5202
[01:02:02] .................................................................................................... 4500/5202
[01:02:05] .................................................................................................... 4600/5202
[01:02:08] .................................................................................................... 4700/5202
[01:02:12] .................F.................................................................................. 4800/5202
[01:02:19] .................................................................................................... 5000/5202
[01:02:19] .................................................................................................... 5000/5202
xt":[{"text":"  opt.and_then(|arg| Some(takes_ref(arg)));","highlight_start":37,"highlight_end":40}],"label":"expected &Foo, found struct `Foo`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `&Foo`\n   found type `Foo`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `as_ref` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/suggestions/as-ref.rs","byte_start":154,"byte_end":154,"line_start":8,"line_end":8,"column_start":7,"column_end":7,"is_primary":true,"text":[{"text":"  opt.and_then(|arg| Some(takes_ref(arg)));","highlight_start":7,"highlight_end":7}],"label":null,"suggested_replacement":"as_ref().","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/suggestions/as-ref.rs:8:37\n   |\nLL |   opt.and_then(|arg| Some(takes_ref(arg)));\n   |       -                             ^^^ expected &Foo, found struct `Foo`\n   |       |\n   |       help: consider using `as_ref` instead: `as_ref().`\n   |\n   = note: expected type `&Foo`\n              found type `Foo`\n\n"}
[01:02:24] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor exam
