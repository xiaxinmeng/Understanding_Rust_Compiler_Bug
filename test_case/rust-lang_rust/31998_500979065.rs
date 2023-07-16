rust
 1 fn main() {
 2     println!("start");
 3
 4     let data = vec![
 5         String::from("Hello"),
 6         String::from("world"),
 7         String::from("!"),
 8     ];
 9
10     // let filter = |_: &_| true; // okay
11     let filter = |_| true; // warning/error
12
13     for obj in data.into_iter().map(Some) {
14         // error since obj is swallowed by filter
15         // filter(obj);
16         // println!("{:?}", obj); // error occurs here
17
18         // okay since only borrowed to filter
19         filter(&obj);
20         println!("{:?}", obj); // okay
21     }
22
23     println!("finish");
24 }
