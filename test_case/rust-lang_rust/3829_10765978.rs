
enum Result {
    Ok(int), Err(int)
}

fn macros() {
    macro_rules! if_ok(
        ($inp: expr) => (
            match $inp {
                Ok(move v) => { move v }
                Err(move e) => { return Err(e); }
            }
        )
    );
}

fn f() -> Result {
    let r = Ok(2);
    let p = if_ok!(move r);
    assert p == 2;
    return Ok(p);
}

fn main() {
    f();
}
