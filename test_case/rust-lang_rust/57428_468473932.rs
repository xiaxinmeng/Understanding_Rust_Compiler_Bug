plain
travis_time:end:155ec128:start=1551394400180104192,finish=1551394401330714980,duration=1150610788
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
$ pip install --user awscli; export PATH=$PATH:$HOME/.local/bin:$HOME/Library/Python/2.7/bin/
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/aa/ea/cb62728e9b38f9d8c620d60815f8dd54ca015f6b9af8f5a3d03d9b2e3c64/awscli-1.16.115-py2.py3-none-any.whl (1.4MB)
    1% |▌                               | 20kB 2.1MB/s eta 0:00:01
    2% |▊                               | 30kB 3.1MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
    3% |█▏                              | 51kB 2.5MB/s eta 0:00:01
---
    99% |████████████████████████████████| 542kB 45.4MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 20.1MB/s 
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
Collecting botocore==1.12.105 (from awscli)
  Downloading https://files.pythonhosted.org/packages/cf/ce/acc9013dee20fc94c9b9ae121f5b7b342a206f0d577be1e5c6129811194a/botocore-1.12.105-py2.py3-none-any.whl (5.3MB)
    0% |▏                               | 20kB 22.6MB/s eta 0:00:01
    0% |▏                               | 30kB 27.1MB/s eta 0:00:01
    0% |▎                               | 40kB 28.8MB/s eta 0:00:01
    0% |▎                               | 51kB 29.9MB/s eta 0:00:01
---
[00:06:17]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:31] error[E0432]: unresolved import `crate::ast::TypeBinding`
[00:06:31]   --> src/libsyntax/parse/parser.rs:28:30
[00:06:31]    |
[00:06:31] 28 | use crate::ast::{Ty, TyKind, TypeBinding, GenericBounds};
[00:06:31]    |                              ^^^^^^^^^^^ no `TypeBinding` in `ast`
[00:06:31] 
[00:06:32] error[E0433]: failed to resolve: use of undeclared type or module `AssocTyConstraintKind`
[00:06:32]      |
[00:06:32]      |
[00:06:32] 5968 |                     AssocTyConstraintKind::Equality {
[00:06:32]      |                     ^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `AssocTyConstraintKind`
[00:06:32] 
[00:06:32] error[E0433]: failed to resolve: use of undeclared type or module `AssocTyConstraintKind`
[00:06:32]      |
[00:06:32]      |
[00:06:32] 5972 |                     AssocTyConstraintKind::Bound {
[00:06:32]      |                     ^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `AssocTyConstraintKind`
[00:06:32] error[E0412]: cannot find type `TypeBinding` in this scope
[00:06:32]    --> src/libsyntax/mut_visit.rs:167:44
[00:06:32]     |
[00:06:32]     |
[00:06:32] 167 |     fn visit_ty_binding(&mut self, t: &mut TypeBinding) {
[00:06:32] 
[00:06:32] error[E0422]: cannot find struct, variant or union type `TypeBinding` in this scope
[00:06:32]    --> src/libsyntax/mut_visit.rs:400:45
[00:06:32]     |
[00:06:32]     |
[00:06:32] 400 | pub fn noop_visit_ty_binding<T: MutVisitor>(TypeBinding { id, ident, ty, span }: &mut TypeBinding,
[00:06:32] 
[00:06:32] error[E0412]: cannot find type `TypeBinding` in this scope
[00:06:32]    --> src/libsyntax/mut_visit.rs:400:87
[00:06:32]     |
[00:06:32]     |
[00:06:32] 400 | pub fn noop_visit_ty_binding<T: MutVisitor>(TypeBinding { id, ident, ty, span }: &mut TypeBinding,
[00:06:32] 
[00:06:32] 
[00:06:32] error[E0412]: cannot find type `AssocTyConstraint` in this scope
[00:06:32]      |
[00:06:32]      |
[00:06:32] 5948 |     fn parse_generic_args(&mut self) -> PResult<'a, (Vec<GenericArg>, Vec<AssocTyConstraint>)> {
[00:06:32] help: possible candidate is found in another module, you can import it into scope
[00:06:32]      |
[00:06:32]      |
[00:06:32] 1    | use crate::ast::AssocTyConstraint;
[00:06:32] 
[00:06:32] 
[00:06:32] error[E0422]: cannot find struct, variant or union type `AssocTyConstraint` in this scope
[00:06:32]      |
[00:06:32]      |
[00:06:32] 5979 |                 constraints.push(AssocTyConstraint {
[00:06:32] help: possible candidate is found in another module, you can import it into scope
[00:06:32]      |
[00:06:32]      |
[00:06:32] 1    | use crate::ast::AssocTyConstraint;
[00:06:32] 
[00:06:32] 
[00:06:33] error[E0560]: struct `ast::AngleBracketedArgs` has no field named `bindings`
[00:06:33]    --> src/libsyntax/ast.rs:225:13
[00:06:33] 225 |             bindings: vec![],
[00:06:33] 225 |             bindings: vec![],
[00:06:33]     |             ^^^^^^^^ `ast::AngleBracketedArgs` does not have this field
[00:06:33]     |
[00:06:33]     = note: available fields are: `span`, `args`, `constraints`
[00:06:33] 
[00:06:35] error[E0026]: struct `ast::AngleBracketedArgs` does not have a field named `bindings`
[00:06:35]    --> src/libsyntax/mut_visit.rs:498:36
[00:06:35]     |
[00:06:35] 498 |     let AngleBracketedArgs { args, bindings, span } = data;
[00:06:35]     |                                    ^^^^^^^^ struct `ast::AngleBracketedArgs` does not have this field
[00:06:35] error[E0027]: pattern does not mention field `constraints`
[00:06:35]    --> src/libsyntax/mut_visit.rs:498:9
[00:06:35]     |
[00:06:35]     |
[00:06:35] 498 |     let AngleBracketedArgs { args, bindings, span } = data;
[00:06:35]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing field `constraints`
[00:06:36] error[E0061]: this function takes 1 parameter but 0 parameters were supplied
[00:06:36]     --> src/libsyntax/parse/parser.rs:5973:38
[00:06:36]      |
[00:06:36]      |
[00:06:36] 5619 |     fn parse_generic_bounds(&mut self, colon_span: Option<Span>) -> PResult<'a, GenericBounds> {
[00:06:36]      |     ------------------------------------------------------------------------------------------ defined here
[00:06:36] ...
[00:06:36] 5973 |                         bounds: self.parse_generic_bounds()?,
[00:06:36] 
[00:06:37] error: aborting due to 12 previous errors
[00:06:37] 
[00:06:37] Some errors occurred: E0026, E0027, E0061, E0412, E0422, E0432, E0433, E0560.
---
travis_time:end:07ca3c9c:start=1551394810919918782,finish=1551394810924783474,duration=4864692
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ad8ee16
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04030378
travis_time:start:04030378
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b5601b4
$ dmesg | grep -i kill
