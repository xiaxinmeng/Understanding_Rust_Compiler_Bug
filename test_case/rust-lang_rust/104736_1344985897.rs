rust
struct Response {
    f: String,
}

#[inline(always)]
fn ident(a: Response) -> Response {
    a
}

fn format_response(a: Response, b: Response, c: String, d: String, x: bool) -> Response {
    'scope: {
        ident(Response {
            f: {
                let _z = d;
                if x {
                    c
                } else {
                    break 'scope b;
                }
            },
            ..{ a }
        })
    }
}
