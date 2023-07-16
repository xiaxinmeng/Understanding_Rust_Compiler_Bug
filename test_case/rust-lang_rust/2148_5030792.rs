
fn process(idents: [str]) -> [str] { idents }

fn class_field_mutbl(idents: [str], ident: str) -> bool {
    for process(idents).each {|fld|
        if str::eq(ident, fld) {
            ret true;
        }
    }
    fail #fmt["No class field `%s` found", ident];
}

fn main() { }
