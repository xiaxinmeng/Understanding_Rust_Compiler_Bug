rust
pub trait Foo {}

pub struct Baz;
pub struct Quux;
pub type Thud = Quux;

impl Foo for Baz {}
impl Foo for Thud {}
