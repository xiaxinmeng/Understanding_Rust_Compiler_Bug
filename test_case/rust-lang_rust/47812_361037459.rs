rust
#[macro_use]
extern crate serde_derive;

#[derive(Serialize)]
struct S {
    #[serde(broken)]
    f: u8,
}
