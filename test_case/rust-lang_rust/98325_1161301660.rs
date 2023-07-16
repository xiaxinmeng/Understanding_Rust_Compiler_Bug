plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: associated function has missing stability attribute
   --> library/core/src/pin.rs:604:5
    |
604 | /     pub const fn as_inner_ref(&self) -> &P {
605 | |         &self.p
    | |_____^


error[E0609]: no field `p` on type `&Pin<P>`
    |
605 |         &self.p
    |               ^ unknown field

