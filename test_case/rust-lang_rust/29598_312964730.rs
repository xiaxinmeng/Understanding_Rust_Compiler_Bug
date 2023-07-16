
    = note: expanding `gen_test_funcs! { @ named test_21 [ "/**" , "*" , false ] }`
    = note: to `# [ test ] fn test_21 (  ) { gen_tests ! ( [ "/**" , "*" , false ] ) ; }`
    = note: expanding `gen_tests! { [ "/**" , "*" , false ] }`
    = note: to `gen_tests ! ( @ single [ "/**" , "*" , false ] ) ;`
    = note: expanding `gen_tests! { @ single [ "/**" , "*" , false ] }`
    = note: to `gen_tests ! { @ test [ "/**" , "*" , None :: < String > ] }`
    = note: expanding `gen_tests! { @ test [ "/**" , "*" , None :: < String > ] }`
    = note: to `use $crate :: intersect ; let d = intersect ( "/**" , "*" , (  ) ) ; if d !=
            None::<String> {
            panic ! (
            "Assertion failed:\nInput:  intersect({:?}, {:?})\nExpected:  {:?}\n  Actual:  {:?}\n"
            , "/**" , "*" , None::<String> , d ) ; }`
    = note: expanding `panic! { "Assertion failed:\nInput:  intersect({:?}, {:?})\nExpected:  {:?}\n  Actual:  {:?}\n"
            , "/**" , "*" , None::<String> , d }`
    = note: to `{
            $crate :: rt :: begin_panic_fmt (
            & format_args ! (
            "Assertion failed:\nInput:  intersect({:?}, {:?})\nExpected:  {:?}\n  Actual:  {:?}\n"
            , "/**" , "*" , None::<String> , d ) , {
            static _FILE_LINE : ( & 'static str , u32 ) = ( file ! (  ) , line ! (  ) ) ;
            & _FILE_LINE } ) }`
    = note: expanding `gen_test_funcs! { @ name_them [
            test_22 test_23 test_24 test_25 test_26 test_27 test_28 test_29 test_30
            test_31 test_32 test_33 test_34 test_35 test_36 test_37 test_38 test_39
            test_40 test_41 test_42 test_43 test_44 test_45 test_46 test_47 test_48
            test_49 ] [ [ "**/" , "*" , false ] ] }`
    = note: to `gen_test_funcs ! ( @ named test_22 [ "**/" , "*" , false ] ) ; gen_test_funcs
            ! (
            @ name_them [
            test_23 test_24 test_25 test_26 test_27 test_28 test_29 test_30 test_31
            test_32 test_33 test_34 test_35 test_36 test_37 test_38 test_39 test_40
            test_41 test_42 test_43 test_44 test_45 test_46 test_47 test_48 test_49 ] [  ]
            ) ;`
    = note: expanding `gen_test_funcs! { @ named test_22 [ "**/" , "*" , false ] }`
    = note: to `# [ test ] fn test_22 (  ) { gen_tests ! ( [ "**/" , "*" , false ] ) ; }`
    = note: expanding `gen_tests! { [ "**/" , "*" , false ] }`
    = note: to `gen_tests ! ( @ single [ "**/" , "*" , false ] ) ;`
    = note: expanding `gen_tests! { @ single [ "**/" , "*" , false ] }`
    = note: to `gen_tests ! { @ test [ "**/" , "*" , None :: < String > ] }`
    = note: expanding `gen_tests! { @ test [ "**/" , "*" , None :: < String > ] }`
    = note: to `use $crate :: intersect ; let d = intersect ( "**/" , "*" , (  ) ) ; if d !=
            None::<String> {
            panic ! (
            "Assertion failed:\nInput:  intersect({:?}, {:?})\nExpected:  {:?}\n  Actual:  {:?}\n"
            , "**/" , "*" , None::<String> , d ) ; }`
    = note: expanding `panic! { "Assertion failed:\nInput:  intersect({:?}, {:?})\nExpected:  {:?}\n  Actual:  {:?}\n"
            , "**/" , "*" , None::<String> , d }`
    = note: to `{
            $crate :: rt :: begin_panic_fmt (
            & format_args ! (
            "Assertion failed:\nInput:  intersect({:?}, {:?})\nExpected:  {:?}\n  Actual:  {:?}\n"
            , "**/" , "*" , None::<String> , d ) , {
            static _FILE_LINE : ( & 'static str , u32 ) = ( file ! (  ) , line ! (  ) ) ;
            & _FILE_LINE } ) }`
    = note: expanding `gen_test_funcs! { @ name_them [
            test_23 test_24 test_25 test_26 test_27 test_28 test_29 test_30 test_31
            test_32 test_33 test_34 test_35 test_36 test_37 test_38 test_39 test_40
            test_41 test_42 test_43 test_44 test_45 test_46 test_47 test_48 test_49 ] [  ] }`
    = note: to ``
