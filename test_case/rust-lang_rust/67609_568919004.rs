rust
static DIVISIONS_200312: Option<String> = None;

pub static DIVISIONS: &&Option<String> = {
    let x = &&DIVISIONS_200312;
    x
};
