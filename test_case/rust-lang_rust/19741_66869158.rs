 rust
trait BorrowFrom<'a, To> {
    fn borrow_from(&'a self) -> To;
}

impl<'a> BorrowFrom<'a, &'a [u8]> for Vec<u8> {
    fn borrow_from(&'a self) -> &'a [u8] {
        self.as_slice()
    }
}

impl<'a> BorrowFrom<'a, &'a str> for String {
    fn borrow_from(&'a self) -> &'a str {
        self.as_slice()
    }
}

impl<'a> BorrowFrom<'a, &'a [u8]> for String {
    fn borrow_from(&'a self) -> &'a [u8] {
        self.as_bytes()
    }
}

impl<'a> BorrowFrom<'a, &'a [u8]> for &'a str {
    fn borrow_from(&'a self) -> &'a [u8] {
        self.as_bytes()
    }
}

impl<'a, T: BorrowFrom<'a, U>, U> BorrowFrom<'a, U> for &'a T {
    fn borrow_from(&'a self) -> U {
        (**self).borrow_from()
    }
}

#[deriving(Show)]
struct Datum<'a> { data: &'a [u8] }

impl<'a> BorrowFrom<'a, Datum<'a>> for String {
    fn borrow_from(&'a self) -> Datum<'a> {
        self.as_slice().borrow_from()
    }
}

impl<'a> BorrowFrom<'a, Datum<'a>> for &'a str {
    fn borrow_from(&'a self) -> Datum<'a> {
        Datum { data: self.as_bytes() }
    }
}

fn foo_slice<'a, T>(t: &'a T) where T: BorrowFrom<'a, &'a [u8]> {
    let datum: &'a [u8] = t.borrow_from();
    println!("datum: {}", datum);
}

fn foo_str<'a, T>(t: &'a T) where T: BorrowFrom<'a, &'a str> {
    let datum: &'a str = t.borrow_from();
    println!("datum: {}", datum);
}

fn foo_custom<'a, T>(t: &'a T) where T: BorrowFrom<'a, Datum<'a>> {
    let datum: Datum<'a> = t.borrow_from();
    println!("datum: {}", datum);
}

fn main() {
    let s = "hello world".to_string();
    foo_slice(&s);
    foo_str(&s);
    foo_custom(&s);
}
