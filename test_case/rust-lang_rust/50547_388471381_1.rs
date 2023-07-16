rust
async fn foo(&self) -> T {
   let result = await self.myService.foo();
   self.logger.log("foo executed with result {}.", result);
   result
}
