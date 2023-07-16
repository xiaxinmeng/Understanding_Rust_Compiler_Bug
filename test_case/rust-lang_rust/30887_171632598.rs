
<anon>:6:9: 6:20 error: blocks in constant functions are limited to items and tail expressions [E0016]
<anon>:6     let mut sum = 0;
                 ^~~~~~~~~~~
<anon>:6:9: 6:20 help: see the detailed explanation for E0016
<anon>:7:5: 9:6 error: blocks in constant functions are limited to items and tail expressions [E0016]
<anon>:7     for i in 0..x {
<anon>:8         sum += i;
<anon>:9     }
<anon>:7:5: 9:6 help: see the detailed explanation for E0016
<anon>:7:5: 9:6 error: blocks in constant functions are limited to items and tail expressions [E0016]
<anon>:7     for i in 0..x {
<anon>:8         sum += i;
<anon>:9     }
<anon>:7:5: 9:6 help: see the detailed explanation for E0016
<anon>:7:5: 9:6 error: constant function contains unimplemented expression type [E0019]
<anon>:7     for i in 0..x {
<anon>:8         sum += i;
<anon>:9     }
<anon>:7:5: 9:6 help: see the detailed explanation for E0019
<anon>:7:5: 9:6 error: function calls in constant functions are limited to constant functions, struct and enum constructors [E0015]
<anon>:7     for i in 0..x {
<anon>:8         sum += i;
<anon>:9     }
<anon>:7:5: 9:6 help: see the detailed explanation for E0015
<anon>:7:14: 7:18 error: constant function contains unimplemented expression type [E0019]
<anon>:7     for i in 0..x {
                      ^~~~
<anon>:7:14: 7:18 help: see the detailed explanation for E0019
<anon>:7:5: 9:6 error: constant function contains unimplemented expression type [E0019]
<anon>:7     for i in 0..x {
<anon>:8         sum += i;
<anon>:9     }
<anon>:7:5: 9:6 help: see the detailed explanation for E0019
<anon>:7:5: 9:6 error: constant function contains unimplemented expression type [E0019]
<anon>:7     for i in 0..x {
<anon>:8         sum += i;
<anon>:9     }
<anon>:7:5: 9:6 help: see the detailed explanation for E0019
<anon>:7:5: 9:6 error: function calls in constant functions are limited to constant functions, struct and enum constructors [E0015]
<anon>:7     for i in 0..x {
<anon>:8         sum += i;
<anon>:9     }
<anon>:7:5: 9:6 help: see the detailed explanation for E0015
<anon>:7:5: 9:6 error: references in constant functions may only refer to immutable values [E0017]
<anon>:7     for i in 0..x {
<anon>:8         sum += i;
<anon>:9     }
<anon>:7:5: 9:6 help: see the detailed explanation for E0017
<anon>:8:9: 8:17 error: blocks in constant functions are limited to items and tail expressions [E0016]
<anon>:8         sum += i;
                 ^~~~~~~~
<anon>:8:9: 8:17 help: see the detailed explanation for E0016
<anon>:8:9: 8:17 error: constant function contains unimplemented expression type [E0019]
<anon>:8         sum += i;
                 ^~~~~~~~
<anon>:8:9: 8:17 help: see the detailed explanation for E0019
<anon>:7:5: 9:6 error: constant function contains unimplemented expression type [E0019]
<anon>:7     for i in 0..x {
<anon>:8         sum += i;
<anon>:9     }
<anon>:7:5: 9:6 help: see the detailed explanation for E0019
