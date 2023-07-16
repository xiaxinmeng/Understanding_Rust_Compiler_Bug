 rust
#![feature(globs)]

extern crate ice;

use ice::*;

#[test]
fn test() {
  let values: Insert<(), (), ()> = InsertDefaultValues;
  let query: InsertQuery<(), (), ()> = InsertQuery {
       values: values
  };
  println!("result");
  println!("{}", query.to_sql());  
}
