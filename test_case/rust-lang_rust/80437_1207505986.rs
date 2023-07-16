rust
    /// Registers a route middleware.
    ///
    /// `mw` is a middleware component (type), that can modify the requests and responses handled by
    /// this `Route`.
    ///
    /// See [`App::wrap`](crate::App::wrap) for more details.
    #[doc(alias = "middleware")]
    #[doc(alias = "use")] // nodejs terminology
    pub fn wrap<M, B>(self, mw: M) -> Route
    where
        M: Transform<
                BoxService<ServiceRequest, ServiceResponse, Error>,
                ServiceRequest,
                Response = ServiceResponse<B>,
                Error = Error,
                InitError = (),
            > + 'static,
        B: MessageBody + 'static,
    {
        Route {
            service: boxed::factory(apply(Compat::new(mw), self.service)),
            guards: self.guards,
        }
    }
 