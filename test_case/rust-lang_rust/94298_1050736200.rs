plain
   Compiling askama_shared v0.12.0
   Compiling askama_derive v0.11.0
   Compiling askama v0.11.0
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error: using `iter` can result in unstable query results
   --> src/librustdoc/clean/auto_trait.rs:249:41
    |
249 |             for smaller in deps.smaller.iter() {
    |
    |
    = note: `-D rustc::potential-query-instability` implied by `-D warnings`
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   --> src/librustdoc/clean/auto_trait.rs:250:43
    |
250 |                 for larger in deps.larger.iter() {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   --> src/librustdoc/clean/auto_trait.rs:293:14
293 |             .iter()
    |              ^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/clean/auto_trait.rs:308:36
308 |                     bounds: bounds.into_iter().collect(),
    |                                    ^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/clean/auto_trait.rs:330:14
330 |             .into_iter()
    |              ^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/clean/auto_trait.rs:352:14
352 |             .into_iter()
    |              ^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/clean/auto_trait.rs:393:45
393 |                 let mut bounds_vec = bounds.into_iter().collect();
    |                                             ^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/clean/auto_trait.rs:403:36
    |
403 |                 lifetime_to_bounds.into_iter().filter(|&(_, ref bounds)| !bounds.is_empty()).map(
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/clean/auto_trait.rs:405:53
405 |                         let mut bounds_vec = bounds.into_iter().collect();
    |                                                     ^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/clean/auto_trait.rs:524:65
    |
524 | ...                   p.generic_params = for_generics.into_iter().collect();
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    --> src/librustdoc/clean/types.rs:1109:17
1109 |         aliases.into_iter().collect::<Vec<_>>().into()
     |                 ^^^^^^^^^
     |
     |
     = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/clean/mod.rs:169:10
169 |         .into_iter()
    |          ^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
    |
    |
340 |         map: tcx.privacy_access_levels(()).map.iter().map(|(k, v)| (k.to_def_id(), *v)).collect(),
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
    |
182 |                 .iter()
    |                  ^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `intersection` can result in unstable query results
    |
    |
180 |                     uextsa.intersection(&uextsb).copied().collect::<FxHashSet<&String>>()
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    |
114 |         for (k, mut v) in external_traits {
    |                           ^^^^^^^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    |
    |
164 |         for (prim, &def_id) in &cx.cache.primitive_locations {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    |
    |
177 |                 for did in dids {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    |
    |
466 |                 for did in dids {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/html/highlight.rs:276:14
276 |             .into_iter()
    |              ^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
  --> src/librustdoc/html/layout.rs:44:10
44 | #[derive(Template)]
   |          ^^^^^^^^ in this derive macro expansion
   |
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/askama_derive-0.11.0/src/lib.rs:16:1
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/askama_derive-0.11.0/src/lib.rs:16:1
   |
16 | pub fn derive_template(input: TokenStream) -> TokenStream {
   | --------------------------------------------------------- in this expansion of `#[derive(Template)]`
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `drain` can result in unstable query results
    |
    |
692 |                         let mut v: Vec<_> = self.footnotes.drain().map(|(_, x)| x).collect();
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   --> src/librustdoc/html/render/write_shared.rs:257:43
    |
257 |     let mut themes: Vec<&String> = themes.iter().collect();
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/html/render/write_shared.rs:290:29
    |
290 |     for (name, contents) in &*FILES_UNVERSIONED {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   --> src/librustdoc/html/render/write_shared.rs:371:18
371 |                 .iter()
    |                  ^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   --> src/librustdoc/html/render/write_shared.rs:395:14
395 |             .iter()
    |              ^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/html/render/write_shared.rs:505:25
    |
505 |     for (&did, imps) in &cache.implementors {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   --> src/librustdoc/html/render/mod.rs:294:48
    |
294 |                 let mut e: Vec<&ItemEntry> = e.iter().collect();
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    --> src/librustdoc/html/render/mod.rs:2789:39
2789 |         let mut locs = call_locations.into_iter().collect::<Vec<_>>();
     |                                       ^^^^^^^^^
     |
     |
     = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/json/conversions.rs:537:18
537 |                 .into_iter()
    |                  ^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   --> src/librustdoc/json/mod.rs:102:14
102 |             .iter()
    |              ^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/json/mod.rs:223:26
223 |             index: index.into_iter().collect(),
    |                          ^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/json/mod.rs:228:18
228 |                 .into_iter()
    |                  ^^^^^^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/json/mod.rs:229:58
    |
229 |                 .chain(self.cache.external_paths.clone().into_iter())
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   --> src/librustdoc/json/mod.rs:244:18
244 |                 .iter()
    |                  ^^^^
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> src/librustdoc/passes/collect_intra_doc_links/early.rs:157:37
    |
157 |         for (parent_module, doc) in attrs.collapsed_doc_value_by_module_level() {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
    |
    |
866 |     let mut candidates = traits.iter().filter_map(|&(impl_, trait_)| {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
     |
     |
1030 |         for (parent_module, doc) in item.attrs.collapsed_doc_value_by_module_level() {
     |
     |
     = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    |
    |
304 |             for (function, fn_calls) in calls.into_iter() {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    |
    |
305 |                 all_calls.entry(function).or_default().extend(fn_calls.into_iter());
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   |
   |
26 |             for child in &self.children {
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   |
   |
27 |                 if !other.children.iter().any(|c| child == c) {
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   |
   |
39 |         for x in &self.children {
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
    |
    |
203 |                 for entry in inner(v, events, pos).iter() {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    |
    |
226 |         for child in &against.children {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
    |
    |
231 |             for other_child in &other.children {
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   |
66 |         for (k, v) in external_traits {
   |                       ^^^^^^^^^^^^^^^
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
error: could not compile `rustdoc` due to 47 previous errors
Build completed unsuccessfully in 0:15:17
