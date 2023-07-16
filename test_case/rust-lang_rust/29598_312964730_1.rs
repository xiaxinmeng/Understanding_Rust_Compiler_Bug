
gen_test_funcs! { [ "{x,{a,b}}" , "*" , "{x,a,b}" ] [
"{{x,y},{ab{bc,{de}}}}" , "*" , "{x,y,ab{bc,de}}" ] [
"{{{},{{}}},{{{},{{}}}}}" , "*" , "" ] }
gen_test_funcs! { @ name_them [
test_01 test_02 test_03 test_04 test_05 test_06 test_07 test_08 test_09
test_10 test_11 test_12 test_13 test_14 test_15 test_16 test_17 test_18
test_19 test_20 test_21 test_22 test_23 test_24 test_25 test_26 test_27
test_28 test_29 test_30 test_31 test_32 test_33 test_34 test_35 test_36
test_37 test_38 test_39 test_40 test_41 test_42 test_43 test_44 test_45
test_46 test_47 test_48 test_49 ] [
[ "{x,{a,b}}" , "*" , "{x,a,b}" ] [
"{{x,y},{ab{bc,{de}}}}" , "*" , "{x,y,ab{bc,de}}" ] [
"{{{},{{}}},{{{},{{}}}}}" , "*" , "" ] ] }
gen_test_funcs! { @ named test_01 [ "{x,{a,b}}" , "*" , "{x,a,b}" ] }
gen_tests! { [ "{x,{a,b}}" , "*" , "{x,a,b}" ] }
gen_tests! { @ single [ "{x,{a,b}}" , "*" , "{x,a,b}" ] }
gen_tests! { @ test [ "{x,{a,b}}" , "*" , Some ( "{x,a,b}" . to_string (  ) ) ] }
panic! { "Assertion failed:\nInput:  intersect({:?}, {:?})\nExpected:  {:?}\n  Actual:  {:?}\n"
, "{x,{a,b}}" , "*" , Some("{x,a,b}".to_string()) , d }
gen_test_funcs! { @ name_them [
test_02 test_03 test_04 test_05 test_06 test_07 test_08 test_09 test_10
test_11 test_12 test_13 test_14 test_15 test_16 test_17 test_18 test_19
test_20 test_21 test_22 test_23 test_24 test_25 test_26 test_27 test_28
test_29 test_30 test_31 test_32 test_33 test_34 test_35 test_36 test_37
test_38 test_39 test_40 test_41 test_42 test_43 test_44 test_45 test_46
test_47 test_48 test_49 ] [
[ "{{x,y},{ab{bc,{de}}}}" , "*" , "{x,y,ab{bc,de}}" ] [
"{{{},{{}}},{{{},{{}}}}}" , "*" , "" ] ] }
gen_test_funcs! { @ named test_02 [ "{{x,y},{ab{bc,{de}}}}" , "*" , "{x,y,ab{bc,de}}" ] }
gen_tests! { [ "{{x,y},{ab{bc,{de}}}}" , "*" , "{x,y,ab{bc,de}}" ] }
gen_tests! { @ single [ "{{x,y},{ab{bc,{de}}}}" , "*" , "{x,y,ab{bc,de}}" ] }
gen_tests! { @ test [
"{{x,y},{ab{bc,{de}}}}" , "*" , Some ( "{x,y,ab{bc,de}}" . to_string (  ) ) ] }
panic! { "Assertion failed:\nInput:  intersect({:?}, {:?})\nExpected:  {:?}\n  Actual:  {:?}\n"
, "{{x,y},{ab{bc,{de}}}}" , "*" , Some("{x,y,ab{bc,de}}".to_string()) , d }
gen_test_funcs! { @ name_them [
test_03 test_04 test_05 test_06 test_07 test_08 test_09 test_10 test_11
test_12 test_13 test_14 test_15 test_16 test_17 test_18 test_19 test_20
test_21 test_22 test_23 test_24 test_25 test_26 test_27 test_28 test_29
test_30 test_31 test_32 test_33 test_34 test_35 test_36 test_37 test_38
test_39 test_40 test_41 test_42 test_43 test_44 test_45 test_46 test_47
test_48 test_49 ] [ [ "{{{},{{}}},{{{},{{}}}}}" , "*" , "" ] ] }
gen_test_funcs! { @ named test_03 [ "{{{},{{}}},{{{},{{}}}}}" , "*" , "" ] }
gen_tests! { [ "{{{},{{}}},{{{},{{}}}}}" , "*" , "" ] }
gen_tests! { @ single [ "{{{},{{}}},{{{},{{}}}}}" , "*" , "" ] }
gen_tests! { @ test [ "{{{},{{}}},{{{},{{}}}}}" , "*" , Some ( "" . to_string (  ) ) ] }
panic! { "Assertion failed:\nInput:  intersect({:?}, {:?})\nExpected:  {:?}\n  Actual:  {:?}\n"
, "{{{},{{}}},{{{},{{}}}}}" , "*" , Some("".to_string()) , d }
gen_test_funcs! { @ name_them [
test_04 test_05 test_06 test_07 test_08 test_09 test_10 test_11 test_12
test_13 test_14 test_15 test_16 test_17 test_18 test_19 test_20 test_21
test_22 test_23 test_24 test_25 test_26 test_27 test_28 test_29 test_30
test_31 test_32 test_33 test_34 test_35 test_36 test_37 test_38 test_39
test_40 test_41 test_42 test_43 test_44 test_45 test_46 test_47 test_48
test_49 ] [  ] }
