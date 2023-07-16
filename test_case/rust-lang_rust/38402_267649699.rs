
$ cat foo.rs

pub struct Parser {
    state: State,
}

#[derive(Clone)]
pub struct State {
    // This struct must have at least two members to trigger
    // NEON code generation.
    pub u: u32,
    pub v: u32,
}

#[no_mangle]
pub unsafe fn parser_new(state: *const State) -> *mut Parser {
    let parser = Box::new(Parser {
        state: (*state).clone(),
    });
    Box::into_raw(parser)

}

$ rustc +nightly -C opt-level=3 --target armv7-unknown-linux-gnueabihf foo.rs --crate-type lib --emit asm && cat foo.s
warning: field is never used: `state`, #[warn(dead_code)] on by default
 --> foo.rs:3:5
  |
3 |     state: State,
  |     ^^^^^^^^^^^^

	.text
	.syntax unified
	.eabi_attribute	67, "2.09"
	.eabi_attribute	6, 10
	.eabi_attribute	7, 65
	.eabi_attribute	8, 1
	.eabi_attribute	9, 2
	.fpu	vfpv3-d16
	.eabi_attribute	15, 1
	.eabi_attribute	16, 1
	.eabi_attribute	17, 2
	.eabi_attribute	20, 1
	.eabi_attribute	21, 1
	.eabi_attribute	23, 3
	.eabi_attribute	34, 1
	.eabi_attribute	24, 1
	.eabi_attribute	25, 1
	.eabi_attribute	28, 1
	.eabi_attribute	38, 1
	.eabi_attribute	14, 0
	.file	"foo.cgu-0.rs"
	.section	.text.parser_new,"ax",%progbits
	.globl	parser_new
	.p2align	2
	.type	parser_new,%function
parser_new:
	.fnstart
	.save	{r4, r5, r11, lr}
	push	{r4, r5, r11, lr}
	ldm	r0, {r4, r5}
	mov	r0, #8
	mov	r1, #4
	bl	__rust_allocate
	cmp	r0, #0
	stmne	r0, {r4, r5}
	popne	{r4, r5, r11, pc}
	bl	_ZN5alloc3oom3oom17h9abadf64e5736b05E
.Lfunc_end0:
	.size	parser_new, .Lfunc_end0-parser_new
	.fnend


	.section	".note.GNU-stack","",%progbits
	.eabi_attribute	30, 2

