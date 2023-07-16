
#[derive(diesel::AsExpression)]
#[sql_type = crate::"::Bar"]
struct Foo;

struct Bar;
