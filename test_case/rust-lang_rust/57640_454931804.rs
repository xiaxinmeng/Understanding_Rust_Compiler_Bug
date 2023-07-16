
async fn log_service(&self) -> T {
   let service = self.myService.foo(); // Only construction
   self.logger.log("beginning service call")(?);
   let output = service(?); // Actually wait for its result
   self.logger.log("foo executed with result {}.", output)(?);
   output
}
