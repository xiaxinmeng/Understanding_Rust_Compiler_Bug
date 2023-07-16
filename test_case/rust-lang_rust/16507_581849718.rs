rust
// Return result, alternatively it could panic:
fn file_in_dir<I>(&self, components: I) -> Result<PathBuf, ComponentContainsSlashError> 
where I: Copy + IntoIterator, <I as IntoIterator>::Item: AsRef<OsStr>;

// Alternatively, using newtype SlashlessOsStr that can only be created from str and guarantees to not contain slashes
fn file_in_dir<I>(&self, components: I) -> PathBuf 
where I: Copy + IntoIterator, <I as IntoIterator>::Item: AsRef<SlashlessOsStr>;

// join renamed
fn as_base_to(&self, maybe_relative_trusted_path: &Path) -> PathBuf;

// Sloppy chroot assumes there are no symlinks, so /foo/../bar will become /bar
fn as_sloppy_chroot(&self, containing: &Path) -> PathBuf;

// Does exactly what chroot would: resolves symlinks in a way that makes sure they don't point outside
// I have no clue what it should do on windows if you use "c:\" in containing, possibly return error with kind InvalidInput
fn as_chroot(&self, containing: &Path) -> io::Result<PathBuf>;
