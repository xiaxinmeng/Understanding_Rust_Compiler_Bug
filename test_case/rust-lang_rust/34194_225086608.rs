 rust
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

pub struct UID {
    uid: &'static str,
}

pub static ImplicitVRLittleEndian: &'static UID = &UID { uid: "1.2.840.10008.1.2" };

pub struct TransferSyntax {
    uid: &'static UID,
}

/**** The addition of the following line is what causes the error */
pub static ImplicitVRLittleEndian_TS: &'static TransferSyntax = &TransferSyntax { uid: &ImplicitVRLittleEndian };

fn main() {
    println!("Hello, world!");
}
