diff
  impl Path {
-     pub fn strip_prefix<'a, P: ?Sized>(&'a self, base: &'a P) -> Result<&'a Path, StripPrefixError>
+     pub fn strip_prefix<P>(&self, base: P) -> Result<&Path, StripPrefixError>
          where P: AsRef<Path>;
  }
