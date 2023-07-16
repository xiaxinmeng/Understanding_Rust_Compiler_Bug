
/home/gps/src/pyoxidizer.git/pyembed/src/interpreter_config.rs:
585         fn try_into(self) -> Result<pyffi::PyConfig, Self::Error> {
   0x0000555555a0cff0 <+0>:     push   %r15
   0x0000555555a0cff2 <+2>:     push   %r14
   0x0000555555a0cff4 <+4>:     push   %r12
   0x0000555555a0cff6 <+6>:     push   %rbx
   0x0000555555a0cff7 <+7>:     sub    $0x4a8,%rsp
   0x0000555555a0cffe <+14>:    mov    %rsi,%rbx
   0x0000555555a0d001 <+17>:    mov    %rdi,%r14

586             // We use the raw configuration as a base then we apply any adjustments,
587             // as needed.
588             let mut config: pyffi::PyConfig =
589                 python_interpreter_config_to_py_config(&self.interpreter_config)?;
=> 0x0000555555a0d004 <+20>:    add    $0x30,%rsi
   0x0000555555a0d008 <+24>:    lea    0x8(%rsp),%rdi
   0x0000555555a0d00d <+29>:    call   0x555555a4cd00 <_ZN7pyembed18interpreter_config38python_interpreter_config_to_py_config17h8fe88207c7bebad5E>

/rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs:
1664    /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs: No such file or directory.
   0x0000555555a0d012 <+34>:    cmpl   $0x1,0x8(%rsp)
   0x0000555555a0d017 <+39>:    lea    0x10(%rsp),%rsi
   0x0000555555a0d01c <+44>:    jne    0x555555a0d04b <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h3c0366d657e57554E+91>

1665    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
   0x0000555555a0d01e <+46>:    movups (%rsi),%xmm0
   0x0000555555a0d021 <+49>:    movups 0x10(%rsi),%xmm1
   0x0000555555a0d025 <+53>:    movaps %xmm1,0x330(%rsp)
   0x0000555555a0d02d <+61>:    movaps %xmm0,0x320(%rsp)

1666    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
1667    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
1668    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
1669    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
1670    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
1671    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
1672    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
1673    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
1674    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
1675    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
   0x0000555555a0d035 <+69>:    movups %xmm1,0x18(%r14)
   0x0000555555a0d03a <+74>:    movups %xmm0,0x8(%r14)
   0x0000555555a0d03f <+79>:    movq   $0x1,(%r14)
   0x0000555555a0d046 <+86>:    jmp    0x555555a0d1cb <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h3c0366d657e57554E+475>
   0x0000555555a0d04b <+91>:    lea    0x320(%rsp),%r15
   0x0000555555a0d053 <+99>:    mov    0x1464446(%rip),%r12        # 0x555556e714a0

1664    in /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs
   0x0000555555a0d05a <+106>:   mov    $0x188,%edx
   0x0000555555a0d05f <+111>:   mov    %r15,%rdi
   0x0000555555a0d062 <+114>:   call   *%r12
   0x0000555555a0d065 <+117>:   lea    0x198(%rsp),%rdi

/home/gps/src/pyoxidizer.git/pyembed/src/interpreter_config.rs:
589                 python_interpreter_config_to_py_config(&self.interpreter_config)?;
   0x0000555555a0d06d <+125>:   mov    $0x188,%edx
   0x0000555555a0d072 <+130>:   mov    %r15,%rsi
   0x0000555555a0d075 <+133>:   call   *%r12

590
591             if let Some(argv) = &self.argv {
   0x0000555555a0d078 <+136>:   mov    0x290(%rbx),%rdx
   0x0000555555a0d07f <+143>:   test   %rdx,%rdx
   0x0000555555a0d082 <+146>:   je     0x555555a0d0ac <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h3c0366d657e57554E+188>

/rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/alloc/src/vec/mod.rs:
2366    /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/alloc/src/vec/mod.rs: No such file or directory.
   0x0000555555a0d084 <+148>:   mov    0x2a0(%rbx),%rcx
   0x0000555555a0d08b <+155>:   lea    0x8(%rsp),%rdi
   0x0000555555a0d090 <+160>:   lea    0x198(%rsp),%rsi

/home/gps/src/pyoxidizer.git/pyembed/src/interpreter_config.rs:
592                 set_argv(&mut config, argv)?;
   0x0000555555a0d098 <+168>:   call   0x555555a4c8c0 <_ZN7pyembed18interpreter_config8set_argv17hbfb1f04fcddc907fE>

/rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs:
1664    /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/result.rs: No such file or directory.
   0x0000555555a0d09d <+173>:   mov    0x8(%rsp),%rax
   0x0000555555a0d0a2 <+178>:   cmp    $0x2,%rax
   0x0000555555a0d0a6 <+182>:   jne    0x555555a0d178 <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h3c0366d657e57554E+392>

/rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/option.rs:
199     /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/option.rs: No such file or directory.
   0x0000555555a0d0ac <+188>:   mov    (%rbx),%rcx
   0x0000555555a0d0af <+191>:   test   %rcx,%rcx

/home/gps/src/pyoxidizer.git/pyembed/src/interpreter_config.rs:
595             if self.exe.is_none() {
   0x0000555555a0d0b2 <+194>:   je     0x555555a0d198 <_ZN7pyembed18interpreter_config161_$LT$impl$u20$core..convert..TryInto$LT$pyo3..ffi..cpython..initconfig..PyConfig$GT$$u20$for$u20$$RF$pyembed..config..ResolvedOxidizedPythonInterpreterConfig$GT$8try_into17h3c0366d657e57554E+424>

/rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/option.rs:
199     /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/option.rs: No such file or directory.
   0x0000555555a0d0b8 <+200>:   cmpq   $0x0,0x18(%rbx)

