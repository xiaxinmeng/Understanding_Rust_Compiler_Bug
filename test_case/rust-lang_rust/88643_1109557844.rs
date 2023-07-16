rust
trait Task<D> {}
struct Domain;
struct Collect;
impl Task<Domain> for Collect {}

fn main() {
    const POSSIBLE_TASKS: [dyn Task<_>] = [Collect];
}
