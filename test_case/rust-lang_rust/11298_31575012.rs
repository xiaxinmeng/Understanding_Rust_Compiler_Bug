 rust
#[deriving(::foo::bar::Trait)] struct Struct;

use foo;
#[deriving(foo::bar::Trait)] struct Struct;

use foo::bar;
#[deriving(bar::Trait)] struct Struct;

use foo::bar::Trait;
#[deriving(Trait)] struct Struct;

use new_name = foo::bar;
#[deriving(new_name::Trait)] struct Struct;
