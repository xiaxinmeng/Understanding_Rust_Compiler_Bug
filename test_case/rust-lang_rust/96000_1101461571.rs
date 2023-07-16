
---- builder::tests::dist::dist_with_targets_and_hosts stdout ----
thread 'builder::tests::dist::dist_with_targets_and_hosts' panicked at 'assertion failed: `(left == right)`

Diff < left / right > :
 [
     Rustc {
         compiler: Compiler {
<            stage: 1,
>            stage: 2,
             host: A,
>        },
>    },
>    Rustc {
>        compiler: Compiler {
>            stage: 2,
>            host: B,
         },
     },
 ]

', src/bootstrap/builder/tests.rs:314:9
