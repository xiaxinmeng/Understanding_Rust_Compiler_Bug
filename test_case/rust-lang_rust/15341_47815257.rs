
extern crate test;

#[deriving(Show)]
enum Instruction {
    Jump(i32),
    Match(bool),
    Text(&'static str),
}

fn gen_instr() -> Vec<Instruction>
{
    range(0i32, 10000).map(|x| match x % 3 {
        0 => Jump(x),
        1 => Match(x % 7 == 0),
        _ => Text("test-text")
    }).collect()
}

#[bench]
fn print_single(b: &mut test::Bencher)
{
    b.iter(|| {
        println!("abc");
    })
}

#[bench]
fn print_all(b: &mut test::Bencher)
{
    let instr = gen_instr();
    b.iter(|| {
        for (i, x) in instr.iter().enumerate() {
            println!("{} = {}", i, x);
        }
    })
}
