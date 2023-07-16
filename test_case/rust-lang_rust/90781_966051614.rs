rust
pub trait T {}

pub struct VisPub;
struct VisPriv;
#[doc(hidden)]
pub struct HidPub;
#[doc(hidden)]
struct HidPriv;

impl T for VisPub {}
impl T for VisPriv {}
impl T for HidPub {}
impl T for HidPriv {}
