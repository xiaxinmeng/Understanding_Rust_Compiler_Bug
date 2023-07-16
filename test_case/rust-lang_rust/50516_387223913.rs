
   │1979        fn _create(&self, path: &Path) -> io::Result<()> {
   │1980            if self.recursive {
   │1981                self.create_dir_all(path)
   │1982            } else {
  >│1983                self.inner.mkdir(path)
   │1984            }
