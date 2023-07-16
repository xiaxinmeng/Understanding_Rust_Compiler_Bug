fust
let tmp_ref = &foo(); // freed at end of block
let p = bar(tmp_ref); 
...
