rust
fn foo(&self) -> impl Future<Output = T> {
    async {
        let result = self.my_service.foo();
        self.logger.log("foo executed with result {}.", result);
        result
    }
}
