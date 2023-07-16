rust 
{ // 1
    8;
    8
} // -> implicit return of an i32
{ // 2
    8;
    8;
} // no last expression, therefore i32 type is "discarded" and () is returned
{ // 3
    8;
    return 8;
} // no last expression, but last statement is a return, so returns an i32
