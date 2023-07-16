
#[derive(diesel::AsExpression)]
#[sql_type = "Option<::Bar>"]
struct Foo;

struct Bar;
