plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0425]: cannot find value `state` in this scope
   --> compiler/rustc_mir_dataflow/src/value_analysis.rs:231:17
    |
231 |                 state.flood_with(Place::from(*local).as_ref(), self.map(), Self::Value::bottom());
...
...
417 | pub struct State<V>(StateData<V>);
    | ---------------------------------- similarly named tuple struct `State` defined here
help: a tuple struct with a similar name exists
    |
    |
231 |                 State.flood_with(Place::from(*local).as_ref(), self.map(), Self::Value::bottom());
help: consider importing one of these items
    |
35  | use crate::sym::state;
    |
    |
35  | use rustc_span::sym::state;
    |

error[E0425]: cannot find value `local` in this scope
   --> compiler/rustc_mir_dataflow/src/value_analysis.rs:231:47
    |
231 |                 state.flood_with(Place::from(*local).as_ref(), self.map(), Self::Value::bottom());
    |
help: consider importing one of these items
    |
35  | use crate::sym::local;
