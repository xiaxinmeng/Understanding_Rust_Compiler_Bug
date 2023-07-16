
let (x, &(y, ref z)) = &(1, (2, 3));
//                          ^^^^^^ should be `&(2, 3)`, else you get an error
