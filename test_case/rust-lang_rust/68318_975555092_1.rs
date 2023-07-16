
pub struct Test{

}

impl Drop for Test {
    fn drop() {
        panic!("Do not drop Test.")
    }
}
