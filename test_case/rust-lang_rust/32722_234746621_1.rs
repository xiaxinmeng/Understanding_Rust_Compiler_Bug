
trait SessionType {
    type Dual;
}

trait Recv: SessionType
    where Self::Dual: Send {
}

trait Send: SessionType
    where Self::Dual: Recv {
}

fn main() {}
