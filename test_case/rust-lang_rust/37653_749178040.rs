rust
default impl<A: ToRoute> Routable for A {
    type Route = <A as ToRoute>::Route;
    fn route(&self) -> Self::Route {
        self.to_route()
    }
}

default impl<B: ToString> Routable for B {
    type Route = DefaultRoute;
    fn route(&self) -> Self::Route {
        DefaultRoute::from(self.to_string())
    }
}
