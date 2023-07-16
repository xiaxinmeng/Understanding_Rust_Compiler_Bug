rust
struct NoDerive;

impl PartialEq for NoDerive {
    fn eq(&self, _: &Self) -> bool { false }
}

impl Eq for NoDerive { }

#[derive(PartialEq, Eq)]
struct WrapInline(NoDerive);

const WRAP_INLINE: & &WrapInline = & &WrapInline(NoDerive);

fn main() {
    match WRAP_INLINE {
        WRAP_INLINE => { println!("WRAP_INLINE matched WRAP_INLINE"); }
        _ => { println!("WRAP_INLINE did not match WRAP_INLINE"); }
    }
}
