
[INFO] [stdout] error: expected expression, found keyword `const`
[INFO] [stdout]    --> src/ir.rs:481:23
[INFO] [stdout]     |
[INFO] [stdout] 347 |     (exp $e:expr) => {
[INFO] [stdout]     |          ------- while parsing argument for this `expr` macro fragment
[INFO] [stdout] ...
[INFO] [stdout] 481 |             stmt!(exp const 1);
[INFO] [stdout]     |                       ^^^^^ expected expression
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stdout] error: expected expression, found keyword `const`
[INFO] [stdout]    --> src/ir.rs:482:23
[INFO] [stdout]     |
[INFO] [stdout] 347 |     (exp $e:expr) => {
[INFO] [stdout]     |          ------- while parsing argument for this `expr` macro fragment
[INFO] [stdout] ...
[INFO] [stdout] 482 |             stmt!(exp const 2);
[INFO] [stdout]     |                       ^^^^^ expected expression
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stdout] error: expected expression, found keyword `const`
[INFO] [stdout]    --> src/ir.rs:483:23
[INFO] [stdout]     |
[INFO] [stdout] 347 |     (exp $e:expr) => {
[INFO] [stdout]     |          ------- while parsing argument for this `expr` macro fragment
[INFO] [stdout] ...
[INFO] [stdout] 483 |             stmt!(exp const 3);
[INFO] [stdout]     |                       ^^^^^ expected expression
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stdout] error: aborting due to 3 previous errors
