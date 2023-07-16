rust
fn foo(&self) -> impl Future<Output = T> {
    async {
        let result = self.my_service.foo();
        self.logger.log("foo executed with result {}.", result);
        let bars: Vec<Bar> = Vec::new();
        for i in 0..100 {
           bars.push(self.my_other_service.bar(i, result));
        }
        result
    }
}
