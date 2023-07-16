
    #1 with rustc 1.35.0
error[E0597]: borrowed value does not live long enough
 --> <source>:2:18
  |
2 |     let x = &mut [1,2,3];
  |                  ^^^^^^^ temporary value does not live long enough
3 |     x
4 | };
  | - temporary value only lives until here
