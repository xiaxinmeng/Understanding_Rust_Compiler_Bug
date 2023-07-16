
#[derive(diesel::AsExpression)]
#[sql_type = crate::"Option<::Bar>"]
struct Foo;

struct Bar;
