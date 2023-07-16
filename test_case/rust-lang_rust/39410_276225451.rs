
extern {
    #[link_name="llvm.bitreverse.i64"]
    fn bitrev_i64(in: i64) -> i64;
}
