
Dump of assembler code for function _ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h749e4f0eba59cec2E:
/home/gps/src/pyoxidizer.git/pyembed/src/interpreter_config.rs:
585         fn try_into(self) -> Result<pyffi::PyConfig, Self::Error> {
   0x0000555555a02e30 <+0>:     push   %r15
   0x0000555555a02e32 <+2>:     push   %r14
   0x0000555555a02e34 <+4>:     push   %rbx
   0x0000555555a02e35 <+5>:     sub    $0x4b0,%rsp
   0x0000555555a02e3c <+12>:    mov    %rsi,%r15
   0x0000555555a02e3f <+15>:    mov    %rdi,%r14

586             // We use the raw configuration as a base then we apply any adjustments,
587             // as needed.
588             let mut config: pyffi::PyConfig =
589                 python_interpreter_config_to_py_config(&self.interpreter_config)?;
=> 0x0000555555a02e42 <+18>:    add    $0x30,%rsi
   0x0000555555a02e46 <+22>:    lea    0x198(%rsp),%rdi
   0x0000555555a02e4e <+30>:    call   0x555555a42bd0 <_ZN7pyembed18interpreter_config38python_interpreter_config_to_py_config17h7c09a0c959ab9a73E>

/rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/result.rs:
1636    /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/result.rs: No such file or directory.
   0x0000555555a02e53 <+35>:    mov    0x198(%rsp),%rbx
   0x0000555555a02e5b <+43>:    lea    0x1a0(%rsp),%rsi
   0x0000555555a02e63 <+51>:    lea    0x10(%rsp),%rdi
   0x0000555555a02e68 <+56>:    mov    $0x188,%edx
   0x0000555555a02e6d <+61>:    call   *0x147169d(%rip)        # 0x555556e74510

/home/gps/src/pyoxidizer.git/pyembed/src/interpreter_config.rs:
589                 python_interpreter_config_to_py_config(&self.interpreter_config)?;
   0x0000555555a02e73 <+67>:    cmp    $0x1,%rbx
   0x0000555555a02e77 <+71>:    jne    0x555555a02e92 <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h749e4f0eba59cec2E+98>
   0x0000555555a02e79 <+73>:    movups 0x10(%rsp),%xmm0
   0x0000555555a02e7e <+78>:    movups 0x20(%rsp),%xmm1
   0x0000555555a02e83 <+83>:    movups %xmm1,0x18(%r14)
   0x0000555555a02e88 <+88>:    movups %xmm0,0x8(%r14)
   0x0000555555a02e8d <+93>:    jmp    0x555555a03026 <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h749e4f0eba59cec2E+502>
   0x0000555555a02e92 <+98>:    lea    0x328(%rsp),%rdi
   0x0000555555a02e9a <+106>:   lea    0x10(%rsp),%rsi
   0x0000555555a02e9f <+111>:   mov    $0x188,%edx
   0x0000555555a02ea4 <+116>:   call   *0x1471666(%rip)        # 0x555556e74510

590
591             if let Some(argv) = &self.argv {
   0x0000555555a02eaa <+122>:   mov    0x290(%r15),%rdx
   0x0000555555a02eb1 <+129>:   test   %rdx,%rdx
   0x0000555555a02eb4 <+132>:   je     0x555555a02efe <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h749e4f0eba59cec2E+206>

/rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/alloc/src/vec/mod.rs:
2323    /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/alloc/src/vec/mod.rs: No such file or directory.
   0x0000555555a02eb6 <+134>:   mov    0x2a0(%r15),%rcx
   0x0000555555a02ebd <+141>:   lea    0x198(%rsp),%rdi
   0x0000555555a02ec5 <+149>:   lea    0x328(%rsp),%rsi

/home/gps/src/pyoxidizer.git/pyembed/src/interpreter_config.rs:
592                 set_argv(&mut config, argv)?;
   0x0000555555a02ecd <+157>:   call   0x555555a42790 <_ZN7pyembed18interpreter_config8set_argv17hedd5bbad1f1fa6c0E>

/rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/result.rs:
1636    /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/result.rs: No such file or directory.
   0x0000555555a02ed2 <+162>:   mov    0x198(%rsp),%rax
   0x0000555555a02eda <+170>:   movups 0x1a0(%rsp),%xmm0
   0x0000555555a02ee2 <+178>:   movaps %xmm0,0x10(%rsp)
   0x0000555555a02ee7 <+183>:   mov    0x1b0(%rsp),%rcx
   0x0000555555a02eef <+191>:   mov    %rcx,0x20(%rsp)

/home/gps/src/pyoxidizer.git/pyembed/src/interpreter_config.rs:
592                 set_argv(&mut config, argv)?;
   0x0000555555a02ef4 <+196>:   cmp    $0x2,%rax
   0x0000555555a02ef8 <+200>:   jne    0x555555a0300f <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h749e4f0eba59cec2E+479>

/rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/option.rs:
197     /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/option.rs: No such file or directory.
   0x0000555555a02efe <+206>:   mov    (%r15),%rcx
   0x0000555555a02f01 <+209>:   test   %rcx,%rcx

/home/gps/src/pyoxidizer.git/pyembed/src/interpreter_config.rs:
595             if self.exe.is_none() {
   0x0000555555a02f04 <+212>:   je     0x555555a0302f <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h749e4f0eba59cec2E+511>

/rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/option.rs:
197     /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/option.rs: No such file or directory.
   0x0000555555a02f0a <+218>:   cmpq   $0x0,0x18(%r15)
