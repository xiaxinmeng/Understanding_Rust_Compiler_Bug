plain
[00:37:52]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:37:52] error[E0433]: failed to resolve. Could not find `ItemKind` in `self`
[00:37:52]   --> librustdoc/clean/mod.rs:16:15
[00:37:52]    |
[00:37:52] 16 | pub use self::ItemKind::Enum::*;
[00:37:52]    |               ^^^^^^^^ Could not find `ItemKind` in `self`
[00:37:52] error[E0432]: unresolved import
[00:37:52]   --> librustdoc/clean/inline.rs:28:19
[00:37:52]    |
[00:37:52]    |
[00:37:52] 28 | use clean::{self, GetDefId, ToSource, get_auto_traits_with_def_id};
[00:37:52] 
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:364:16
[00:37:53]     |
[00:37:53]     |
[00:37:53] 364 |     pub inner: ItemKind::Enum,
[00:37:53]     |                ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:536:22
[00:37:53]     |
[00:37:53]     |
[00:37:53] 536 |     StrippedItem(Box<ItemKind::Enum>),
[00:37:53]     |                      ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:540:6
[00:37:53]     |
[00:37:53] 540 | impl ItemKind::Enum {
[00:37:53] 540 | impl ItemKind::Enum {
[00:37:53]     |      ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] 
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:543:13
[00:37:53]     |
[00:37:53] 543 |             ItemKind::Enum::StructItem(ref s) => &s.generics,
[00:37:53]     |             ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:544:13
[00:37:53]     |
[00:37:53]     |
[00:37:53] 544 |             ItemKind::Enum::EnumItem(ref e) => &e.generics,
[00:37:53]     |             ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:545:13
[00:37:53]     |
[00:37:53]     |
[00:37:53] 545 |             ItemKind::Enum::FunctionItem(ref f) => &f.generics,
[00:37:53]     |             ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:546:13
[00:37:53]     |
[00:37:53]     |
[00:37:53] 546 |             ItemKind::Enum::TypedefItem(ref t, _) => &t.generics,
[00:37:53]     |             ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:547:13
[00:37:53]     |
[00:37:53]     |
[00:37:53] 547 |             ItemKind::Enum::TraitItem(ref t) => &t.generics,
[00:37:53]     |             ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:548:13
[00:37:53]     |
[00:37:53]     |
[00:37:53] 548 |             ItemKind::Enum::ImplItem(ref i) => &i.generics,
[00:37:53]     |             ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:549:13
[00:37:53]     |
[00:37:53]     |
[00:37:53] 549 |             ItemKind::Enum::TyMethodItem(ref i) => &i.generics,
[00:37:53]     |             ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:550:13
[00:37:53]     |
[00:37:53]     |
[00:37:53] 550 |             ItemKind::Enum::MethodItem(ref i) => &i.generics,
[00:37:53]     |             ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]    --> librustdoc/clean/mod.rs:551:13
[00:37:53]     |
[00:37:53]     |
[00:37:53] 551 |             ItemKind::Enum::ForeignFunctionItem(ref f) => &f.generics,
[00:37:53]     |             ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] 
[00:37:53] error[E0408]: variable `TyInfer` is not bound in all patterns
[00:37:53]     --> librustdoc/clean/mod.rs:2990:23
[00:37:53]      |
[00:37:53] 2990 |             TyInfer | TyErr => Infer,
[00:37:53]      |             -------   ^^^^^ pattern doesn't bind `TyInfer`
[00:37:53]      |             variable not in all patterns
[00:37:53] 
[00:37:53] 
[00:37:53] error[E0408]: variable `TyErr` is not bound in all patterns
[00:37:53]     --> librustdoc/clean/mod.rs:2990:13
[00:37:53]      |
[00:37:53] 2990 |             TyInfer | TyErr => Infer,
[00:37:53]      |             ^^^^^^^   ----- variable not in all patterns
[00:37:53]      |             |
[00:37:53]      |             pattern doesn't bind `TyErr`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]   --> librustdoc/fold.rs:41:43
[00:37:53]    |
[00:37:53]    |
[00:37:53] 41 |     fn fold_inner_recur(&mut self, inner: ItemKind::Enum) -> ItemKind::Enum {
[00:37:53]    |                                           ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] error[E0433]: failed to resolve. Use of undeclared type or module `ItemKind`
[00:37:53]   --> librustdoc/fold.rs:41:62
[00:37:53]    |
[00:37:53]    |
[00:37:53] 41 |     fn fold_inner_recur(&mut self, inner: ItemKind::Enum) -> ItemKind::Enum {
[00:37:53]    |                                                              ^^^^^^^^ Use of undeclared type or module `ItemKind`
[00:37:53] 
[00:37:57] error[E0433]: failed to resolve. Could not find `ItemKind` in `clean`
[00:37:57]     --> librustdoc/html/render.rs:4178:16
[00:37:57]      |
[00:37:57] 4178 |         clean::ItemKind::Enum::ImplItem(ref i) => {
[00:37:57]      |                ^^^^^^^^ Could not find `ItemKind` in `clean`
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `TraitItem` in module `clean`
[00:37:57]   --> librustdoc/clean/inline.rs:54:20
[00:37:57]    |
[00:37:57] 54 |             clean::TraitItem(build_external_trait(cx, did))
[00:37:57]    |                    ^^^^^^^^^ not found in `clean`
[00:37:57]    |
[00:37:57]    |
[00:37:57] 13 | use clean::ItemEnum::TraitItem;
[00:37:57]    |
[00:37:57] 13 | use rustc::hir::print::Nested::TraitItem;
[00:37:57]    |
[00:37:57] 13 | use syntax::ext::base::Annotatable::TraitItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `FunctionItem` in module `clean`
[00:37:57]   --> librustdoc/clean/inline.rs:58:20
[00:37:57]    |
[00:37:57] 58 |             clean::FunctionItem(build_external_function(cx, did))
[00:37:57]    |                    ^^^^^^^^^^^^ not found in `clean`
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]    |
[00:37:57] 13 | use clean::ItemEnum::FunctionItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `StructItem` in module `clean`
[00:37:57]   --> librustdoc/clean/inline.rs:63:20
[00:37:57]    |
[00:37:57] 63 |             clean::StructItem(build_struct(cx, did))
[00:37:57]    |                    ^^^^^^^^^^ not found in `clean`
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]    |
[00:37:57] 13 | use clean::ItemEnum::StructItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `UnionItem` in module `clean`
[00:37:57]   --> librustdoc/clean/inline.rs:68:20
[00:37:57]    |
[00:37:57] 68 |             clean::UnionItem(build_union(cx, did))
[00:37:57]    |                    ^^^^^^^^^ not found in `clean`
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]    |
[00:37:57] 13 | use clean::ItemEnum::UnionItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `TypedefItem` in module `clean`
[00:37:57]   --> librustdoc/clean/inline.rs:73:20
[00:37:57]    |
[00:37:57] 73 |             clean::TypedefItem(build_type_alias(cx, did), false)
[00:37:57]    |                    ^^^^^^^^^^^ not found in `clean`
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]    |
[00:37:57] 13 | use clean::ItemEnum::TypedefItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `EnumItem` in module `clean`
[00:37:57]   --> librustdoc/clean/inline.rs:78:20
[00:37:57]    |
[00:37:57] 78 |             clean::EnumItem(build_enum(cx, did))
[00:37:57]    |                    ^^^^^^^^ not found in `clean`
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]    |
[00:37:57] 13 | use clean::ItemEnum::EnumItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find value `ForeignTypeItem` in module `clean`
[00:37:57]   --> librustdoc/clean/inline.rs:83:20
[00:37:57] 83 |             clean::ForeignTypeItem
[00:37:57]    |                    ^^^^^^^^^^^^^^^ not found in `clean`
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]    |
[00:37:57]    |
[00:37:57] 13 | use clean::ItemEnum::ForeignTypeItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `ModuleItem` in module `clean`
[00:37:57]   --> librustdoc/clean/inline.rs:93:20
[00:37:57]    |
[00:37:57] 93 |             clean::ModuleItem(build_module(cx, did, visited))
[00:37:57]    |                    ^^^^^^^^^^ not found in `clean`
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]    |
[00:37:57] 13 | use clean::ItemEnum::ModuleItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `StaticItem` in module `clean`
[00:37:57]   --> librustdoc/clean/inline.rs:97:20
[00:37:57]    |
[00:37:57] 97 |             clean::StaticItem(build_static(cx, did, mtbl))
[00:37:57]    |                    ^^^^^^^^^^ not found in `clean`
[00:37:57]    |
[00:37:57]    |
[00:37:57] 13 | use clean::ItemEnum::StaticItem;
[00:37:57]    |
[00:37:57] 13 | use rustc::middle::mem_categorization::Categorization::StaticItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `ConstantItem` in module `clean`
[00:37:57]    --> librustdoc/clean/inline.rs:101:20
[00:37:57]     |
[00:37:57] 101 |             clean::ConstantItem(build_const(cx, did))
[00:37:57]     |                    ^^^^^^^^^^^^ not found in `clean`
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57] 13  | use clean::ItemEnum::ConstantItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `MacroItem` in module `clean`
[00:37:57]    --> librustdoc/clean/inline.rs:107:20
[00:37:57]     |
[00:37:57] 107 |             clean::MacroItem(build_macro(cx, did, name))
[00:37:57]     |                    ^^^^^^^^^ not found in `clean`
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57] 13  | use clean::ItemEnum::MacroItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `ImplItem` in module `clean`
[00:37:57]    --> librustdoc/clean/inline.rs:405:23
[00:37:57]     |
[00:37:57] 405 |         inner: clean::ImplItem(clean::Impl {
[00:37:57]     |                       ^^^^^^^^ not found in `clean`
[00:37:57]     |
[00:37:57]     |
[00:37:57] 13  | use clean::ItemEnum::ImplItem;
[00:37:57]     |
[00:37:57] 13  | use rustc::hir::print::Nested::ImplItem;
[00:37:57]     |
[00:37:57] 13  | use syntax::ext::base::Annotatable::ImplItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `ImplItem` in this scope
[00:37:57]    --> librustdoc/clean/auto_trait.rs:232:24
[00:37:57]     |
[00:37:57] 232 |                 inner: ImplItem(Impl {
[00:37:57] help: possible candidates are found in other modules, you can import them into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 11  | use clean::ItemEnum::ImplItem;
[00:37:57]     |
[00:37:57] 11  | use rustc::hir::print::Nested::ImplItem;
[00:37:57]     |
[00:37:57] 11  | use syntax::ext::base::Annotatable::ImplItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0531]: cannot find tuple struct/variant `ModuleItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:171:13
[00:37:57] 171 |             ModuleItem(ref module) => {
[00:37:57]     |             ^^^^^^^^^^ not found in this scope
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::ModuleItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0531]: cannot find tuple struct/variant `ModuleItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:184:17
[00:37:57]     |
[00:37:57] 184 |                 ModuleItem(ref mut m) => m,
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::ModuleItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `PrimitiveItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:196:28
[00:37:57]     |
[00:37:57] 196 |                     inner: PrimitiveItem(prim),
[00:37:57]     |                            ^^^^^^^^^^^^^ did you mean `Primitive`?
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::PrimitiveItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `KeywordItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:208:28
[00:37:57]     |
[00:37:57] 208 |                     inner: KeywordItem(kw),
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::KeywordItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0531]: cannot find tuple struct/variant `StrippedItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:409:13
[00:37:57]     |
[00:37:57] 409 |             StrippedItem(box ModuleItem(Module { is_crate: true, ..})) |
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::StrippedItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0531]: cannot find tuple struct/variant `ModuleItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:409:30
[00:37:57]     |
[00:37:57] 409 |             StrippedItem(box ModuleItem(Module { is_crate: true, ..})) |
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::ModuleItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0531]: cannot find tuple struct/variant `ModuleItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:410:13
[00:37:57]     |
[00:37:57] 410 |             ModuleItem(Module { is_crate: true, ..}) => true,
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::ModuleItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0531]: cannot find tuple struct/variant `StrippedItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:461:28
[00:37:57]     |
[00:37:57] 461 |         match self.inner { StrippedItem(..) => true, _ => false }
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::StrippedItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0531]: cannot find tuple struct/variant `StructItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:465:13
[00:37:57]     |
[00:37:57] 465 |             StructItem(ref _struct) => Some(_struct.fields_stripped),
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::StructItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0531]: cannot find tuple struct/variant `UnionItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:466:13
[00:37:57]     |
[00:37:57] 466 |             UnionItem(ref union) => Some(union.fields_stripped),
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::UnionItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0531]: cannot find tuple struct/variant `VariantItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:467:13
[00:37:57]     |
[00:37:57] 467 |             VariantItem(Variant { kind: VariantKind::Struct(ref vstruct)} ) => {
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::VariantItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `ModuleItem` in this scope
[00:37:57]    --> librustdoc/clean/mod.rs:629:20
[00:37:57]     |
[00:37:57] 629 |             inner: ModuleItem(Module {
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]     |
[00:37:57]     |
[00:37:57] 14  | use clean::ItemEnum::ModuleItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `FunctionItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2121:20
[00:37:57]      |
[00:37:57] 2121 |             inner: FunctionItem(Function {
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::FunctionItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `TraitItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2301:20
[00:37:57]      |
[00:37:57] 2301 |             inner: TraitItem(Trait {
[00:37:57] help: possible candidates are found in other modules, you can import them into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::TraitItem;
[00:37:57]      |
[00:37:57] 14   | use rustc::hir::print::Nested::TraitItem;
[00:37:57]      |
[00:37:57] 14   | use syntax::ext::base::Annotatable::TraitItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `AssociatedConstItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2342:17
[00:37:57]      |
[00:37:57] 2342 |                 AssociatedConstItem(ty.clean(cx),
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::AssociatedConstItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `MethodItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2346:17
[00:37:57]      |
[00:37:57] 2346 |                 MethodItem((sig, &self.generics, body).clean(cx))
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::MethodItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `TyMethodItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2352:17
[00:37:57]      |
[00:37:57] 2352 |                 TyMethodItem(TyMethod {
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::TyMethodItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `AssociatedTypeItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2359:17
[00:37:57]      |
[00:37:57] 2359 |                 AssociatedTypeItem(bounds.clean(cx), default.clean(cx))
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::AssociatedTypeItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `AssociatedConstItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2379:17
[00:37:57]      |
[00:37:57] 2379 |                 AssociatedConstItem(ty.clean(cx),
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::AssociatedConstItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `MethodItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2383:17
[00:37:57]      |
[00:37:57] 2383 |                 MethodItem((sig, &self.generics, body).clean(cx))
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::MethodItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `TypedefItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2385:48
[00:37:57]      |
[00:37:57] 2385 |             hir::ImplItemKind::Type(ref ty) => TypedefItem(Typedef {
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::TypedefItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `AssociatedConstItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2413:17
[00:37:57]      |
[00:37:57] 2413 |                 AssociatedConstItem(ty.clean(cx), default)
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::AssociatedConstItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `MethodItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2453:21
[00:37:57]      |
[00:37:57] 2453 |                     MethodItem(Method {
[00:37:57] help: possible candidate is found in another module, you can import it into scope
[00:37:57]      |
[00:37:57]      |
[00:37:57] 14   | use clean::ItemEnum::MethodItem;
[00:37:57] 
[00:37:57] 
[00:37:57] error[E0425]: cannot find function `TyMethodItem` in this scope
[00:37:57]     --> librustdoc/clean/mod.rs:2464:21
[00:37:57]      |
