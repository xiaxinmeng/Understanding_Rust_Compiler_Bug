rust
async fn once_upon_a_time() {
    let _doggy = Bad { friend: () };
    async {}.await;
}

fn control() {
    enforce(once_upon_a_time());
}
fn enforce(_: impl Send) {}

type Bad = GenericBad<dyn LaserDoggy>;
struct GenericBad<T: Doggy+?Sized> {
    friend: <T as Doggy>::Friend,
}

trait Doggy {
    type Friend;
}
trait LaserDoggy {}
impl Doggy for dyn LaserDoggy {
    type Friend = ();
}

