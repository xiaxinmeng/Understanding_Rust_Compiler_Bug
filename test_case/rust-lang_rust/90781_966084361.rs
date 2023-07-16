rust
pub trait TPubVis {}
#[doc(hidden)]
pub trait TPubHidden {}
trait TPrivVis {}
#[doc(hidden)]
trait TPrivHidden {}

pub struct VisPub;
struct VisPriv;
#[doc(hidden)]
pub struct HidPub;
#[doc(hidden)]
struct HidPriv;

macro_rules! implement {
    ($trait:ident - $($struct:ident)+) => {
        $(
            impl $trait for $struct {}
        )+
    }
}


implement!(TPubVis - VisPub VisPriv HidPub HidPriv);
implement!(TPubHidden - VisPub VisPriv HidPub HidPriv);
implement!(TPrivVis - VisPub VisPriv HidPub HidPriv);
implement!(TPrivHidden - VisPub VisPriv HidPub HidPriv);
