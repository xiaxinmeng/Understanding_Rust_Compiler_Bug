
#[deprecated]
fn foo() {}

macro_rules! bar (
    ($e: expr) => ( $e )
)

macro_rules! baz(
    ($i: ident) => ( bar!( $i() ) )
)

baz!(foo)
