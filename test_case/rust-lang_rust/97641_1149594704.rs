plain
   Compiling fluent-bundle v0.15.2
[RUSTC-TIMING] tracing test:false 1.395
[RUSTC-TIMING] object test:false 14.610
[RUSTC-TIMING] fluent_syntax test:false 0.884
error[E0310]: the parameter type `S` may not live long enough
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/filter/layer_filters.rs:184:16
    |
184 |     fn enabled(&self, meta: &Metadata<'_>, cx: &Context<'_, S>) -> bool {
    |                ^^^^^ ...so that the reference type `&Arc<(dyn layer::Filter<S> + Send + Sync + 'static)>` does not outlive the data it points at
help: consider adding an explicit lifetime bound...
    |
    |
182 | impl<S: 'static> layer::Filter<S> for Arc<dyn layer::Filter<S> + Send + Sync + 'static> {


error[E0310]: the parameter type `S` may not live long enough
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/filter/layer_filters.rs:189:25
    |
189 |     fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest {
    |                         ^^^^^ ...so that the reference type `&Arc<(dyn layer::Filter<S> + Send + Sync + 'static)>` does not outlive the data it points at
help: consider adding an explicit lifetime bound...
    |
    |
182 | impl<S: 'static> layer::Filter<S> for Arc<dyn layer::Filter<S> + Send + Sync + 'static> {


error[E0310]: the parameter type `S` may not live long enough
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/filter/layer_filters.rs:194:23
    |
194 |     fn max_level_hint(&self) -> Option<LevelFilter> {
    |                       ^^^^^ ...so that the reference type `&Arc<(dyn layer::Filter<S> + Send + Sync + 'static)>` does not outlive the data it points at
help: consider adding an explicit lifetime bound...
    |
    |
182 | impl<S: 'static> layer::Filter<S> for Arc<dyn layer::Filter<S> + Send + Sync + 'static> {


error[E0310]: the parameter type `S` may not live long enough
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/filter/layer_filters.rs:201:16
    |
201 |     fn enabled(&self, meta: &Metadata<'_>, cx: &Context<'_, S>) -> bool {
    |                ^^^^^ ...so that the reference type `&Box<(dyn layer::Filter<S> + Send + Sync + 'static)>` does not outlive the data it points at
help: consider adding an explicit lifetime bound...
    |
    |
199 | impl<S: 'static> layer::Filter<S> for Box<dyn layer::Filter<S> + Send + Sync + 'static> {


error[E0310]: the parameter type `S` may not live long enough
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/filter/layer_filters.rs:206:25
    |
206 |     fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest {
    |                         ^^^^^ ...so that the reference type `&Box<(dyn layer::Filter<S> + Send + Sync + 'static)>` does not outlive the data it points at
help: consider adding an explicit lifetime bound...
    |
    |
199 | impl<S: 'static> layer::Filter<S> for Box<dyn layer::Filter<S> + Send + Sync + 'static> {


error[E0310]: the parameter type `S` may not live long enough
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/filter/layer_filters.rs:211:23
    |
211 |     fn max_level_hint(&self) -> Option<LevelFilter> {
    |                       ^^^^^ ...so that the reference type `&Box<(dyn layer::Filter<S> + Send + Sync + 'static)>` does not outlive the data it points at
help: consider adding an explicit lifetime bound...
    |
    |
199 | impl<S: 'static> layer::Filter<S> for Box<dyn layer::Filter<S> + Send + Sync + 'static> {

For more information about this error, try `rustc --explain E0310`.
[RUSTC-TIMING] tracing_subscriber test:false 0.988
error: could not compile `tracing-subscriber` due to 6 previous errors
