rs
trait DefaultNonConstIsConst {
    #[default_method_body_is_const]
    fn i_lied() {
        println!("WARNING: NOT CONST")
    }
}
