
pub struct VariantDoc {
    desc: Option<~str>,
    sig: Option<~str>
}

fn main() {
    let variants = ~[
        VariantDoc {
            desc: None,
            sig: None
        }
    ];

    let _ = do vec::map(variants) |variant| {
        let new_variant = copy *variant;
        let result = VariantDoc {
            desc: None,
            sig: None,
        };
        io::println("vvv");
        result
    };
    io::println("^^^");
}
