
trait Channel {
    type ID;
    const CHANNEL: Self::ID;
}

impl<T> Channel for T {
    type ID = u8;
    const CHANNEL: u8 = 3;
}
