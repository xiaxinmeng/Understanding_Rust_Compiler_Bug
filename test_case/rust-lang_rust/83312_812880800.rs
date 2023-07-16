rust
fn attrs() {
    (#![print_target_and_args(fifth)] 1, 2);

    [#![print_target_and_args(sixth)] 1 , 2];
    [#![print_target_and_args(seventh)] true ; 5];


    match 0 {
        #![print_target_and_args(eighth)]
        _ => {}
    }

    MyStruct { #![print_target_and_args(ninth)] field: true };
}
