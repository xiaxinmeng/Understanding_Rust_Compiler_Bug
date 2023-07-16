rust
// crate a
pub trait MyTrait {}

pub struct MyLocalStruct;

impl MyTrait for MyLocalStruct {}

// crate b
use crate_a::MyTrait;

pub struct MyStruct;

impl MyTrait for MyStruct {}
