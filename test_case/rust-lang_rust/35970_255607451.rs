
trait T3 {}

trait My : std::str::FromStr {
    type Err: T3;
    fn test() {
        let _: Self::Err;
    }
}
