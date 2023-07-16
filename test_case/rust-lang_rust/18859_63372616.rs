
#![feature(phase)]
#[phase(plugin, link)] extern crate log;

mod test {
    pub fn test() {
        info!("test");
    }
}

fn main() {
    let ret = 3i;
    info!("this function is about to return: {}", ret);
    test::test();
}
