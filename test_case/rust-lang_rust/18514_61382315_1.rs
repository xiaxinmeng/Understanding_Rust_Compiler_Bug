 rust
extern crate ice;
use ice::{St,Tr};

fn main() {
    let st: St<()> = St(vec![]);
    st.tr();
}
