rust
trait Svc<Req> {
    type Res;
}

mod http {
    struct Req;
    struct Res<B>(pub B);

    trait Body {}

    trait HttpSvc = <B> super::Svc<Req, Res = Res<B>> {
        type Res = B;
    }
}
