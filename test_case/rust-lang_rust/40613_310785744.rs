rust

impl File {
    pub fn has_parent<'a, P: IsA<File> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q) -> bool {
            let parent1 = parent.into();
let a = {
        unsafe {
            //This works
            //let parent2 = parent1.to_glib_none();
            //from_glib2(ffi::g_file_has_parent(self.to_glib_none().0, parent2.0))

            //This fails
            from_glib2(ffi::g_file_has_parent(self.to_glib_none().0, parent1.to_glib_none().0))

            //This works too
            //let v=ffi::g_file_has_parent(self.to_glib_none().0, parent1.to_glib_none().0);
            //from_glib2(v)
        }
};
a
    }
}
