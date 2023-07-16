impl Hash for DefId {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.krate.hash(h);
        self.index.hash(h);
    }
}
