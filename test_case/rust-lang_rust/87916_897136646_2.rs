rust
fn black_box(_1: T) -> T {
    let mut _0: T; // return place
    let mut _2: &mut T;

	bb0: {
		  _0 = move _1; // LLVM can optimize out
	      _2 = &mut _0;
		  llvm_asm!(""::"r"(_2):"memory":"volatile");
		  return;
	}
}
