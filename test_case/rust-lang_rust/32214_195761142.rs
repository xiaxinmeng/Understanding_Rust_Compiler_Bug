
pub fn write_mir_pretty<'a, W, I>(iter: &Iterator<Item=(&'a NodeId, &'a Mir<'a>), w: &mut W) -> io::Result<()>
