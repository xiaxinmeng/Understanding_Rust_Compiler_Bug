
help: to declare that the trait object captures data from argument `f`, you can add a lifetime parameter `'a` in the type alias
   |
82 | type Lazy<T><'a> = Shared<Pin<Box<dyn Future<Output = T> + Send + 'static + 'a>>>;
   |             ++++                                                          ++++
