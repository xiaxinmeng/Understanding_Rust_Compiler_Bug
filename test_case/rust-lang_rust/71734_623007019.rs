rust
  pub trait ServiceMarker: Sized + Send + Sync + 'static {
      /// The type of the structure against which FIDL requests are made.
      /// Queries made against the proxy are sent to the paired `ServerEnd`.
      type Proxy: Proxy<Service = Self>;

      /// The type of the stream of requests coming into a server.
      type RequestStream: RequestStream<Service = Self>;

      /// The name of the service suitable for debug purposes.
      ///
      /// This will be removed-- users should switch to either
      /// `DEBUG_NAME` or `DiscoverableService::NAME`.
      const NAME: &'static str = Self::DEBUG_NAME;

      /// The name of the service suitable for debug purposes.
      ///
      /// For discoverable services, this should be identical to
      /// `<Self as DiscoverableService>::NAME`.
      const DEBUG_NAME: &'static str;
  }
