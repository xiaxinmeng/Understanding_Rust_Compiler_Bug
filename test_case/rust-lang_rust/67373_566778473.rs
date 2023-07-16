
#[derive(diesel::AsExpression)]
#[sql_type = "::Bar"]
struct Foo;

struct Bar;
