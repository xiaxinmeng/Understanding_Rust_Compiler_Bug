
Breakpoint 1, test::main () at test.rs:38
38	    println!("yay");
number_none = core::option::Option::None
number_some = core::option::Option::Some(42)
number = 42
field_none = <error reading variable>
field_some_b = <error reading variable>
field_some_a = <error reading variable>
field_b = test::Field::B{field: 84}
field_a = test::Field::A{field: 42}
mixed_none = <error reading variable>
mixed_some_b = <error reading variable>
mixed_some_a = <error reading variable>
mixed_b = test::Mixed::B
mixed_a = test::Mixed::A(42)
union_none = <error reading variable>
union_some_b = <error reading variable>
union_some_a = <error reading variable>
union_b = test::Union::B(84)
union_a = test::Union::A(42)
enum_none = core::option::Option<test::Enum>::Some((unknown: 2))
enum_some_b = core::option::Option<test::Enum>::Some(test::Enum::B)
enum_some_a = core::option::Option<test::Enum>::None
enum_b = test::Enum::B
enum_a = test::Enum::A
