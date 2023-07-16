
trait Danger {}
struct DangerDummy;
impl Danger for DangerDummy {}

static mut danger: *mut Danger = (&mut DangerDummy as *mut DangerDummy) as *mut Danger;

fn main() {
}
