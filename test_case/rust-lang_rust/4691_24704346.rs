 rust
use NC = std::util::NonCopyable;

#[cfg(version2)]
struct S { a: int, b: int, p: ~str, nc: NC }
#[cfg(version2)]
fn make_s(a: int) -> S { S{ a: a, b: 0, p: ~"A", nc: NC } }
#[cfg(version2)]
fn ownmut(s: &mut S) { s.b = 1; s.p.push_char('B'); }

#[cfg(not(version2))]
struct S { a: int, b: int, nc: NC }
#[cfg(not(version2))]
fn make_s(a: int) -> S { S{ a: a, b: 0, nc: NC } }
#[cfg(not(version2))]
fn ownmut(s: &mut S) { s.b = 1; }

impl ToStr for S {
    #[cfg(version2)]
    fn to_str(&self) -> ~str {
        "S { a: "+ self.a.to_str() + 
            ", b: "+ self.b.to_str() +
            ", p: "+ self.p + " (nc) }"
    }
    #[cfg(not(version2))]
    fn to_str(&self) -> ~str {
        "S { a: "+ self.a.to_str() +
            ", b: "+ self.b.to_str() + " (nc) }"
    }
}

impl Drop for S {
    fn drop(&mut self) {
        println!("{} is dropped", self.to_str());
    }
}

fn main() {
    let s0 = make_s(0);
    println!("s0: {}", s0.to_str());

    // Shouldn't be allowed: A move out of value with a dtor via
    // Functional Struct Update (FSU):
    let mut s2 = S{a: 2, ..s0};

    // Uncommenting below will fail to compile, b/c s0 has been moved
    // println!("s0: {}", s0.to_str());

    ownmut(&mut s2);

    println!("s2: {}", s2.to_str());
}
