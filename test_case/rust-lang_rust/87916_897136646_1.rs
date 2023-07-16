rust
fn black_box(_1: T) -> T {
    let mut _0: T; // return place
    let mut _2: &mut T;

	bb0: {
	      _2 = &mut _1;
		  llvm_asm!(""::"r"(_2):"memory":"volatile");
	      _0 = move _1; // LLVM can't optimize out
		  return;
	}
}
