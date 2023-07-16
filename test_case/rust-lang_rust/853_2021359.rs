
fn force(f: &block()) { f() }
fn main() { 
    let i = 4;
    force({ | | 
        let j <- i;
        log_err j;
    });
    log_err i;
}
