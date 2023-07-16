plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0277]: the trait bound `Option<AbstractConst<'_>>: ProcessQueryValue<'_, Result<Option<&AbstractConst<'tcx>>, ErrorGuaranteed>>` is not satisfied
   --> compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:88:22
78  | / macro_rules! provide_one {
78  | / macro_rules! provide_one {
79  | |     (<$lt:tt> $tcx:ident, $def_id:ident, $other:ident, $cdata:ident, $name:ident => { table }) => {
80  | |         provide_one! {
81  | |             <$lt> $tcx, $def_id, $other, $cdata, $name => {
...   |
88  | |                     .process_decoded($tcx, || panic!("{:?} does not have a {:?}", $def_id, stringify!($name)))
    | |                      ^^^^^^^^^^^^^^^ the trait `ProcessQueryValue<'_, Result<Option<&AbstractConst<'tcx>>, ErrorGuaranteed>>` is not implemented for `Option<AbstractConst<'_>>`
129 | |     };
130 | | }
    | |_- in this expansion of `provide_one!` (#2)
131 |
131 |
132 | / macro_rules! provide {
133 | |     (<$lt:tt> $tcx:ident, $def_id:ident, $other:ident, $cdata:ident,
134 | |       $($name:ident => { $($compute:tt)* })*) => {
135 | |         pub fn provide_extern(providers: &mut ExternProviders) {
136 |               $(provide_one! {
    |  _______________-
137 |                   <$lt> $tcx, $def_id, $other, $cdata, $name => { $($compute)* }
138 | |             })*
...
145 | |     }
146 | | }
    | |_- in this expansion of `provide!` (#1)
    | |_- in this expansion of `provide!` (#1)
...
190 | / provide! { <'tcx> tcx, def_id, other, cdata,
191 | |     explicit_item_bounds => { table }
192 | |     explicit_predicates_of => { table }
193 | |     generics_of => { table }
...   |
319 | |     generator_diagnostic_data => { cdata.get_generator_diagnostic_data(tcx, def_id.index) }
    | |_- in this macro invocation (#1)
    |
    |
    = help: the following other types implement trait `ProcessQueryValue<'tcx, T>`:
              <Option<DecodeIterator<'a, 'tcx, T>> as ProcessQueryValue<'tcx, &'tcx [T]>>
              <Option<Deprecation> as ProcessQueryValue<'_, Option<DeprecationEntry>>>
              <Option<T> as ProcessQueryValue<'_, Option<T>>>
              <Option<T> as ProcessQueryValue<'_, Result<Option<T>, E>>>
              <Option<T> as ProcessQueryValue<'_, T>>
              <Option<T> as ProcessQueryValue<'tcx, &'tcx T>>
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_metadata` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_metadata` due to previous error
