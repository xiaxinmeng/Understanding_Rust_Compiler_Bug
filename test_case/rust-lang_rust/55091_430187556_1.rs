
> rustc +stable --version --verbose
rustc 1.29.2 (17a9dc751 2018-10-05)
binary: rustc
commit-hash: 17a9dc7513b9fea883dc9505f09f97c63d1d601b
commit-date: 2018-10-05
host: x86_64-pc-windows-msvc
release: 1.29.2
LLVM version: 7.0

> rustc +stable -O --emit asm main.rs
> type main.s
...
_ZN4main8traverse17h1340ae51c350589fE:
.seh_proc _ZN4main8traverse17h1340ae51c350589fE
	pushq	%rsi
	.seh_pushreg 6
	pushq	%rdi
	.seh_pushreg 7
	subq	$88, %rsp
	.seh_stackalloc 88
	.seh_endprologue
...
