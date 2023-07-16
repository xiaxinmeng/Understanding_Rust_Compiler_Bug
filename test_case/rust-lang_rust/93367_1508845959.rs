rust
trait Group {
    type Container: AsRef<dyn Item<Self>> + Clone;
}

trait Item<G: Group> {
    fn make_owned(&self) -> G::Container;
}

struct View<'a, G> {
    item: &'a dyn Item<G>,
}

// Rc
struct RcGroup;
impl Group for RcGroup {
    type Container = std::rc::Rc<dyn Item<Self>>;
}

// Arc
struct ArcGroup;
impl Group for ArcGroup {
    type Container = std::sync::Arc<dyn Item<Self>>;
}

unsafe impl<'a> Send for (dyn Item<ArcGroup> + 'a) {}   // <---

fn test_send<T: Send>() {}
fn should_compile() {
    test_send::<View<ArcGroup>>();
}
