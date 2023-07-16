
#[derive(Debug)]
struct Pair { lft: u8, rgt: u8, }

fn main() {
    let mut p = Pair { lft: 1, rgt: 2 };
    let mut f = move || { p.lft = 3; p.rgt };
    p.rgt = 4;
    println!("f: {:?}", f());
    p.lft = 5;
    println!("p: {:?}", p);
}
