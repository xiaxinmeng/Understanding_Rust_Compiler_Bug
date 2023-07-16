rust
pub fn lib() {}

trait Ext {
    fn unused(&self) { unused2() }
}

fn unused2() {}
