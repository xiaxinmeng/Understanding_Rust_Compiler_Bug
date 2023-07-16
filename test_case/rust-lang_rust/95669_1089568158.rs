
#[allow(unused_macros)]
macro_rules! m_used { ( $id ) => {} }
macro_rules! m_unused { ( $id ) => {} }
fn main() { m_used!(x); }
