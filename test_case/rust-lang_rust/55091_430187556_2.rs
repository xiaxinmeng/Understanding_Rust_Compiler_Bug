
> rustc +nightly --version --verbose
rustc 1.31.0-nightly (46880f41b 2018-10-15)
binary: rustc
commit-hash: 46880f41b7aeb897b8245474196bba9dc11f0e88
commit-date: 2018-10-15
host: x86_64-pc-windows-msvc
release: 1.31.0-nightly
LLVM version: 8.0

> rustc +nightly -O --emit asm main.rs
> type main.s
...
_ZN4main8traverse17h9d4efd61173c1cd3E:
.seh_proc _ZN4main8traverse17h9d4efd61173c1cd3E
	pushq	%rsi
	.seh_pushreg 6
	pushq	%rdi
	.seh_pushreg 7
	subq	$88, %rsp
	.seh_stackalloc 88
	.seh_endprologue
...
