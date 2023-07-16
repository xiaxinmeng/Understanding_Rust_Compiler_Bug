
// note the ordering here! This temporary is introduced into "the surrounding scope", 
// which means it goes *before* the let-bound variables. 
// This is basically the bug in our current rules.
let _3; 
let config = 666;
_3= connect2(&config, "".to_string());
let the_future = _3;
$await(the_future);
...
drop(_temp);
