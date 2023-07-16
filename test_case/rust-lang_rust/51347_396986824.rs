
(gdb) info local
number_none = core::option::Option<i32>::None
number_some = core::option::Option<i32>::Some(42)
number = 42
field_none = core::option::Option<a::Field>::None
field_some_b = core::option::Option<a::Field>::Some(a::Field::B{field: 84})
field_some_a = core::option::Option<a::Field>::Some(a::Field::A{field: 42})
field_b = a::Field::B{field: 84}
field_a = a::Field::A{field: 42}
mixed_none = core::option::Option<a::Mixed>::None
mixed_some_b = core::option::Option<a::Mixed>::Some(a::Mixed::B)
mixed_some_a = core::option::Option<a::Mixed>::Some(a::Mixed::A(42))
mixed_b = a::Mixed::B
mixed_a = a::Mixed::A(42)
union_none = core::option::Option<a::Union>::None
union_some_b = core::option::Option<a::Union>::Some(a::Union::B(84))
union_some_a = core::option::Option<a::Union>::Some(a::Union::A(42))
union_b = a::Union::B(84)
union_a = a::Union::A(42)
enum_none = core::option::Option<a::Enum>::None
enum_some_b = core::option::Option<a::Enum>::Some(a::Enum::B)
enum_some_a = core::option::Option<a::Enum>::Some(a::Enum::A)
enum_b = a::Enum::B
enum_a = a::Enum::A
