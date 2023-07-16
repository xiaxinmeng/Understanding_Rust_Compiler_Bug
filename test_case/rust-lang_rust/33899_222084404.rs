
#[macro_use]
extern crate bson;
extern crate mongodb;

use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};

fn main() {
    let client = Client::connect("192.168.8.110", 27017).unwrap();
    let collection = client.db("test").collection("test");
    let index_keys = doc!{"x" => 1};
    let result = collection.create_index(index_keys, None);
    println!("{:?}", result);
}

