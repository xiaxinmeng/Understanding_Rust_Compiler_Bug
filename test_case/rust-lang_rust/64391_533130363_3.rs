rust
let _temp; 
let config = 666;
let _4 = &config;
let _6 = "".to_string();
_3 = connect2(move _4, move _6);
drop(_6); // <-- interesting!
let the_future = _3;
$await(the_future);
...
drop(_3);
