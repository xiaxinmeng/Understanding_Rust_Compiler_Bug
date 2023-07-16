plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 9'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200517.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200517.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cd5c4646-1978-4a2b-ba54-1ba81af20674.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72906/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72906/merge:refs/remotes/pull/72906/merge
---
 ---> 3adb0605cc65
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 28dbc326cb7f
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 537a01811900
Successfully built 537a01811900
Successfully tagged rust-ci:latest
Built container sha256:537a018119009dc218456238dec90b5530050db1e2a1e166550c218003f6159d
---
    Checking alloc v0.0.0 (/checkout/src/liballoc)
error: this file contains an unclosed delimiter
    --> src/liballoc/rc.rs:2121:3
     |
2021 | trait RcBoxPtr<T: ?Sized> {
...
...
2030 |     fn inc_strong(&self) {
...
...
2037 |         if strong == 0 || strong == usize::MAX {
...
2040 |     }
     |     - ...as it matches this but it has different indentation
...
---

error: `self` parameter is only allowed in associated functions
    --> src/liballoc/rc.rs:2048:13
     |
2048 |     fn weak(&self) -> usize {
     |
     = note: associated functions are those in `impl` or `trait` definitions

error: `self` parameter is only allowed in associated functions
error: `self` parameter is only allowed in associated functions
    --> src/liballoc/rc.rs:2053:17
     |
2053 |     fn inc_weak(&self) {
     |
     = note: associated functions are those in `impl` or `trait` definitions

error: `self` parameter is only allowed in associated functions
error: `self` parameter is only allowed in associated functions
    --> src/liballoc/rc.rs:2066:17
     |
2066 |     fn dec_weak(&self) {
     |
     = note: associated functions are those in `impl` or `trait` definitions

error[E0401]: can't use generic parameters from outer function
---

error[E0401]: can't use generic parameters from outer function
    --> src/liballoc/rc.rs:2048:13
     |
2048 |     fn weak(&self) -> usize {
     |             |
     |             use of generic parameter from outer function
     |             can't use `Self` here


error[E0401]: can't use generic parameters from outer function
    --> src/liballoc/rc.rs:2053:17
     |
2053 |     fn inc_weak(&self) {
     |                 |
     |                 use of generic parameter from outer function
     |                 can't use `Self` here


error[E0401]: can't use generic parameters from outer function
    --> src/liballoc/rc.rs:2066:17
     |
2066 |     fn dec_weak(&self) {
     |                 |
     |                 use of generic parameter from outer function
     |                 can't use `Self` here


error[E0425]: cannot find function `data_offset` in this scope
   --> src/liballoc/rc.rs:602:26
    |
602 |             let offset = data_offset(&(*ptr).value);
    |
help: consider importing this function
    |
233 | use crate::sync::data_offset;
---
    Checking rustc-demangle v0.1.16
error[E0599]: no method named `dec_strong` found for struct `rc::Rc<T>` in the current scope
   --> src/liballoc/rc.rs:430:22
    |
285 | pub struct Rc<T: ?Sized> {
    | ------------------------ method `dec_strong` not found for this
...
430 |                 this.dec_strong();
    |                      ^^^^^^^^^^ method not found in `rc::Rc<T>`

error[E0599]: no method named `inc_weak` found for reference `&rc::Rc<T>` in the current scope
   --> src/liballoc/rc.rs:693:14
    |
693 |         this.inc_weak();
    |              ^^^^^^^^ method not found in `&rc::Rc<T>`
error[E0599]: no method named `weak` found for reference `&rc::Rc<T>` in the current scope
   --> src/liballoc/rc.rs:716:14
    |
    |
716 |         this.weak() - 1
    |              ^^^^ method not found in `&rc::Rc<T>`
error[E0599]: no method named `dec_strong` found for struct `rc::Rc<T>` in the current scope
   --> src/liballoc/rc.rs:896:22
    |
    |
285 | pub struct Rc<T: ?Sized> {
    | ------------------------ method `dec_strong` not found for this
...
896 |                 swap.dec_strong();
    |                      ^^^^^^^^^^ method not found in `rc::Rc<T>`

error[E0599]: no method named `dec_weak` found for struct `rc::Rc<T>` in the current scope
   --> src/liballoc/rc.rs:899:22
    |
285 | pub struct Rc<T: ?Sized> {
    | ------------------------ method `dec_weak` not found for this
...
899 |                 swap.dec_weak();
    |                      ^^^^^^^^ method not found in `rc::Rc<T>`
error[E0599]: no method named `dec_strong` found for mutable reference `&mut rc::Rc<T>` in the current scope
    --> src/liballoc/rc.rs:1146:18
     |
1146 |             self.dec_strong();
1146 |             self.dec_strong();
     |                  ^^^^^^^^^^ method not found in `&mut rc::Rc<T>`

error[E0599]: no method named `dec_weak` found for mutable reference `&mut rc::Rc<T>` in the current scope
    --> src/liballoc/rc.rs:1153:22
     |
1153 |                 self.dec_weak();
     |                      ^^^^^^^^ method not found in `&mut rc::Rc<T>`
error[E0599]: no method named `weak` found for mutable reference `&mut rc::Rc<T>` in the current scope
    --> src/liballoc/rc.rs:1155:25
     |
     |
1155 |                 if self.weak() == 0 {
     |                         ^^^^ method not found in `&mut rc::Rc<T>`

error[E0599]: no method named `weak` found for reference `&rc::RcBox<T>` in the current scope
    --> src/liballoc/rc.rs:1858:27
     |
1858 |                     inner.weak() - 1 // subtract the implicit weak ptr
     |                           |
     |                           field, not a method


error[E0599]: no method named `dec_weak` found for reference `&rc::RcBox<T>` in the current scope
    --> src/liballoc/rc.rs:1949:19
     |
1949 |             inner.dec_weak();
     |                   ^^^^^^^^ method not found in `&rc::RcBox<T>`

error[E0599]: no method named `weak` found for reference `&rc::RcBox<T>` in the current scope
    --> src/liballoc/rc.rs:1952:22
     |
1952 |             if inner.weak() == 0 {
     |                      |
     |                      field, not a method


error[E0599]: no method named `inc_weak` found for reference `&rc::RcBox<T>` in the current scope
    --> src/liballoc/rc.rs:1977:19
     |
1977 |             inner.inc_weak()
     |                   ^^^^^^^^ method not found in `&rc::RcBox<T>`
error: unreachable statement
    --> src/liballoc/rc.rs:2039:9
     |
2038 |             abort();
2038 |             abort();
     |             ------- any code following this expression is unreachable
2039 |         self.inner().strong.set(strong + 1);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
     = note: `-D unreachable-code` implied by `-D warnings`

error: unreachable statement
    --> src/liballoc/rc.rs:2062:9
    --> src/liballoc/rc.rs:2062:9
     |
2061 |             abort();
     |             ------- any code following this expression is unreachable
2062 |         self.inner().weak.set(weak + 1);
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
    Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
error: aborting due to 27 previous errors

Some errors have detailed explanations: E0401, E0425, E0599.
---
  local time: Wed Jun 10 01:34:15 UTC 2020
  network time: Wed, 10 Jun 2020 01:34:15 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72906/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72906/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3557) (python)
##[section]Finishing: Finalize Job
