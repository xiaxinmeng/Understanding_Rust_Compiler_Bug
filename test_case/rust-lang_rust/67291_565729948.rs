plain
2019-12-14T15:16:37.8901671Z 
2019-12-14T15:16:37.8901784Z error: missing documentation for a type alias
2019-12-14T15:16:37.8908617Z   --> $DIR/missing-doc.rs:11:1
2019-12-14T15:16:37.8908726Z    |
2019-12-14T15:16:37.8908820Z LL | pub type PubTypedef = String;
2019-12-14T15:16:37.8908970Z 
2019-12-14T15:16:37.8909061Z error: missing documentation for a struct
2019-12-14T15:16:37.8909346Z   --> $DIR/missing-doc.rs:13:1
2019-12-14T15:16:37.8909445Z    |
2019-12-14T15:16:37.8909445Z    |
2019-12-14T15:16:37.8909514Z LL | / struct Foo {
2019-12-14T15:16:37.8909608Z LL | |     a: isize,
2019-12-14T15:16:37.8909683Z LL | |     b: isize,
2019-12-14T15:16:37.8910104Z    | |_^
2019-12-14T15:16:37.8910162Z 
2019-12-14T15:16:37.8910240Z error: missing documentation for a struct field
2019-12-14T15:16:37.8910573Z   --> $DIR/missing-doc.rs:14:5
2019-12-14T15:16:37.8910573Z   --> $DIR/missing-doc.rs:14:5
2019-12-14T15:16:37.8910672Z    |
2019-12-14T15:16:37.8910742Z LL |     a: isize,
2019-12-14T15:16:37.8910881Z 
2019-12-14T15:16:37.8911006Z error: missing documentation for a struct field
2019-12-14T15:16:37.8911295Z   --> $DIR/missing-doc.rs:15:5
2019-12-14T15:16:37.8911527Z    |
2019-12-14T15:16:37.8911527Z    |
2019-12-14T15:16:37.8911596Z LL |     b: isize,
2019-12-14T15:16:37.8911732Z 
2019-12-14T15:16:37.8911803Z error: missing documentation for a struct
2019-12-14T15:16:37.8912120Z   --> $DIR/missing-doc.rs:18:1
2019-12-14T15:16:37.8912218Z    |
2019-12-14T15:16:37.8912218Z    |
2019-12-14T15:16:37.8912288Z LL | / pub struct PubFoo {
2019-12-14T15:16:37.8912381Z LL | |     pub a: isize,
2019-12-14T15:16:37.8912456Z LL | |     b: isize,
2019-12-14T15:16:37.8912625Z    | |_^
2019-12-14T15:16:37.8912684Z 
2019-12-14T15:16:37.8912758Z error: missing documentation for a struct field
2019-12-14T15:16:37.8913046Z   --> $DIR/missing-doc.rs:19:5
2019-12-14T15:16:37.8913046Z   --> $DIR/missing-doc.rs:19:5
2019-12-14T15:16:37.8913127Z    |
2019-12-14T15:16:37.8913212Z LL |     pub a: isize,
2019-12-14T15:16:37.8913352Z 
2019-12-14T15:16:37.8913428Z error: missing documentation for a struct field
2019-12-14T15:16:37.8913708Z   --> $DIR/missing-doc.rs:20:5
2019-12-14T15:16:37.8913798Z    |
2019-12-14T15:16:37.8913798Z    |
2019-12-14T15:16:37.8913883Z LL |     b: isize,
2019-12-14T15:16:37.8914020Z 
2019-12-14T15:16:37.8914092Z error: missing documentation for a module
2019-12-14T15:16:37.8914374Z   --> $DIR/missing-doc.rs:29:1
2019-12-14T15:16:37.8914453Z    |
2019-12-14T15:16:37.8914453Z    |
2019-12-14T15:16:37.8914539Z LL | mod module_no_dox {}
2019-12-14T15:16:37.8914682Z 
2019-12-14T15:16:37.8914762Z error: missing documentation for a module
2019-12-14T15:16:37.8915044Z   --> $DIR/missing-doc.rs:30:1
2019-12-14T15:16:37.8915123Z    |
2019-12-14T15:16:37.8915123Z    |
2019-12-14T15:16:37.8915208Z LL | pub mod pub_module_no_dox {}
2019-12-14T15:16:37.8915353Z 
2019-12-14T15:16:37.8915426Z error: missing documentation for a function
2019-12-14T15:16:37.8915704Z   --> $DIR/missing-doc.rs:33:1
2019-12-14T15:16:37.8915783Z    |
---
2019-12-14T15:16:37.8920040Z 
2019-12-14T15:16:37.8920109Z error: missing documentation for a trait
2019-12-14T15:16:37.8920390Z   --> $DIR/missing-doc.rs:53:1
2019-12-14T15:16:37.8920470Z    |
2019-12-14T15:16:37.8920556Z LL | / pub trait C {
2019-12-14T15:16:37.8920632Z LL | |     fn foo(&self);
2019-12-14T15:16:37.8920825Z LL | |     fn foo_with_impl(&self) {}
2019-12-14T15:16:37.8920985Z    | |_^
2019-12-14T15:16:37.8921025Z 
2019-12-14T15:16:37.8921117Z error: missing documentation for a trait method
2019-12-14T15:16:37.8921415Z   --> $DIR/missing-doc.rs:54:5
---
2019-12-14T15:16:37.8922404Z 
2019-12-14T15:16:37.8922491Z error: missing documentation for a trait
2019-12-14T15:16:37.8922833Z   --> $DIR/missing-doc.rs:64:1
2019-12-14T15:16:37.8922913Z    |
2019-12-14T15:16:37.8923009Z LL | / pub trait E {
2019-12-14T15:16:37.8923085Z LL | |     type AssociatedType;
2019-12-14T15:16:37.8923183Z LL | |     type AssociatedTypeDef = Self;
2019-12-14T15:16:37.8923345Z ...  |
2019-12-14T15:16:37.8923345Z ...  |
2019-12-14T15:16:37.8923414Z LL | |     fn dummy(&self) {}
2019-12-14T15:16:37.8923573Z    | |_^
2019-12-14T15:16:37.8923631Z 
2019-12-14T15:16:37.8923707Z error: missing documentation for an associated type
2019-12-14T15:16:37.8924009Z   --> $DIR/missing-doc.rs:65:5
---
2019-12-14T15:16:37.8926351Z 
2019-12-14T15:16:37.8926441Z error: missing documentation for a trait method
2019-12-14T15:16:37.8926709Z   --> $DIR/missing-doc.rs:73:5
2019-12-14T15:16:37.8926844Z    |
2019-12-14T15:16:37.8926912Z LL |     fn dummy(&self) {}
2019-12-14T15:16:37.8927055Z 
2019-12-14T15:16:37.8927151Z error: missing documentation for a method
2019-12-14T15:16:37.8927418Z   --> $DIR/missing-doc.rs:77:5
2019-12-14T15:16:37.8927512Z    |
---
2019-12-14T15:16:37.8930610Z 
2019-12-14T15:16:37.8930707Z error: missing documentation for an enum
2019-12-14T15:16:37.8930973Z   --> $DIR/missing-doc.rs:112:1
2019-12-14T15:16:37.8931069Z    |
2019-12-14T15:16:37.8931137Z LL | / enum Baz {
2019-12-14T15:16:37.8931228Z LL | |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.8931307Z LL | |     BarB,
2019-12-14T15:16:37.8931478Z    | |_^
2019-12-14T15:16:37.8931519Z 
2019-12-14T15:16:37.8931590Z error: missing documentation for a variant
2019-12-14T15:16:37.8931873Z   --> $DIR/missing-doc.rs:113:5
2019-12-14T15:16:37.8931873Z   --> $DIR/missing-doc.rs:113:5
2019-12-14T15:16:37.8931979Z    |
2019-12-14T15:16:37.8932047Z LL |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.8932196Z 
2019-12-14T15:16:37.8932270Z error: missing documentation for a struct field
2019-12-14T15:16:37.8932559Z   --> $DIR/missing-doc.rs:113:12
2019-12-14T15:16:37.8932657Z    |
2019-12-14T15:16:37.8932657Z    |
2019-12-14T15:16:37.8932726Z LL |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.8932880Z 
2019-12-14T15:16:37.8932971Z error: missing documentation for a struct field
2019-12-14T15:16:37.8933240Z   --> $DIR/missing-doc.rs:113:22
2019-12-14T15:16:37.8933337Z    |
2019-12-14T15:16:37.8933337Z    |
2019-12-14T15:16:37.8933405Z LL |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.8933552Z 
2019-12-14T15:16:37.8933638Z error: missing documentation for a variant
2019-12-14T15:16:37.8933903Z   --> $DIR/missing-doc.rs:114:5
2019-12-14T15:16:37.8934009Z    |
2019-12-14T15:16:37.8934009Z    |
2019-12-14T15:16:37.8934076Z LL |     BarB,
2019-12-14T15:16:37.8934167Z    |     ^^^^
2019-12-14T15:16:37.8934211Z 
2019-12-14T15:16:37.8934297Z error: missing documentation for an enum
2019-12-14T15:16:37.8934567Z   --> $DIR/missing-doc.rs:117:1
2019-12-14T15:16:37.8934677Z    |
2019-12-14T15:16:37.8934746Z LL | / pub enum PubBaz {
2019-12-14T15:16:37.8934838Z LL | |     PubBazA { a: isize },
2019-12-14T15:16:37.8934999Z    | |_^
2019-12-14T15:16:37.8935050Z 
2019-12-14T15:16:37.8935137Z error: missing documentation for a variant
2019-12-14T15:16:37.9039513Z   --> $DIR/missing-doc.rs:118:5
2019-12-14T15:16:37.9039513Z   --> $DIR/missing-doc.rs:118:5
2019-12-14T15:16:37.9039637Z    |
2019-12-14T15:16:37.9039706Z LL |     PubBazA { a: isize },
2019-12-14T15:16:37.9039853Z 
2019-12-14T15:16:37.9039947Z error: missing documentation for a struct field
2019-12-14T15:16:37.9040245Z   --> $DIR/missing-doc.rs:118:15
2019-12-14T15:16:37.9040345Z    |
2019-12-14T15:16:37.9040345Z    |
2019-12-14T15:16:37.9040433Z LL |     PubBazA { a: isize },
2019-12-14T15:16:37.9040579Z 
2019-12-14T15:16:37.9040668Z error: missing documentation for an enum
2019-12-14T15:16:37.9040959Z   --> $DIR/missing-doc.rs:122:1
2019-12-14T15:16:37.9041057Z    |
2019-12-14T15:16:37.9041057Z    |
2019-12-14T15:16:37.9041127Z LL | / pub enum PubBaz2 {
2019-12-14T15:16:37.9109863Z LL | |     /// dox
2019-12-14T15:16:37.9110030Z LL | |     PubBaz2A {
2019-12-14T15:16:37.9110130Z LL | |         /// dox
2019-12-14T15:16:37.9110448Z LL | |         a: isize,
2019-12-14T15:16:37.9110631Z LL | | }
2019-12-14T15:16:37.9110713Z    | |_^
2019-12-14T15:16:37.9110759Z 
2019-12-14T15:16:37.9110847Z error: missing documentation for a variant
2019-12-14T15:16:37.9110847Z error: missing documentation for a variant
2019-12-14T15:16:37.9111432Z   --> $DIR/missing-doc.rs:124:5
2019-12-14T15:16:37.9111536Z    |
2019-12-14T15:16:37.9111607Z LL | /     PubBaz2A {
2019-12-14T15:16:37.9111699Z LL | |         /// dox
2019-12-14T15:16:37.9111776Z LL | |         a: isize,
2019-12-14T15:16:37.9112120Z    | |_____^
2019-12-14T15:16:37.9112169Z 
2019-12-14T15:16:37.9112247Z error: missing documentation for a struct field
2019-12-14T15:16:37.9112614Z   --> $DIR/missing-doc.rs:126:9
2019-12-14T15:16:37.9112614Z   --> $DIR/missing-doc.rs:126:9
2019-12-14T15:16:37.9112699Z    |
2019-12-14T15:16:37.9112786Z LL |         a: isize,
2019-12-14T15:16:37.9112928Z 
2019-12-14T15:16:37.9113004Z error: missing documentation for a constant
2019-12-14T15:16:37.9113323Z   --> $DIR/missing-doc.rs:138:1
2019-12-14T15:16:37.9113407Z    |
---
2019-12-14T15:16:37.9121482Z 
2019-12-14T15:16:37.9121555Z error: missing documentation for a module
2019-12-14T15:16:37.9121884Z   --> $DIR/missing-doc.rs:163:5
2019-12-14T15:16:37.9121968Z    |
2019-12-14T15:16:37.9122050Z LL | /     pub mod globbed {
2019-12-14T15:16:37.9122129Z LL | |         /// dox
2019-12-14T15:16:37.9122224Z LL | |         pub fn also_documented() {}
2019-12-14T15:16:37.9122315Z LL | |         pub fn also_undocumented1() {}
2019-12-14T15:16:37.9122427Z LL | |         fn also_undocumented2() {}
2019-12-14T15:16:37.9122598Z    | |_____^
2019-12-14T15:16:37.9122645Z 
2019-12-14T15:16:37.9122734Z error: missing documentation for a function
2019-12-14T15:16:37.9123140Z   --> $DIR/missing-doc.rs:165:9
---
2019-12-14T15:16:37.9124960Z 
2019-12-14T15:16:37.9125946Z error: missing documentation for a module
2019-12-14T15:16:37.9126345Z   --> $DIR/missing-doc.rs:171:1
2019-12-14T15:16:37.9126429Z    |
2019-12-14T15:16:37.9126514Z LL | / pub mod public_interface {
2019-12-14T15:16:37.9126603Z LL | |     pub use internal_impl::documented as foo;
2019-12-14T15:16:37.9126701Z LL | |     pub use internal_impl::globbed::*;
2019-12-14T15:16:37.9126806Z LL | |     pub use internal_impl::undocumented1 as bar;
2019-12-14T15:16:37.9126918Z LL | |     pub use internal_impl::{documented, undocumented2};
2019-12-14T15:16:37.9127089Z    | |_^
2019-12-14T15:16:37.9127131Z 
2019-12-14T15:16:37.9127214Z error: aborting due to 57 previous errors
2019-12-14T15:16:37.9127266Z 
---
2019-12-14T15:16:37.9128881Z 
2019-12-14T15:16:37.9128956Z error: missing documentation for a type alias
2019-12-14T15:16:37.9129302Z   --> $DIR/missing-doc.rs:11:1
2019-12-14T15:16:37.9129381Z    |
2019-12-14T15:16:37.9129460Z LL | pub type PubTypedef = String;
2019-12-14T15:16:37.9129606Z 
2019-12-14T15:16:37.9129677Z error: missing documentation for a struct
2019-12-14T15:16:37.9129953Z   --> $DIR/missing-doc.rs:13:1
2019-12-14T15:16:37.9130032Z    |
2019-12-14T15:16:37.9130032Z    |
2019-12-14T15:16:37.9130114Z LL | / struct Foo {
2019-12-14T15:16:37.9130321Z LL | |     a: isize,
2019-12-14T15:16:37.9130423Z LL | |     b: isize,
2019-12-14T15:16:37.9130575Z    | |_^
2019-12-14T15:16:37.9130616Z 
2019-12-14T15:16:37.9130706Z error: missing documentation for a struct field
2019-12-14T15:16:37.9131013Z   --> $DIR/missing-doc.rs:14:5
2019-12-14T15:16:37.9131013Z   --> $DIR/missing-doc.rs:14:5
2019-12-14T15:16:37.9131106Z    |
2019-12-14T15:16:37.9131186Z LL |     a: isize,
2019-12-14T15:16:37.9131307Z 
2019-12-14T15:16:37.9131491Z error: missing documentation for a struct field
2019-12-14T15:16:37.9131849Z   --> $DIR/missing-doc.rs:15:5
2019-12-14T15:16:37.9131929Z    |
2019-12-14T15:16:37.9131929Z    |
2019-12-14T15:16:37.9132010Z LL |     b: isize,
2019-12-14T15:16:37.9132129Z 
2019-12-14T15:16:37.9132213Z error: missing documentation for a struct
2019-12-14T15:16:37.9132493Z   --> $DIR/missing-doc.rs:18:1
2019-12-14T15:16:37.9132572Z    |
2019-12-14T15:16:37.9132572Z    |
2019-12-14T15:16:37.9132655Z LL | / pub struct PubFoo {
2019-12-14T15:16:37.9132741Z LL | |     pub a: isize,
2019-12-14T15:16:37.9132829Z LL | |     b: isize,
2019-12-14T15:16:37.9132984Z    | |_^
2019-12-14T15:16:37.9133026Z 
2019-12-14T15:16:37.9133170Z error: missing documentation for a struct field
2019-12-14T15:16:37.9133445Z   --> $DIR/missing-doc.rs:19:5
2019-12-14T15:16:37.9133445Z   --> $DIR/missing-doc.rs:19:5
2019-12-14T15:16:37.9133602Z    |
2019-12-14T15:16:37.9133671Z LL |     pub a: isize,
2019-12-14T15:16:37.9133808Z 
2019-12-14T15:16:37.9133928Z error: missing documentation for a struct field
2019-12-14T15:16:37.9134195Z   --> $DIR/missing-doc.rs:20:5
2019-12-14T15:16:37.9134287Z    |
2019-12-14T15:16:37.9134287Z    |
2019-12-14T15:16:37.9134354Z LL |     b: isize,
2019-12-14T15:16:37.9134489Z 
2019-12-14T15:16:37.9134577Z error: missing documentation for a module
2019-12-14T15:16:37.9134841Z   --> $DIR/missing-doc.rs:29:1
2019-12-14T15:16:37.9134931Z    |
2019-12-14T15:16:37.9134931Z    |
2019-12-14T15:16:37.9135001Z LL | mod module_no_dox {}
2019-12-14T15:16:37.9135150Z 
2019-12-14T15:16:37.9135234Z error: missing documentation for a module
2019-12-14T15:16:37.9135501Z   --> $DIR/missing-doc.rs:30:1
2019-12-14T15:16:37.9135592Z    |
2019-12-14T15:16:37.9135592Z    |
2019-12-14T15:16:37.9135661Z LL | pub mod pub_module_no_dox {}
2019-12-14T15:16:37.9135803Z 
2019-12-14T15:16:37.9135890Z error: missing documentation for a function
2019-12-14T15:16:37.9136155Z   --> $DIR/missing-doc.rs:34:1
2019-12-14T15:16:37.9136257Z    |
---
2019-12-14T15:16:37.9137108Z 
2019-12-14T15:16:37.9137201Z error: missing documentation for a trait
2019-12-14T15:16:37.9137470Z   --> $DIR/missing-doc.rs:53:1
2019-12-14T15:16:37.9137564Z    |
2019-12-14T15:16:37.9137632Z LL | / pub trait C {
2019-12-14T15:16:37.9137719Z LL | |     fn foo(&self);
2019-12-14T15:16:37.9137809Z LL | |     fn foo_with_impl(&self) {}
2019-12-14T15:16:37.9137969Z    | |_^
2019-12-14T15:16:37.9138010Z 
2019-12-14T15:16:37.9138098Z error: missing documentation for a trait method
2019-12-14T15:16:37.9138369Z   --> $DIR/missing-doc.rs:54:5
---
2019-12-14T15:16:37.9143865Z 
2019-12-14T15:16:37.9143951Z error: missing documentation for an enum
2019-12-14T15:16:37.9144216Z   --> $DIR/missing-doc.rs:112:1
2019-12-14T15:16:37.9144319Z    |
2019-12-14T15:16:37.9144399Z LL | / enum Baz {
2019-12-14T15:16:37.9144474Z LL | |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.9144567Z LL | |     BarB,
2019-12-14T15:16:37.9144721Z    | |_^
2019-12-14T15:16:37.9144762Z 
2019-12-14T15:16:37.9144846Z error: missing documentation for a variant
2019-12-14T15:16:37.9145305Z   --> $DIR/missing-doc.rs:113:5
2019-12-14T15:16:37.9145305Z   --> $DIR/missing-doc.rs:113:5
2019-12-14T15:16:37.9145408Z    |
2019-12-14T15:16:37.9145476Z LL |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.9145630Z 
2019-12-14T15:16:37.9145718Z error: missing documentation for a struct field
2019-12-14T15:16:37.9145995Z   --> $DIR/missing-doc.rs:113:12
2019-12-14T15:16:37.9146086Z    |
2019-12-14T15:16:37.9146086Z    |
2019-12-14T15:16:37.9146154Z LL |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.9146294Z 
2019-12-14T15:16:37.9146380Z error: missing documentation for a struct field
2019-12-14T15:16:37.9146661Z   --> $DIR/missing-doc.rs:113:22
2019-12-14T15:16:37.9146754Z    |
2019-12-14T15:16:37.9146754Z    |
2019-12-14T15:16:37.9146824Z LL |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.9146970Z 
2019-12-14T15:16:37.9147052Z error: missing documentation for a variant
2019-12-14T15:16:37.9147322Z   --> $DIR/missing-doc.rs:114:5
2019-12-14T15:16:37.9147415Z    |
2019-12-14T15:16:37.9147415Z    |
2019-12-14T15:16:37.9147484Z LL |     BarB,
2019-12-14T15:16:37.9147567Z    |     ^^^^
2019-12-14T15:16:37.9147622Z 
2019-12-14T15:16:37.9147705Z error: missing documentation for an enum
2019-12-14T15:16:37.9147976Z   --> $DIR/missing-doc.rs:117:1
2019-12-14T15:16:37.9148069Z    |
2019-12-14T15:16:37.9148137Z LL | / pub enum PubBaz {
2019-12-14T15:16:37.9148223Z LL | |     PubBazA { a: isize },
2019-12-14T15:16:37.9148381Z    | |_^
2019-12-14T15:16:37.9148423Z 
2019-12-14T15:16:37.9148507Z error: missing documentation for a variant
2019-12-14T15:16:37.9148883Z   --> $DIR/missing-doc.rs:118:5
2019-12-14T15:16:37.9148883Z   --> $DIR/missing-doc.rs:118:5
2019-12-14T15:16:37.9148990Z    |
2019-12-14T15:16:37.9149058Z LL |     PubBazA { a: isize },
2019-12-14T15:16:37.9149197Z 
2019-12-14T15:16:37.9149294Z error: missing documentation for a struct field
2019-12-14T15:16:37.9149636Z   --> $DIR/missing-doc.rs:118:15
2019-12-14T15:16:37.9149745Z    |
2019-12-14T15:16:37.9149745Z    |
2019-12-14T15:16:37.9149866Z LL |     PubBazA { a: isize },
2019-12-14T15:16:37.9150094Z 
2019-12-14T15:16:37.9150181Z error: missing documentation for a constant
2019-12-14T15:16:37.9150485Z   --> $DIR/missing-doc.rs:138:1
2019-12-14T15:16:37.9150564Z    |
---
2019-12-14T15:16:37.9158934Z  
2019-12-14T15:16:37.9159008Z  error: missing documentation for a type alias
2019-12-14T15:16:37.9159386Z    --> $DIR/missing-doc.rs:11:1
2019-12-14T15:16:37.9159467Z     |
2019-12-14T15:16:37.9159553Z  LL | pub type PubTypedef = String;
2019-12-14T15:16:37.9159723Z  
2019-12-14T15:16:37.9159794Z  error: missing documentation for a struct
2019-12-14T15:16:37.9160130Z    --> $DIR/missing-doc.rs:13:1
2019-12-14T15:16:37.9160226Z     |
2019-12-14T15:16:37.9160226Z     |
2019-12-14T15:16:37.9160295Z  LL | / struct Foo {
2019-12-14T15:16:37.9160386Z  LL | |     a: isize,
2019-12-14T15:16:37.9160461Z  LL | |     b: isize,
2019-12-14T15:16:37.9160619Z     | |_^
2019-12-14T15:16:37.9160702Z  
2019-12-14T15:16:37.9160775Z  error: missing documentation for a struct field
2019-12-14T15:16:37.9161062Z    --> $DIR/missing-doc.rs:14:5
2019-12-14T15:16:37.9161062Z    --> $DIR/missing-doc.rs:14:5
2019-12-14T15:16:37.9161141Z     |
2019-12-14T15:16:37.9161234Z  LL |     a: isize,
2019-12-14T15:16:37.9161396Z  
2019-12-14T15:16:37.9161547Z  error: missing documentation for a struct field
2019-12-14T15:16:37.9161816Z    --> $DIR/missing-doc.rs:15:5
2019-12-14T15:16:37.9161909Z     |
2019-12-14T15:16:37.9161909Z     |
2019-12-14T15:16:37.9161977Z  LL |     b: isize,
2019-12-14T15:16:37.9162138Z  
2019-12-14T15:16:37.9162223Z  error: missing documentation for a struct
2019-12-14T15:16:37.9162497Z    --> $DIR/missing-doc.rs:18:1
2019-12-14T15:16:37.9162593Z     |
2019-12-14T15:16:37.9162593Z     |
2019-12-14T15:16:37.9162678Z  LL | / pub struct PubFoo {
2019-12-14T15:16:37.9162756Z  LL | |     pub a: isize,
2019-12-14T15:16:37.9162850Z  LL | |     b: isize,
2019-12-14T15:16:37.9163007Z     | |_^
2019-12-14T15:16:37.9163073Z  
2019-12-14T15:16:37.9163163Z  error: missing documentation for a struct field
2019-12-14T15:16:37.9163437Z    --> $DIR/missing-doc.rs:19:5
2019-12-14T15:16:37.9163437Z    --> $DIR/missing-doc.rs:19:5
2019-12-14T15:16:37.9163542Z     |
2019-12-14T15:16:37.9163609Z  LL |     pub a: isize,
2019-12-14T15:16:37.9163789Z  
2019-12-14T15:16:37.9163863Z  error: missing documentation for a struct field
2019-12-14T15:16:37.9164147Z    --> $DIR/missing-doc.rs:20:5
2019-12-14T15:16:37.9164226Z     |
2019-12-14T15:16:37.9164226Z     |
2019-12-14T15:16:37.9164309Z  LL |     b: isize,
2019-12-14T15:16:37.9164469Z  
2019-12-14T15:16:37.9164549Z  error: missing documentation for a module
2019-12-14T15:16:37.9164833Z    --> $DIR/missing-doc.rs:29:1
2019-12-14T15:16:37.9164911Z     |
2019-12-14T15:16:37.9164911Z     |
2019-12-14T15:16:37.9164995Z  LL | mod module_no_dox {}
2019-12-14T15:16:37.9165160Z  
2019-12-14T15:16:37.9165247Z  error: missing documentation for a module
2019-12-14T15:16:37.9165512Z    --> $DIR/missing-doc.rs:30:1
2019-12-14T15:16:37.9165607Z     |
2019-12-14T15:16:37.9165607Z     |
2019-12-14T15:16:37.9165675Z  LL | pub mod pub_module_no_dox {}
2019-12-14T15:16:37.9165858Z  
2019-12-14T15:16:37.9165946Z  error: missing documentation for a function
2019-12-14T15:16:37.9166215Z +  --> $DIR/missing-doc.rs:33:1
2019-12-14T15:16:37.9166309Z +   |
---
2019-12-14T15:16:37.9170701Z +
2019-12-14T15:16:37.9170771Z +error: missing documentation for a trait
2019-12-14T15:16:37.9171056Z    --> $DIR/missing-doc.rs:53:1
2019-12-14T15:16:37.9171135Z     |
2019-12-14T15:16:37.9171219Z  LL | / pub trait C {
2019-12-14T15:16:37.9171294Z  LL | |     fn foo(&self);
2019-12-14T15:16:37.9171387Z  LL | |     fn foo_with_impl(&self) {}
2019-12-14T15:16:37.9171557Z     | |_^
2019-12-14T15:16:37.9171638Z  
2019-12-14T15:16:37.9171713Z  error: missing documentation for a trait method
2019-12-14T15:16:37.9172000Z    --> $DIR/missing-doc.rs:54:5
---
2019-12-14T15:16:37.9173029Z  
2019-12-14T15:16:37.9173164Z +error: missing documentation for a trait
2019-12-14T15:16:37.9173430Z +  --> $DIR/missing-doc.rs:64:1
2019-12-14T15:16:37.9173527Z +   |
2019-12-14T15:16:37.9173595Z +LL | / pub trait E {
2019-12-14T15:16:37.9173757Z +LL | |     type AssociatedType;
2019-12-14T15:16:37.9173846Z +LL | |     type AssociatedTypeDef = Self;
2019-12-14T15:16:37.9174024Z +...  |
2019-12-14T15:16:37.9174024Z +...  |
2019-12-14T15:16:37.9174092Z +LL | |     fn dummy(&self) {}
2019-12-14T15:16:37.9174248Z +   | |_^
2019-12-14T15:16:37.9174329Z +
2019-12-14T15:16:37.9174405Z  error: missing documentation for an associated type
2019-12-14T15:16:37.9174698Z    --> $DIR/missing-doc.rs:65:5
---
2019-12-14T15:16:37.9177393Z +
2019-12-14T15:16:37.9177467Z +error: missing documentation for a trait method
2019-12-14T15:16:37.9177773Z +  --> $DIR/missing-doc.rs:73:5
2019-12-14T15:16:37.9177869Z +   |
2019-12-14T15:16:37.9177938Z +LL |     fn dummy(&self) {}
2019-12-14T15:16:37.9178105Z +
2019-12-14T15:16:37.9178190Z  error: missing documentation for a method
2019-12-14T15:16:37.9178467Z    --> $DIR/missing-doc.rs:77:5
2019-12-14T15:16:37.9178563Z     |
---
2019-12-14T15:16:37.9181721Z  
2019-12-14T15:16:37.9181791Z  error: missing documentation for an enum
2019-12-14T15:16:37.9182086Z    --> $DIR/missing-doc.rs:112:1
2019-12-14T15:16:37.9182165Z     |
2019-12-14T15:16:37.9182248Z  LL | / enum Baz {
2019-12-14T15:16:37.9182325Z  LL | |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.9182420Z  LL | |     BarB,
2019-12-14T15:16:37.9182577Z     | |_^
2019-12-14T15:16:37.9182642Z  
2019-12-14T15:16:37.9182731Z  error: missing documentation for a variant
2019-12-14T15:16:37.9182998Z    --> $DIR/missing-doc.rs:113:5
2019-12-14T15:16:37.9182998Z    --> $DIR/missing-doc.rs:113:5
2019-12-14T15:16:37.9183102Z     |
2019-12-14T15:16:37.9183187Z  LL |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.9183359Z  
2019-12-14T15:16:37.9183434Z  error: missing documentation for a struct field
2019-12-14T15:16:37.9183721Z    --> $DIR/missing-doc.rs:113:12
2019-12-14T15:16:37.9183800Z     |
2019-12-14T15:16:37.9183800Z     |
2019-12-14T15:16:37.9183885Z  LL |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.9184052Z  
2019-12-14T15:16:37.9184151Z  error: missing documentation for a struct field
2019-12-14T15:16:37.9184423Z    --> $DIR/missing-doc.rs:113:22
2019-12-14T15:16:37.9184518Z     |
2019-12-14T15:16:37.9184518Z     |
2019-12-14T15:16:37.9184586Z  LL |     BazA { a: isize, b: isize },
2019-12-14T15:16:37.9184758Z  
2019-12-14T15:16:37.9184846Z  error: missing documentation for a variant
2019-12-14T15:16:37.9225626Z    --> $DIR/missing-doc.rs:114:5
2019-12-14T15:16:37.9225769Z     |
2019-12-14T15:16:37.9225769Z     |
2019-12-14T15:16:37.9226026Z  LL |     BarB,
2019-12-14T15:16:37.9226134Z     |     ^^^^
2019-12-14T15:16:37.9226223Z  
2019-12-14T15:16:37.9226311Z  error: missing documentation for an enum
2019-12-14T15:16:37.9226692Z    --> $DIR/missing-doc.rs:117:1
2019-12-14T15:16:37.9226774Z     |
2019-12-14T15:16:37.9226861Z  LL | / pub enum PubBaz {
2019-12-14T15:16:37.9226940Z  LL | |     PubBazA { a: isize },
2019-12-14T15:16:37.9227102Z     | |_^
2019-12-14T15:16:37.9227186Z  
2019-12-14T15:16:37.9227366Z  error: missing documentation for a variant
2019-12-14T15:16:37.9227686Z    --> $DIR/missing-doc.rs:118:5
2019-12-14T15:16:37.9227686Z    --> $DIR/missing-doc.rs:118:5
2019-12-14T15:16:37.9227853Z     |
2019-12-14T15:16:37.9227923Z  LL |     PubBazA { a: isize },
2019-12-14T15:16:37.9228094Z  
2019-12-14T15:16:37.9228186Z  error: missing documentation for a struct field
2019-12-14T15:16:37.9228458Z    --> $DIR/missing-doc.rs:118:15
2019-12-14T15:16:37.9228556Z     |
2019-12-14T15:16:37.9228556Z     |
2019-12-14T15:16:37.9228635Z  LL |     PubBazA { a: isize },
2019-12-14T15:16:37.9228805Z  
2019-12-14T15:16:37.9228889Z +error: missing documentation for an enum
2019-12-14T15:16:37.9229173Z +  --> $DIR/missing-doc.rs:122:1
2019-12-14T15:16:37.9229254Z +   |
2019-12-14T15:16:37.9229254Z +   |
2019-12-14T15:16:37.9229337Z +LL | / pub enum PubBaz2 {
2019-12-14T15:16:37.9229415Z +LL | |     /// dox
2019-12-14T15:16:37.9229505Z +LL | |     PubBaz2A {
2019-12-14T15:16:37.9229580Z +LL | |         /// dox
2019-12-14T15:16:37.9229682Z +LL | |         a: isize,
2019-12-14T15:16:37.9229893Z +LL | | }
2019-12-14T15:16:37.9229962Z +   | |_^
2019-12-14T15:16:37.9230044Z +
2019-12-14T15:16:37.9230132Z +error: missing documentation for a variant
2019-12-14T15:16:37.9230132Z +error: missing documentation for a variant
2019-12-14T15:16:37.9230407Z +  --> $DIR/missing-doc.rs:124:5
2019-12-14T15:16:37.9230503Z +   |
2019-12-14T15:16:37.9230571Z +LL | /     PubBaz2A {
2019-12-14T15:16:37.9230663Z +LL | |         /// dox
2019-12-14T15:16:37.9230747Z +LL | |         a: isize,
2019-12-14T15:16:37.9230909Z +   | |_____^
2019-12-14T15:16:37.9230994Z +
2019-12-14T15:16:37.9231068Z +error: missing documentation for a struct field
2019-12-14T15:16:37.9231357Z +  --> $DIR/missing-doc.rs:126:9
2019-12-14T15:16:37.9231357Z +  --> $DIR/missing-doc.rs:126:9
2019-12-14T15:16:37.9231453Z +   |
2019-12-14T15:16:37.9231520Z +LL |         a: isize,
2019-12-14T15:16:37.9231683Z +
2019-12-14T15:16:37.9231771Z  error: missing documentation for a constant
2019-12-14T15:16:37.9232053Z    --> $DIR/missing-doc.rs:138:1
2019-12-14T15:16:37.9232149Z     |
---
2019-12-14T15:16:37.9240253Z  
2019-12-14T15:16:37.9240337Z +error: missing documentation for a module
2019-12-14T15:16:37.9240623Z +  --> $DIR/missing-doc.rs:163:5
2019-12-14T15:16:37.9240703Z +   |
2019-12-14T15:16:37.9240786Z +LL | /     pub mod globbed {
2019-12-14T15:16:37.9240863Z +LL | |         /// dox
2019-12-14T15:16:37.9240959Z +LL | |         pub fn also_documented() {}
2019-12-14T15:16:37.9241055Z +LL | |         pub fn also_undocumented1() {}
2019-12-14T15:16:37.9241154Z +LL | |         fn also_undocumented2() {}
2019-12-14T15:16:37.9241422Z +   | |_____^
2019-12-14T15:16:37.9241505Z +
2019-12-14T15:16:37.9241577Z  error: missing documentation for a function
2019-12-14T15:16:37.9241941Z +  --> $DIR/missing-doc.rs:165:9
---
2019-12-14T15:16:37.9244029Z -error: aborting due to 39 previous errors
2019-12-14T15:16:37.9244133Z +error: missing documentation for a module
2019-12-14T15:16:37.9244396Z +  --> $DIR/missing-doc.rs:171:1
2019-12-14T15:16:37.9244586Z +   |
2019-12-14T15:16:37.9244664Z +LL | / pub mod public_interface {
2019-12-14T15:16:37.9244768Z +LL | |     pub use internal_impl::documented as foo;
2019-12-14T15:16:37.9244859Z +LL | |     pub use internal_impl::globbed::*;
2019-12-14T15:16:37.9244966Z +LL | |     pub use internal_impl::undocumented1 as bar;
2019-12-14T15:16:37.9245063Z +LL | |     pub use internal_impl::{documented, undocumented2};
2019-12-14T15:16:37.9245255Z +   | |_^
2019-12-14T15:16:37.9245387Z +
2019-12-14T15:16:37.9245476Z +error: aborting due to 57 previous errors
2019-12-14T15:16:37.9245553Z  
---
2019-12-14T15:16:37.9251483Z 
2019-12-14T15:16:37.9251749Z ------------------------------------------
2019-12-14T15:16:37.9251848Z stderr:
2019-12-14T15:16:37.9252110Z ------------------------------------------
2019-12-14T15:16:37.9254476Z {"message":"missing documentation for a type alias","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":307,"byte_end":329,"line_start":10,"line_end":10,"column_start":1,"column_end":23,"is_primary":true,"text":[{"text":"type Typedef = String;","highlight_start":1,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::missing-docs-in-private-items` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: missing documentation for a type alias\n  --> tests/ui/missing-doc.rs:10:1\n   |\nLL | type Typedef = String;\n   | ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::missing-docs-in-private-items` implied by `-D warnings`\n\n"}
2019-12-14T15:16:37.9256583Z {"message":"missing documentation for a type alias","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":330,"byte_end":359,"line_start":11,"line_end":11,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"pub type PubTypedef = String;","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a type alias\n  --> tests/ui/missing-doc.rs:11:1\n   |\nLL | pub type PubTypedef = String;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9259041Z {"message":"missing documentation for a struct","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":361,"byte_end":403,"line_start":13,"line_end":16,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct Foo {","highlight_start":1,"highlight_end":13},{"text":"    a: isize,","highlight_start":1,"highlight_end":14},{"text":"    b: isize,","highlight_start":1,"highlight_end":14},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct\n  --> tests/ui/missing-doc.rs:13:1\n   |\nLL | / struct Foo {\nLL | |     a: isize,\nLL | |     b: isize,\nLL | | }\n   | |_^\n\n"}
2019-12-14T15:16:37.9260585Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":378,"byte_end":386,"line_start":14,"line_end":14,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    a: isize,","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:14:5\n   |\nLL |     a: isize,\n   |     ^^^^^^^^\n\n"}
2019-12-14T15:16:37.9261960Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":392,"byte_end":400,"line_start":15,"line_end":15,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    b: isize,","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:15:5\n   |\nLL |     b: isize,\n   |     ^^^^^^^^\n\n"}
2019-12-14T15:16:37.9263681Z {"message":"missing documentation for a struct","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":405,"byte_end":458,"line_start":18,"line_end":21,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub struct PubFoo {","highlight_start":1,"highlight_end":20},{"text":"    pub a: isize,","highlight_start":1,"highlight_end":18},{"text":"    b: isize,","highlight_start":1,"highlight_end":14},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct\n  --> tests/ui/missing-doc.rs:18:1\n   |\nLL | / pub struct PubFoo {\nLL | |     pub a: isize,\nLL | |     b: isize,\nLL | | }\n   | |_^\n\n"}
2019-12-14T15:16:37.9265680Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":429,"byte_end":441,"line_start":19,"line_end":19,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    pub a: isize,","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:19:5\n   |\nLL |     pub a: isize,\n   |     ^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9267300Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":447,"byte_end":455,"line_start":20,"line_end":20,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    b: isize,","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:20:5\n   |\nLL |     b: isize,\n   |     ^^^^^^^^\n\n"}
2019-12-14T15:16:37.9268733Z {"message":"missing documentation for a module","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":568,"byte_end":588,"line_start":29,"line_end":29,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"mod module_no_dox {}","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a module\n  --> tests/ui/missing-doc.rs:29:1\n   |\nLL | mod module_no_dox {}\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9270212Z {"message":"missing documentation for a module","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":589,"byte_end":617,"line_start":30,"line_end":30,"column_start":1,"column_end":29,"is_primary":true,"text":[{"text":"pub mod pub_module_no_dox {}","highlight_start":1,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a module\n  --> tests/ui/missing-doc.rs:30:1\n   |\nLL | pub mod pub_module_no_dox {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9271674Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":627,"byte_end":642,"line_start":33,"line_end":33,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"pub fn foo() {}","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:33:1\n   |\nLL | pub fn foo() {}\n   | ^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9273115Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":643,"byte_end":659,"line_start":34,"line_end":34,"column_start":1,"column_end":17,"is_primary":true,"text":[{"text":"pub fn foo2() {}","highlight_start":1,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:34:1\n   |\nLL | pub fn foo2() {}\n   | ^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9274615Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":660,"byte_end":672,"line_start":35,"line_end":35,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"fn foo3() {}","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:35:1\n   |\nLL | fn foo3() {}\n   | ^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9276816Z {"message":"missing documentation for a trait","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":747,"byte_end":836,"line_start":40,"line_end":45,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub trait A {","highlight_start":1,"highlight_end":14},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    fn foo(&self);","highlight_start":1,"highlight_end":19},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    fn foo_with_impl(&self) {}","highlight_start":1,"highlight_end":31},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait\n  --> tests/ui/missing-doc.rs:40:1\n   |\nLL | / pub trait A {\nLL | |     /// dox\nLL | |     fn foo(&self);\nLL | |     /// dox\nLL | |     fn foo_with_impl(&self) {}\nLL | | }\n   | |_^\n\n"}
2019-12-14T15:16:37.9278479Z {"message":"missing documentation for a trait method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":777,"byte_end":791,"line_start":42,"line_end":42,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    fn foo(&self);","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait method\n  --> tests/ui/missing-doc.rs:42:5\n   |\nLL |     fn foo(&self);\n   |     ^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9279998Z {"message":"missing documentation for a trait method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":808,"byte_end":834,"line_start":44,"line_end":44,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    fn foo_with_impl(&self) {}","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait method\n  --> tests/ui/missing-doc.rs:44:5\n   |\nLL |     fn foo_with_impl(&self) {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9283213Z {"message":"missing documentation for a trait","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":949,"byte_end":1014,"line_start":53,"line_end":56,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub trait C {","highlight_start":1,"highlight_end":14},{"text":"    fn foo(&self);","highlight_start":1,"highlight_end":19},{"text":"    fn foo_with_impl(&self) {}","highlight_start":1,"highlight_end":31},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait\n  --> tests/ui/missing-doc.rs:53:1\n   |\nLL | / pub trait C {\nLL | |     fn foo(&self);\nLL | |     fn foo_with_impl(&self) {}\nLL | | }\n   | |_^\n\n"}
2019-12-14T15:16:37.9285005Z {"message":"missing documentation for a trait method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":967,"byte_end":981,"line_start":54,"line_end":54,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    fn foo(&self);","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait method\n  --> tests/ui/missing-doc.rs:54:5\n   |\nLL |     fn foo(&self);\n   |     ^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9286595Z {"message":"missing documentation for a trait method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":986,"byte_end":1012,"line_start":55,"line_end":55,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    fn foo_with_impl(&self) {}","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait method\n  --> tests/ui/missing-doc.rs:55:5\n   |\nLL |     fn foo_with_impl(&self) {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9289696Z {"message":"missing documentation for a trait","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1112,"byte_end":1307,"line_start":64,"line_end":74,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub trait E {","highlight_start":1,"highlight_end":14},{"text":"    type AssociatedType;","highlight_start":1,"highlight_end":25},{"text":"    type AssociatedTypeDef = Self;","highlight_start":1,"highlight_end":35},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    type DocumentedType;","highlight_start":1,"highlight_end":25},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    type DocumentedTypeDef = Self;","highlight_start":1,"highlight_end":35},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    fn dummy(&self) {}","highlight_start":1,"highlight_end":23},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait\n  --> tests/ui/missing-doc.rs:64:1\n   |\nLL | / pub trait E {\nLL | |     type AssociatedType;\nLL | |     type AssociatedTypeDef = Self;\nLL | |\n...  |\nLL | |     fn dummy(&self) {}\nLL | | }\n   | |_^\n\n"}
2019-12-14T15:16:37.9291681Z {"message":"missing documentation for an associated type","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1130,"byte_end":1150,"line_start":65,"line_end":65,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    type AssociatedType;","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an associated type\n  --> tests/ui/missing-doc.rs:65:5\n   |\nLL |     type AssociatedType;\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9293351Z {"message":"missing documentation for an associated type","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1155,"byte_end":1185,"line_start":66,"line_end":66,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    type AssociatedTypeDef = Self;","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an associated type\n  --> tests/ui/missing-doc.rs:66:5\n   |\nLL |     type AssociatedTypeDef = Self;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9294925Z {"message":"missing documentation for an associated type","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1203,"byte_end":1223,"line_start":69,"line_end":69,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    type DocumentedType;","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an associated type\n  --> tests/ui/missing-doc.rs:69:5\n   |\nLL |     type DocumentedType;\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9296607Z {"message":"missing documentation for an associated type","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1240,"byte_end":1270,"line_start":71,"line_end":71,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    type DocumentedTypeDef = Self;","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an associated type\n  --> tests/ui/missing-doc.rs:71:5\n   |\nLL |     type DocumentedTypeDef = Self;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9298124Z {"message":"missing documentation for a trait method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1287,"byte_end":1305,"line_start":73,"line_end":73,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    fn dummy(&self) {}","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait method\n  --> tests/ui/missing-doc.rs:73:5\n   |\nLL |     fn dummy(&self) {}\n   |     ^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9299582Z {"message":"missing documentation for a method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1324,"byte_end":1339,"line_start":77,"line_end":77,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    pub fn foo() {}","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a method\n  --> tests/ui/missing-doc.rs:77:5\n   |\nLL |     pub fn foo() {}\n   |     ^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9301073Z {"message":"missing documentation for a method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1344,"byte_end":1355,"line_start":78,"line_end":78,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    fn bar() {}","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a method\n  --> tests/ui/missing-doc.rs:78:5\n   |\nLL |     fn bar() {}\n   |     ^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9302592Z {"message":"missing documentation for a method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1377,"byte_end":1392,"line_start":82,"line_end":82,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    pub fn foo() {}","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a method\n  --> tests/ui/missing-doc.rs:82:5\n   |\nLL |     pub fn foo() {}\n   |     ^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9304074Z {"message":"missing documentation for a method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1409,"byte_end":1425,"line_start":84,"line_end":84,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    pub fn foo1() {}","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a method\n  --> tests/ui/missing-doc.rs:84:5\n   |\nLL |     pub fn foo1() {}\n   |     ^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9305788Z {"message":"missing documentation for a method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1430,"byte_end":1442,"line_start":85,"line_end":85,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    fn foo2() {}","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a method\n  --> tests/ui/missing-doc.rs:85:5\n   |\nLL |     fn foo2() {}\n   |     ^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9307620Z {"message":"missing documentation for an enum","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1923,"byte_end":1978,"line_start":112,"line_end":115,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"enum Baz {","highlight_start":1,"highlight_end":11},{"text":"    BazA { a: isize, b: isize },","highlight_start":1,"highlight_end":33},{"text":"    BarB,","highlight_start":1,"highlight_end":10},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an enum\n  --> tests/ui/missing-doc.rs:112:1\n   |\nLL | / enum Baz {\nLL | |     BazA { a: isize, b: isize },\nLL | |     BarB,\nLL | | }\n   | |_^\n\n"}
2019-12-14T15:16:37.9309214Z {"message":"missing documentation for a variant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1938,"byte_end":1965,"line_start":113,"line_end":113,"column_start":5,"column_end":32,"is_primary":true,"text":[{"text":"    BazA { a: isize, b: isize },","highlight_start":5,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a variant\n  --> tests/ui/missing-doc.rs:113:5\n   |\nLL |     BazA { a: isize, b: isize },\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9310748Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1945,"byte_end":1953,"line_start":113,"line_end":113,"column_start":12,"column_end":20,"is_primary":true,"text":[{"text":"    BazA { a: isize, b: isize },","highlight_start":12,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:113:12\n   |\nLL |     BazA { a: isize, b: isize },\n   |            ^^^^^^^^\n\n"}
2019-12-14T15:16:37.9312407Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1955,"byte_end":1963,"line_start":113,"line_end":113,"column_start":22,"column_end":30,"is_primary":true,"text":[{"text":"    BazA { a: isize, b: isize },","highlight_start":22,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:113:22\n   |\nLL |     BazA { a: isize, b: isize },\n   |                      ^^^^^^^^\n\n"}
2019-12-14T15:16:37.9313883Z {"message":"missing documentation for a variant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1971,"byte_end":1975,"line_start":114,"line_end":114,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    BarB,","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a variant\n  --> tests/ui/missing-doc.rs:114:5\n   |\nLL |     BarB,\n   |     ^^^^\n\n"}
2019-12-14T15:16:37.9315717Z {"message":"missing documentation for an enum","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1980,"byte_end":2025,"line_start":117,"line_end":119,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub enum PubBaz {","highlight_start":1,"highlight_end":18},{"text":"    PubBazA { a: isize },","highlight_start":1,"highlight_end":26},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an enum\n  --> tests/ui/missing-doc.rs:117:1\n   |\nLL | / pub enum PubBaz {\nLL | |     PubBazA { a: isize },\nLL | | }\n   | |_^\n\n"}
2019-12-14T15:16:37.9317262Z {"message":"missing documentation for a variant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2002,"byte_end":2022,"line_start":118,"line_end":118,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    PubBazA { a: isize },","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a variant\n  --> tests/ui/missing-doc.rs:118:5\n   |\nLL |     PubBazA { a: isize },\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9318766Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2012,"byte_end":2020,"line_start":118,"line_end":118,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"    PubBazA { a: isize },","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:118:15\n   |\nLL |     PubBazA { a: isize },\n   |               ^^^^^^^^\n\n"}
2019-12-14T15:16:37.9321056Z {"message":"missing documentation for an enum","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2035,"byte_end":2123,"line_start":122,"line_end":128,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub enum PubBaz2 {","highlight_start":1,"highlight_end":19},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    PubBaz2A {","highlight_start":1,"highlight_end":15},{"text":"        /// dox","highlight_start":1,"highlight_end":16},{"text":"        a: isize,","highlight_start":1,"highlight_end":18},{"text":"    },","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an enum\n  --> tests/ui/missing-doc.rs:122:1\n   |\nLL | / pub enum PubBaz2 {\nLL | |     /// dox\nLL | |     PubBaz2A {\nLL | |         /// dox\nLL | |         a: isize,\nLL | |     },\nLL | | }\n   | |_^\n\n"}
2019-12-14T15:16:37.9323119Z {"message":"missing documentation for a variant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2070,"byte_end":2120,"line_start":124,"line_end":127,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    PubBaz2A {","highlight_start":5,"highlight_end":15},{"text":"        /// dox","highlight_start":1,"highlight_end":16},{"text":"        a: isize,","highlight_start":1,"highlight_end":18},{"text":"    },","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a variant\n  --> tests/ui/missing-doc.rs:124:5\n   |\nLL | /     PubBaz2A {\nLL | |         /// dox\nLL | |         a: isize,\nLL | |     },\n   | |_____^\n\n"}
2019-12-14T15:16:37.9324805Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2105,"byte_end":2113,"line_start":126,"line_end":126,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        a: isize,","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:126:9\n   |\nLL |         a: isize,\n   |         ^^^^^^^^\n\n"}
2019-12-14T15:16:37.9326265Z {"message":"missing documentation for a constant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2254,"byte_end":2273,"line_start":138,"line_end":138,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"const FOO: u32 = 0;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a constant\n  --> tests/ui/missing-doc.rs:138:1\n   |\nLL | const FOO: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9327746Z {"message":"missing documentation for a constant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2282,"byte_end":2306,"line_start":140,"line_end":140,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"pub const FOO1: u32 = 0;","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a constant\n  --> tests/ui/missing-doc.rs:140:1\n   |\nLL | pub const FOO1: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9329227Z {"message":"missing documentation for a constant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2420,"byte_end":2444,"line_start":145,"line_end":145,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"pub const FOO4: u32 = 0;","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a constant\n  --> tests/ui/missing-doc.rs:145:1\n   |\nLL | pub const FOO4: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9330788Z {"message":"missing documentation for a static","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2446,"byte_end":2466,"line_start":147,"line_end":147,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"static BAR: u32 = 0;","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a static\n  --> tests/ui/missing-doc.rs:147:1\n   |\nLL | static BAR: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9332349Z {"message":"missing documentation for a static","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2475,"byte_end":2500,"line_start":149,"line_end":149,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"pub static BAR1: u32 = 0;","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a static\n  --> tests/ui/missing-doc.rs:149:1\n   |\nLL | pub static BAR1: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9333895Z {"message":"missing documentation for a static","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2616,"byte_end":2641,"line_start":154,"line_end":154,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"pub static BAR4: u32 = 0;","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a static\n  --> tests/ui/missing-doc.rs:154:1\n   |\nLL | pub static BAR4: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9337024Z {"message":"missing documentation for a module","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2643,"byte_end":2955,"line_start":156,"line_end":169,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"mod internal_impl {","highlight_start":1,"highlight_end":20},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    pub fn documented() {}","highlight_start":1,"highlight_end":27},{"text":"    pub fn undocumented1() {}","highlight_start":1,"highlight_end":30},{"text":"    pub fn undocumented2() {}","highlight_start":1,"highlight_end":30},{"text":"    fn undocumented3() {}","highlight_start":1,"highlight_end":26},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    pub mod globbed {","highlight_start":1,"highlight_end":22},{"text":"        /// dox","highlight_start":1,"highlight_end":16},{"text":"        pub fn also_documented() {}","highlight_start":1,"highlight_end":36},{"text":"        pub fn also_undocumented1() {}","highlight_start":1,"highlight_end":39},{"text":"        fn also_undocumented2() {}","highlight_start":1,"highlight_end":35},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a module\n  --> tests/ui/missing-doc.rs:156:1\n   |\nLL | / mod internal_impl {\nLL | |     /// dox\nLL | |     pub fn documented() {}\nLL | |     pub fn undocumented1() {}\n...  |\nLL | |     }\nLL | | }\n   | |_^\n\n"}
2019-12-14T15:16:37.9339280Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2679,"byte_end":2701,"line_start":158,"line_end":158,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    pub fn documented() {}","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:158:5\n   |\nLL |     pub fn documented() {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9340842Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2706,"byte_end":2731,"line_start":159,"line_end":159,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    pub fn undocumented1() {}","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:159:5\n   |\nLL |     pub fn undocumented1() {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9342522Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2736,"byte_end":2761,"line_start":160,"line_end":160,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    pub fn undocumented2() {}","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:160:5\n   |\nLL |     pub fn undocumented2() {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9344059Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2766,"byte_end":2787,"line_start":161,"line_end":161,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    fn undocumented3() {}","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:161:5\n   |\nLL |     fn undocumented3() {}\n   |     ^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9386834Z {"message":"missing documentation for a module","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2804,"byte_end":2953,"line_start":163,"line_end":168,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub mod globbed {","highlight_start":5,"highlight_end":22},{"text":"        /// dox","highlight_start":1,"highlight_end":16},{"text":"        pub fn also_documented() {}","highlight_start":1,"highlight_end":36},{"text":"        pub fn also_undocumented1() {}","highlight_start":1,"highlight_end":39},{"text":"        fn also_undocumented2() {}","highlight_start":1,"highlight_end":35},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a module\n  --> tests/ui/missing-doc.rs:163:5\n   |\nLL | /     pub mod globbed {\nLL | |         /// dox\nLL | |         pub fn also_documented() {}\nLL | |         pub fn also_undocumented1() {}\nLL | |         fn also_undocumented2() {}\nLL | |     }\n   | |_____^\n\n"}
2019-12-14T15:16:37.9388885Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2846,"byte_end":2873,"line_start":165,"line_end":165,"column_start":9,"column_end":36,"is_primary":true,"text":[{"text":"        pub fn also_documented() {}","highlight_start":9,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:165:9\n   |\nLL |         pub fn also_documented() {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9390480Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2882,"byte_end":2912,"line_start":166,"line_end":166,"column_start":9,"column_end":39,"is_primary":true,"text":[{"text":"        pub fn also_undocumented1() {}","highlight_start":9,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:166:9\n   |\nLL |         pub fn also_undocumented1() {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9392128Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2921,"byte_end":2947,"line_start":167,"line_end":167,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"        fn also_undocumented2() {}","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:167:9\n   |\nLL |         fn also_undocumented2() {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T15:16:37.9394637Z {"message":"missing documentation for a module","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2964,"byte_end":3182,"line_start":171,"line_end":176,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub mod public_interface {","highlight_start":1,"highlight_end":27},{"text":"    pub use internal_impl::documented as foo;","highlight_start":1,"highlight_end":46},{"text":"    pub use internal_impl::globbed::*;","highlight_start":1,"highlight_end":39},{"text":"    pub use internal_impl::undocumented1 as bar;","highlight_start":1,"highlight_end":49},{"text":"    pub use internal_impl::{documented, undocumented2};","highlight_start":1,"highlight_end":56},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a module\n  --> tests/ui/missing-doc.rs:171:1\n   |\nLL | / pub mod public_interface {\nLL | |     pub use internal_impl::documented as foo;\nLL | |     pub use internal_impl::globbed::*;\nLL | |     pub use internal_impl::undocumented1 as bar;\nLL | |     pub use internal_impl::{documented, undocumented2};\nLL | | }\n   | |_^\n\n"}
2019-12-14T15:16:37.9395509Z 
2019-12-14T15:16:37.9395802Z ------------------------------------------
2019-12-14T15:16:37.9395877Z 
2019-12-14T15:16:37.9396135Z test [ui] ui/missing-doc.rs ... FAILED
---
2019-12-14T15:56:41.6636496Z Verifying status of clippy-driver...
2019-12-14T15:56:41.6636602Z Verifying status of miri...
2019-12-14T15:56:41.6636874Z Verifying status of embedded-book...
2019-12-14T15:56:41.6637163Z Verifying status of rustc-guide...
2019-12-14T15:56:41.6637504Z error: Tool `clippy-driver` should be test-pass but is test-fail during beta week.
2019-12-14T15:56:41.6642985Z Build completed unsuccessfully in 0:00:01
2019-12-14T15:56:41.6704763Z == clock drift check ==
2019-12-14T15:56:41.6722169Z   local time: Sat Dec 14 15:56:41 UTC 2019
2019-12-14T15:56:41.9499371Z   network time: Sat, 14 Dec 2019 15:56:41 GMT
2019-12-14T15:56:41.9499371Z   network time: Sat, 14 Dec 2019 15:56:41 GMT
2019-12-14T15:56:41.9502868Z == end clock drift check ==
2019-12-14T15:56:42.5041697Z 
2019-12-14T15:56:42.5143426Z ##[error]Bash exited with code '1'.
2019-12-14T15:56:42.5221310Z ##[section]Starting: Checkout
2019-12-14T15:56:42.5223451Z ==============================================================================
2019-12-14T15:56:42.5223575Z Task         : Get sources
2019-12-14T15:56:42.5223687Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
