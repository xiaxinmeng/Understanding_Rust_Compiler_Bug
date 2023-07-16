plain
GITHUB_ENV=/home/runner/work/_temp/_runner_file_commands/set_env_260de518-99e8-4165-87b6-d464ce7b8bec
GITHUB_EVENT_NAME=pull_request
GITHUB_EVENT_PATH=/home/runner/work/_temp/_github_workflow/event.json
GITHUB_GRAPHQL_URL=https://api.github.com/graphql
GITHUB_HEAD_REF=visibility-on-demand
GITHUB_JOB=pr
GITHUB_PATH=/home/runner/work/_temp/_runner_file_commands/add_path_260de518-99e8-4165-87b6-d464ce7b8bec
GITHUB_REF=refs/pull/80099/merge
GITHUB_REPOSITORY_OWNER=rust-lang
GITHUB_RETENTION_DAYS=90
GITHUB_RUN_ID=430882112
GITHUB_RUN_NUMBER=21621
---
Building wheels for collected packages: PyYAML
  Running setup.py bdist_wheel for PyYAML: started
  Running setup.py bdist_wheel for PyYAML: finished with status 'error'
  Failed building wheel for PyYAML
  Complete output from command /usr/bin/python3 -u -c "import setuptools, tokenize;__file__='/tmp/pip-build-34_wpbv7/PyYAML/setup.py';f=getattr(tokenize, 'open', open)(__file__);code=f.read().replace('\r\n', '\n');f.close();exec(compile(code, __file__, 'exec'))" bdist_wheel -d /tmp/tmpjztc5nf_pip-wheel- --python-tag cp36:
     or: -c --help [cmd1 cmd2 ...]
     or: -c --help-commands
     or: -c cmd --help
  
---
.................................................................................................... 9000/11178
.................................................................................................... 9100/11178
......................................................................i......i...................... 9200/11178
.................................................................................................... 9300/11178
..........iiiiii.iiiiii.i........................................................................... 9400/11178
.................................................................................................... 9600/11178
.................................................................................................... 9700/11178
.................................................................................................... 9800/11178
.................................................................................................... 9900/11178
---
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 237 tests
iii............iii...ii..i.ii.........i.ii..........i..............i.............i.i...iii.......iii 100/237
..........i.....i.............i.i.i....ii..iiii......................................ii..i..i....i.. 200/237
............iii.ii...................

 finished in 2.885 seconds
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.065 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 1.97s

 finished in 2.033 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
.............................
test result: ok. 429 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.63s

   Doc-tests alloc
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 528 tests
.................................................................................................... 100/528
.................................................................................................... 200/528
......................i............................................................................. 300/528
---
.................................................................................
test result: ok. 281 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

   Doc-tests core
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 2791 tests
iiiii............................................................................................... 100/2791
...........................................................................................ii....... 200/2791
.................................................................................................... 300/2791
---
 finished in 102.084 seconds
Testing panic_abort stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.14s
   Doc-tests panic_abort
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


Set({"library/panic_unwind"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
 finished in 0.191 seconds
Testing panic_unwind stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.13s
   Doc-tests panic_unwind
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests proc_macro
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
............
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests std
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 1146 tests
i................................................................................................... 100/1146
.................................................................................................... 200/1146
............................iii......i......i...i.........i......................................... 300/1146
---
......i...
test result: ok. 9 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests term
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

---
.............................................
test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

   Doc-tests test
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


Set({"library/unwind"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
 finished in 3.221 seconds
Testing unwind stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling unwind v0.0.0 (/checkout/library/unwind)
    Finished release [optimized] target(s) in 0.15s
   Doc-tests unwind
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
...................
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_apfloat
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
...............
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

   Doc-tests rustc_arena
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_ast_passes
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
 finished in 0.169 seconds
Testing rustc_codegen_ssa stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.16s
   Doc-tests rustc_codegen_ssa
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

---

   Doc-tests rustc_error_codes
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_fs_util
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
...........
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_graphviz
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.51s

---
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_lint
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 48 tests
.......ii.......................................
test result: ok. 46 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.50s

---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_lint_defs
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 77 tests
ii....i......i.........ii......................i.............i...............
test result: ok. 69 passed; 0 failed; 8 ignored; 0 measured; 0 filtered out; finished in 0.63s

---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_llvm
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
.....
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.32s

   Doc-tests rustc_macros
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 4 tests
iiii
test result: ok. 0 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
..............
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_parse_format
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_passes
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_privacy
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
......................
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests rustc_serialize
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.65s

---

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_session
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
..........................................................
test result: ok. 158 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustc_target
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
   Doc-tests rustc_ty_utils

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---
.................................................
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rustdoc
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.06s

---
Rustbook (x86_64-unknown-linux-gnu) - book/2018-edition
[2020-12-18T16:57:36Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Previous versions of mdBook erroneously accepted `./src/theme` as an automatic theme directory
[2020-12-18T16:57:36Z WARN  mdbook::renderer::html_handlebars::hbs_renderer] Please move your theme files to `./theme` for them to continue being used
Documenting standalone (x86_64-unknown-linux-gnu)
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
---
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.9.0
    Checking addr2line v0.14.0
 Documenting std v0.0.0 (/checkout/library/std)
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
   Compiling std v0.0.0 (/checkout/library/std)
 Documenting proc_macro v0.0.0 (/checkout/library/proc_macro)
 Documenting proc_macro v0.0.0 (/checkout/library/proc_macro)
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
    Checking rustc-std-workspace-std v1.99.0 (/checkout/library/rustc-std-workspace-std)
    Checking term v0.0.0 (/checkout/library/term)
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking unicode-width v0.1.8
    Checking unicode-width v0.1.8
    Checking getopts v0.2.21
 Documenting test v0.0.0 (/checkout/library/test)
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
Set({"compiler/rustc"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Documenting stage2 compiler (x86_64-unknown-linux-gnu)
Set({"compiler/rustc_apfloat"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
Set({"compiler/rustc_arena"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
Set({"src/tools/rustdoc-themes"}) not skipped for "bootstrap::test::RustdocTheme" -- not in ["src/tools/tidy"]
Building stage0 tool rustdoc-themes (x86_64-unknown-linux-gnu)
   Compiling rustdoc-themes v0.1.0 (/checkout/src/tools/rustdoc-themes)
    Finished release [optimized] target(s) in 0.54s
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
rustdoc: [check-theme] Starting tests! (Ignoring all other arguments)
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
 - Checking "/checkout/src/librustdoc/html/static/themes/dark.css"... OK
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
 - Checking "/checkout/src/librustdoc/html/static/themes/ayu.css"... OK
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 85 tests
running 85 tests
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFiFFFFFFFFFFFFFFFFFFFFFFFF

---- [ui] rustdoc-ui/check-attr-test.rs stdout ----
---- [ui] rustdoc-ui/check-attr-test.rs stdout ----
thread 'main' panicked at 'I/O failure during tests: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }', src/tools/compiletest/src/main.rs:397:13


+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
+ [src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
1 error: unknown attribute `compile-fail`. Did you mean `compile_fail`?
3   |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-attr-test/check-attr-test.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-attr-test.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/check-attr-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-attr-test" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-attr-test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
[src/librustdoc/lib.rs:106] std::mem::size_of::<Item>() = 584
[src/librustdoc/lib.rs:106] std::mem::size_of::<ItemKind>() = 408
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::Options>() = 1976
[src/librustdoc/lib.rs:106] std::mem::size_of::<config::RenderOptions>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<Import>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Struct>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Union>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Enum>() = 80
[src/librustdoc/lib.rs:106] std::mem::size_of::<Function>() = 328
[src/librustdoc/lib.rs:106] std::mem::size_of::<Module>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Typedef, bool)>() = 248
[src/librustdoc/lib.rs:106] std::mem::size_of::<OpaqueTy>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Static>() = 128
[src/librustdoc/lib.rs:106] std::mem::size_of::<Constant>() = 152
[src/librustdoc/lib.rs:106] std::mem::size_of::<Trait>() = 104
[src/librustdoc/lib.rs:106] std::mem::size_of::<TraitAlias>() = 72
[src/librustdoc/lib.rs:106] std::mem::size_of::<Impl>() = 400
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Function, Option<rustc_hir::Defaultness>)>() = 336
[src/librustdoc/lib.rs:106] std::mem::size_of::<Type>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Variant>() = 40
[src/librustdoc/lib.rs:106] std::mem::size_of::<Macro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<ProcMacro>() = 32
[src/librustdoc/lib.rs:106] std::mem::size_of::<PrimitiveType>() = 1
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Type, Option<String>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<(Vec<GenericBound>, Option<Type>)>() = 120
[src/librustdoc/lib.rs:106] std::mem::size_of::<String>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Span>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_span::Symbol>>() = 4
[src/librustdoc/lib.rs:106] std::mem::size_of::<Attributes>() = 96
[src/librustdoc/lib.rs:106] std::mem::size_of::<Visibility>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<DefId>() = 8
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<Stability>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<rustc_attr::Deprecation>>() = 16
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<ConstStability>>() = 20
[src/librustdoc/lib.rs:106] std::mem::size_of::<Option<String>>() = 24
[src/librustdoc/lib.rs:106] std::mem::size_of::<Box<ItemKind>>() = 8
error: unknown attribute `compile-fail`. Did you mean `compile_fail`?
  |
  |
5 | / /// foo
6 | | ///
7 | | /// 