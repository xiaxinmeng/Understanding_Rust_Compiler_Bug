
#[cuda_constant]
static X: i32 = 0; // not const since rustc would inline it then

#[cuda_kernel]
fn foo() {
    #[cuda_shared]
    let x: [i32; BLOCK_SIZE] = mem::uninitialized();
}
