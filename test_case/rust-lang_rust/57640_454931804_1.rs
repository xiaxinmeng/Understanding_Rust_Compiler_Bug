
async fn log_service(&self) -> T {
   let service = self.myService.foo(); // Only construction
   self.logger.log("beginning service call")(await);
   let output = service(await);
   self.logger.log("foo executed with result {}.", output)(await);
   output
}
