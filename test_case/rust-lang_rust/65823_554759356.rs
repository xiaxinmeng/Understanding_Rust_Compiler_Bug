rust 
#[derive(Serialize,Deserialize)]
struct Foo {
   #[serde(rename = "bar")]
   x: i32
}
