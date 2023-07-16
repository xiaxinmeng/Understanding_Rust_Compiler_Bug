rust
trait Service {
    type S;
}

trait Framing {
    type F;
}

impl Framing for () {
    type F = ();
}

impl Framing for u32 {
    type F = u32;
}

trait HttpService<F: Framing>: Service<S = F::F> {}

type BoxService = Box<dyn HttpService<u32, S = ()>>;

fn build_server<F: FnOnce() -> BoxService>(_: F) {}

fn make_server<F: Framing>() -> Box<dyn HttpService<F, S = F::F>> {
    unimplemented!()
}

fn main() {
    build_server(|| make_server())
}
