rust
impl Provider for MyRequestHandler {
    fn provide<'a>(&'a self, demand: &mut Demand<'a>) {
        demand.provide_ref::<String>(&self.some_string);
        demand.provide_ref::<Backtrace>(&self.bt);
        demand.provide_ref::<HttpErrorCode>(&self.code);
        demand.provide::<Trace>(self.logger.get_trace());
        /// bonus points: `RequestConext` implements `Provider`
        /// and it provides a middleware stack, where each also is a `Provider`
        demand.provide::<RequestContext>(self.context.clone());
        // ... 
    }
}
