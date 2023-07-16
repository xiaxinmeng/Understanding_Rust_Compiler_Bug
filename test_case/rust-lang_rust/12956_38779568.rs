
    <anon>:8:13: 8:23 error: multiple applicable methods in scope
    <anon>:8 assert_eq!( 5.cmp(&10), Less);     // because 5 < 10
                         ^~~~~~~~~~
    <std macros>:1:1: 12:2 note: in expansion of assert_eq!
    <anon>:8:1: 8:31 note: expansion site
    <anon>:8:13: 8:23 note: candidate #1 is `std::cmp::u8.TotalOrd::cmp`
    <anon>:8 assert_eq!( 5.cmp(&10), Less);     // because 5 < 10
                         ^~~~~~~~~~
    <std macros>:1:1: 12:2 note: in expansion of assert_eq!
    <anon>:8:1: 8:31 note: expansion site
    <anon>:8:13: 8:23 note: candidate #2 is `std::cmp::u16.TotalOrd::cmp`
    <anon>:8 assert_eq!( 5.cmp(&10), Less);     // because 5 < 10
                         ^~~~~~~~~~
