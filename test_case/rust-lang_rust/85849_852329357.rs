rust
#[test]
fn recursive_projection_bounds() {
    lowering_success! {
        program {
            trait Tr {
                type Output;
            }
            trait X <T, U> 
            where T: Tr<Output = <U as Tr>::Output>,
                  U: Tr<Output = <T as Tr>::Output>
                  {}
        }
    }
}
