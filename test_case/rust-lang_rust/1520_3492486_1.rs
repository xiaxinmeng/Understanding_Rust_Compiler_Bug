
impl of mul<int, str> for str {
    fn mul(times: int) -> str {
        let base = self;
        uint::range(1, times) {|| base += self; }
    }
}
