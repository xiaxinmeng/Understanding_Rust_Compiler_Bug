rust
fn main() {
    let _ = g((0, 0));
}

type P = (u32, u8); // one field must be ≥u32, the other field must be ≥u8
type R = Result<(), P>;

#[allow(dead_code)]
enum X {
    A(Box<X>),
    B(Box<X>),
    C, // B and C can be omitted, but they are added to ensure well-formed semantic
}

enum K {
    D
}

enum E {
    F(K), // must not be built-in type
    #[allow(dead_code)]
    G([X; 2]), // must present, can also be Vec<X> or (X, X), but not X or [X; 1] ...
}

fn g(mut pt: P) -> R {
    let mut y = None;
    loop {
        let status = if pt.0 == 0 {
            Some(E::F(K::D))
        } else {
            None
        };
        pt.0 = 1;

        match status {
            Some(infix_or_postfix) => {
                if let E::F(_op) = infix_or_postfix { // <- must be captured by value
                    y = Some(pt);
                    Ok(())?; // <- yes this line is needed
                }
            }
            _ => {
                Err(y.unwrap())? // <-- must use `?`, return Err won't trigger segfault
            }
        }
    }
}
