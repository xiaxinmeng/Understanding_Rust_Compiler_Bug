rust
pub macro m() {
    pub(crate) struct S1;
    pub(in path) struct S2;
    pub(self) struct S3;
    struct S4;
}
