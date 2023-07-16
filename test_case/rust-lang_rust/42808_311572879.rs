rust
#[macro_use]
extern crate serde;
extern crate serde_yaml;

use std::fs::OpenOptions;

#[derive(Debug, Deserialize)]
pub struct Something {
    field1: Vec<S1>,
    field2: Vec<S2>,
    field3: Vec<S3>,
    field4: Vec<S4>,
    field5: Vec<S5>,
    field6: Vec<S6>,
    field7: Vec<S7>,
    field8: Vec<S8>,
    field9: Vec<S9>,
}

#[derive(Debug, Deserialize)]
pub struct S1(String);

#[derive(Debug, Deserialize)]
pub struct S2(String);

#[derive(Debug, Deserialize)]
pub struct S3(String);

#[derive(Debug, Deserialize)]
pub struct S4(String);

#[derive(Debug, Deserialize)]
pub struct S5(String);

#[derive(Debug, Deserialize)]
pub struct S6(String);

#[derive(Debug, Deserialize)]
pub struct S7(String);

#[derive(Debug, Deserialize)]
pub struct S8(String);

#[derive(Debug, Deserialize)]
pub struct S9(String);

fn main() {
    println!(
        "{:?}",
        serde_yaml::from_reader::<_, Something>(OpenOptions::new().open("whatever").unwrap())
    );
}
