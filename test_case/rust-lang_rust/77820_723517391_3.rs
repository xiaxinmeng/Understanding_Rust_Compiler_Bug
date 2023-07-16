rust
impl Clean<Path> for hir::definitions::DefPath {
    fn clean(&self, cx: &DocContext<'_>) -> Path {
        Path {
            global: unimplemented!(),
            res: unimplemented!(),
            segments: unimplemented!(),
        }
    }
}
