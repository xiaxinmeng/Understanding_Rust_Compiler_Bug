
extern mod extra;
use extra::arc::RWARC;

fn main() {
    let arc1  = ~RWARC(true);
    let _arc2 = ~RWARC(arc1);
}
