asm
std::rt::lang_start: # @std::rt::lang_start
# %bb.0:
	subq	$56, %rsp
	leaq	.L__unnamed_1(%rip), %rax
	movq	%rdi, 24(%rsp)
	movq	%rsi, 32(%rsp)
	movq	%rdx, 40(%rsp)
	movq	24(%rsp), %rdx
	movq	%rdx, 48(%rsp)
	leaq	48(%rsp), %rdx
	movq	32(%rsp), %rsi
	movq	40(%rsp), %rcx
	movq	%rdx, %rdi
	movq	%rsi, 16(%rsp)          # 8-byte Spill
	movq	%rax, %rsi
	movq	16(%rsp), %rdx          # 8-byte Reload
	callq	*std::rt::lang_start_internal@GOTPCREL(%rip)
	movq	%rax, 8(%rsp)           # 8-byte Spill
# %bb.1:
	movq	8(%rsp), %rax           # 8-byte Reload
	addq	$56, %rsp
	retq
                                        # -- End function

std::rt::lang_start::{{closure}}: # @"std::rt::lang_start::{{closure}}"
# %bb.0:
	subq	$24, %rsp
	movq	%rdi, 16(%rsp)
	movq	16(%rsp), %rdi
	callq	*(%rdi)
# %bb.1:
	callq	<() as std::process::Termination>::report
	movl	%eax, 12(%rsp)          # 4-byte Spill
# %bb.2:
	movl	12(%rsp), %eax          # 4-byte Reload
	addq	$24, %rsp
	retq
                                        # -- End function

std::sys::unix::process::process_common::ExitCode::as_i32: # @std::sys::unix::process::process_common::ExitCode::as_i32
# %bb.0:
	pushq	%rax
	movq	%rdi, (%rsp)
	movq	(%rsp), %rdi
	movzbl	(%rdi), %eax
	popq	%rcx
	retq
                                        # -- End function

core::ops::function::FnOnce::call_once{{vtable.shim}}: # @"core::ops::function::FnOnce::call_once{{vtable.shim}}"
# %bb.0:
	subq	$24, %rsp
	movq	%rdi, 8(%rsp)
	movq	8(%rsp), %rdi
	movq	(%rdi), %rdi
	callq	core::ops::function::FnOnce::call_once
	movl	%eax, 4(%rsp)           # 4-byte Spill
# %bb.1:
	movl	4(%rsp), %eax           # 4-byte Reload
	addq	$24, %rsp
	retq
                                        # -- End function

core::ops::function::FnOnce::call_once: # @core::ops::function::FnOnce::call_once
# %bb.0:
	subq	$40, %rsp
	movq	%rdi, 8(%rsp)
	leaq	8(%rsp), %rdi
	callq	std::rt::lang_start::{{closure}}
	movl	%eax, 4(%rsp)           # 4-byte Spill
	jmp	.LBB4_1

.LBB4_1:
	jmp	.LBB4_2

.LBB4_2:
	movl	4(%rsp), %eax           # 4-byte Reload
	addq	$40, %rsp
	retq

.LBB4_3:
	jmp	.LBB4_4

.LBB4_4:
	movq	24(%rsp), %rdi
	callq	_Unwind_Resume@PLT
	ud2
	movl	%edx, %ecx
	movq	%rax, 24(%rsp)
	movl	%ecx, 32(%rsp)
	jmp	.LBB4_3
                                        # -- End function

core::ptr::real_drop_in_place: # @core::ptr::real_drop_in_place
# %bb.0:
	pushq	%rax
	movq	%rdi, (%rsp)
	popq	%rax
	retq
                                        # -- End function
<() as std::process::Termination>::report: # @"<() as std::process::Termination>::report"
# %bb.0:
	subq	$24, %rsp
	xorl	%edi, %edi
	callq	<std::process::ExitCode as std::process::Termination>::report
	movl	%eax, 12(%rsp)          # 4-byte Spill
# %bb.1:
	movl	12(%rsp), %eax          # 4-byte Reload
	addq	$24, %rsp
	retq
                                        # -- End function

<std::process::ExitCode as std::process::Termination>::report: # @"<std::process::ExitCode as std::process::Termination>::report"
# %bb.0:
	pushq	%rax
	movb	%dil, %al
	movb	%al, 7(%rsp)
	leaq	7(%rsp), %rdi
	#DEBUG_VALUE: report:self <- [$rdi+0]
	callq	std::sys::unix::process::process_common::ExitCode::as_i32
	movl	%eax, (%rsp)            # 4-byte Spill
# %bb.1:
	movl	(%rsp), %eax            # 4-byte Reload
	popq	%rcx
	retq
                                        # -- End function

playground::main: # @playground::main
# %bb.0:
	subq	$1, %rsp
	xorl	%eax, %eax
	movl	%eax, %ecx
	movb	$1, (%rsp)
	movb	(%rsp), %dl
	subb	$0, %dl
	movzbl	%dl, %eax
	movl	%eax, %esi
	cmpb	$3, %dl
	cmovbeq	%rsi, %rcx
	cmpq	$1, %rcx
	jne	.LBB8_2
# %bb.1:
	ud2

.LBB8_2:
	addq	$1, %rsp
	retq
                                        # -- End function

main:                                   # @main
# %bb.0:
	subq	$24, %rsp
	movb	__rustc_debug_gdb_scripts_section__(%rip), %al
	movslq	%edi, %rcx
	leaq	playground::main(%rip), %rdi
	movq	%rsi, 16(%rsp)          # 8-byte Spill
	movq	%rcx, %rsi
	movq	16(%rsp), %rdx          # 8-byte Reload
	movb	%al, 15(%rsp)           # 1-byte Spill
	callq	std::rt::lang_start
	movl	%eax, %r8d
	movl	%r8d, %eax
	addq	$24, %rsp
	retq
                                        # -- End function

.L__unnamed_1:
	.quad	core::ptr::real_drop_in_place
	.quad	8                       # 0x8
	.quad	8                       # 0x8
	.quad	std::rt::lang_start::{{closure}}
	.quad	std::rt::lang_start::{{closure}}
	.quad	core::ops::function::FnOnce::call_once{{vtable.shim}}

__rustc_debug_gdb_scripts_section__:
	.asciz	"\001gdb_load_rust_pretty_printers.py"
                                        # DW_AT_GNU_pubnames
                                        # DW_AT_main_subprogram

