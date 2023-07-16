
1  | ( $ left : expr , $ right : expr ) => (
   |     ^^^^ 1          ^^^^^ 2
2  | {
3  | match ( & $ left , & $ right ) {
   |         --------  ---------- 4 through 2
   |         |
   |         3 through 1
4  | ( left_val , right_val ) => {
   |   --------  ---------- 6 through 4
   |   |
   |   5 through 3
5  | if ! ( * left_val == * right_val ) {
   |        ^^^^^^^^^^    ^^^^^^^^^^^ 8 though 6
   |        |
   |        7 through 5
