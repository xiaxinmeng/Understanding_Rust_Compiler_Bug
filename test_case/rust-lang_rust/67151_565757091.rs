plain
2019-12-14T21:21:34.0638832Z 
2019-12-14T21:21:34.0638919Z error: missing documentation for a type alias
2019-12-14T21:21:34.0639169Z   --> $DIR/missing-doc.rs:11:1
2019-12-14T21:21:34.0639260Z    |
2019-12-14T21:21:34.0639341Z LL | pub type PubTypedef = String;
2019-12-14T21:21:34.0639460Z 
2019-12-14T21:21:34.0639542Z error: missing documentation for a struct
2019-12-14T21:21:34.0640127Z   --> $DIR/missing-doc.rs:13:1
2019-12-14T21:21:34.0640221Z    |
2019-12-14T21:21:34.0640221Z    |
2019-12-14T21:21:34.0640302Z LL | / struct Foo {
2019-12-14T21:21:34.0640371Z LL | |     a: isize,
2019-12-14T21:21:34.0640455Z LL | |     b: isize,
2019-12-14T21:21:34.0640602Z    | |_^
2019-12-14T21:21:34.0640651Z 
2019-12-14T21:21:34.0640737Z error: missing documentation for a struct field
2019-12-14T21:21:34.0641034Z   --> $DIR/missing-doc.rs:14:5
2019-12-14T21:21:34.0641034Z   --> $DIR/missing-doc.rs:14:5
2019-12-14T21:21:34.0641125Z    |
2019-12-14T21:21:34.0641187Z LL |     a: isize,
2019-12-14T21:21:34.0641314Z 
2019-12-14T21:21:34.0641400Z error: missing documentation for a struct field
2019-12-14T21:21:34.0641647Z   --> $DIR/missing-doc.rs:15:5
2019-12-14T21:21:34.0641736Z    |
2019-12-14T21:21:34.0641736Z    |
2019-12-14T21:21:34.0641796Z LL |     b: isize,
2019-12-14T21:21:34.0641921Z 
2019-12-14T21:21:34.0642002Z error: missing documentation for a struct
2019-12-14T21:21:34.0642247Z   --> $DIR/missing-doc.rs:18:1
2019-12-14T21:21:34.0642345Z    |
2019-12-14T21:21:34.0642345Z    |
2019-12-14T21:21:34.0642407Z LL | / pub struct PubFoo {
2019-12-14T21:21:34.0642494Z LL | |     pub a: isize,
2019-12-14T21:21:34.0642562Z LL | |     b: isize,
2019-12-14T21:21:34.0642726Z    | |_^
2019-12-14T21:21:34.0642869Z 
2019-12-14T21:21:34.0645579Z error: missing documentation for a struct field
2019-12-14T21:21:34.0646748Z   --> $DIR/missing-doc.rs:19:5
2019-12-14T21:21:34.0646748Z   --> $DIR/missing-doc.rs:19:5
2019-12-14T21:21:34.0646840Z    |
2019-12-14T21:21:34.0646924Z LL |     pub a: isize,
2019-12-14T21:21:34.0647055Z 
2019-12-14T21:21:34.0647122Z error: missing documentation for a struct field
2019-12-14T21:21:34.0647406Z   --> $DIR/missing-doc.rs:20:5
2019-12-14T21:21:34.0647479Z    |
2019-12-14T21:21:34.0647479Z    |
2019-12-14T21:21:34.0647558Z LL |     b: isize,
2019-12-14T21:21:34.0647683Z 
2019-12-14T21:21:34.0647747Z error: missing documentation for a module
2019-12-14T21:21:34.0648011Z   --> $DIR/missing-doc.rs:29:1
2019-12-14T21:21:34.0648115Z    |
2019-12-14T21:21:34.0648115Z    |
2019-12-14T21:21:34.0648179Z LL | mod module_no_dox {}
2019-12-14T21:21:34.0648310Z 
2019-12-14T21:21:34.0648373Z error: missing documentation for a module
2019-12-14T21:21:34.0648652Z   --> $DIR/missing-doc.rs:30:1
2019-12-14T21:21:34.0648741Z    |
2019-12-14T21:21:34.0648741Z    |
2019-12-14T21:21:34.0648805Z LL | pub mod pub_module_no_dox {}
2019-12-14T21:21:34.0648942Z 
2019-12-14T21:21:34.0649024Z error: missing documentation for a function
2019-12-14T21:21:34.0649272Z   --> $DIR/missing-doc.rs:33:1
2019-12-14T21:21:34.0649422Z    |
---
2019-12-14T21:21:34.0654084Z 
2019-12-14T21:21:34.0654148Z error: missing documentation for a trait
2019-12-14T21:21:34.0655268Z   --> $DIR/missing-doc.rs:53:1
2019-12-14T21:21:34.0655452Z    |
2019-12-14T21:21:34.0655514Z LL | / pub trait C {
2019-12-14T21:21:34.0655600Z LL | |     fn foo(&self);
2019-12-14T21:21:34.0655670Z LL | |     fn foo_with_impl(&self) {}
2019-12-14T21:21:34.0655817Z    | |_^
2019-12-14T21:21:34.0655875Z 
2019-12-14T21:21:34.0655952Z error: missing documentation for a trait method
2019-12-14T21:21:34.0656226Z   --> $DIR/missing-doc.rs:54:5
---
2019-12-14T21:21:34.0657588Z 
2019-12-14T21:21:34.0657653Z error: missing documentation for a trait
2019-12-14T21:21:34.0657917Z   --> $DIR/missing-doc.rs:64:1
2019-12-14T21:21:34.0657988Z    |
2019-12-14T21:21:34.0658067Z LL | / pub trait E {
2019-12-14T21:21:34.0658134Z LL | |     type AssociatedType;
2019-12-14T21:21:34.0658223Z LL | |     type AssociatedTypeDef = Self;
2019-12-14T21:21:34.0658371Z ...  |
2019-12-14T21:21:34.0658371Z ...  |
2019-12-14T21:21:34.0658459Z LL | |     fn dummy(&self) {}
2019-12-14T21:21:34.0658603Z    | |_^
2019-12-14T21:21:34.0658642Z 
2019-12-14T21:21:34.0658728Z error: missing documentation for an associated type
2019-12-14T21:21:34.0658993Z   --> $DIR/missing-doc.rs:65:5
---
2019-12-14T21:21:34.0661876Z 
2019-12-14T21:21:34.0662109Z error: missing documentation for a trait method
2019-12-14T21:21:34.0662831Z   --> $DIR/missing-doc.rs:73:5
2019-12-14T21:21:34.0662927Z    |
2019-12-14T21:21:34.0662989Z LL |     fn dummy(&self) {}
2019-12-14T21:21:34.0663119Z 
2019-12-14T21:21:34.0663198Z error: missing documentation for a method
2019-12-14T21:21:34.0663445Z   --> $DIR/missing-doc.rs:77:5
2019-12-14T21:21:34.0663537Z    |
---
2019-12-14T21:21:34.0666373Z 
2019-12-14T21:21:34.0666453Z error: missing documentation for an enum
2019-12-14T21:21:34.0666818Z   --> $DIR/missing-doc.rs:112:1
2019-12-14T21:21:34.0666892Z    |
2019-12-14T21:21:34.0666972Z LL | / enum Baz {
2019-12-14T21:21:34.0667041Z LL | |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0667129Z LL | |     BarB,
2019-12-14T21:21:34.0667275Z    | |_^
2019-12-14T21:21:34.0667312Z 
2019-12-14T21:21:34.0667393Z error: missing documentation for a variant
2019-12-14T21:21:34.0667646Z   --> $DIR/missing-doc.rs:113:5
2019-12-14T21:21:34.0667646Z   --> $DIR/missing-doc.rs:113:5
2019-12-14T21:21:34.0667735Z    |
2019-12-14T21:21:34.0667796Z LL |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0667932Z 
2019-12-14T21:21:34.0668025Z error: missing documentation for a struct field
2019-12-14T21:21:34.0668283Z   --> $DIR/missing-doc.rs:113:12
2019-12-14T21:21:34.0668373Z    |
2019-12-14T21:21:34.0668373Z    |
2019-12-14T21:21:34.0668434Z LL |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0668577Z 
2019-12-14T21:21:34.0668660Z error: missing documentation for a struct field
2019-12-14T21:21:34.0668915Z   --> $DIR/missing-doc.rs:113:22
2019-12-14T21:21:34.0669005Z    |
2019-12-14T21:21:34.0669005Z    |
2019-12-14T21:21:34.0669067Z LL |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0669203Z 
2019-12-14T21:21:34.0669284Z error: missing documentation for a variant
2019-12-14T21:21:34.0669529Z   --> $DIR/missing-doc.rs:114:5
2019-12-14T21:21:34.0669618Z    |
2019-12-14T21:21:34.0669618Z    |
2019-12-14T21:21:34.0669679Z LL |     BarB,
2019-12-14T21:21:34.0669762Z    |     ^^^^
2019-12-14T21:21:34.0669802Z 
2019-12-14T21:21:34.0669882Z error: missing documentation for an enum
2019-12-14T21:21:34.0670126Z   --> $DIR/missing-doc.rs:117:1
2019-12-14T21:21:34.0670246Z    |
2019-12-14T21:21:34.0670307Z LL | / pub enum PubBaz {
2019-12-14T21:21:34.0670393Z LL | |     PubBazA { a: isize },
2019-12-14T21:21:34.0670540Z    | |_^
2019-12-14T21:21:34.0670577Z 
2019-12-14T21:21:34.0670665Z error: missing documentation for a variant
2019-12-14T21:21:34.0670916Z   --> $DIR/missing-doc.rs:118:5
2019-12-14T21:21:34.0670916Z   --> $DIR/missing-doc.rs:118:5
2019-12-14T21:21:34.0671005Z    |
2019-12-14T21:21:34.0671083Z LL |     PubBazA { a: isize },
2019-12-14T21:21:34.0671197Z 
2019-12-14T21:21:34.0671281Z error: missing documentation for a struct field
2019-12-14T21:21:34.0671547Z   --> $DIR/missing-doc.rs:118:15
2019-12-14T21:21:34.0671619Z    |
2019-12-14T21:21:34.0671619Z    |
2019-12-14T21:21:34.0671697Z LL |     PubBazA { a: isize },
2019-12-14T21:21:34.0671812Z 
2019-12-14T21:21:34.0671893Z error: missing documentation for an enum
2019-12-14T21:21:34.0672154Z   --> $DIR/missing-doc.rs:122:1
2019-12-14T21:21:34.0672300Z    |
2019-12-14T21:21:34.0672300Z    |
2019-12-14T21:21:34.0672385Z LL | / pub enum PubBaz2 {
2019-12-14T21:21:34.0672452Z LL | |     /// dox
2019-12-14T21:21:34.0672536Z LL | |     PubBaz2A {
2019-12-14T21:21:34.0672604Z LL | |         /// dox
2019-12-14T21:21:34.0672690Z LL | |         a: isize,
2019-12-14T21:21:34.0672845Z LL | | }
2019-12-14T21:21:34.0672907Z    | |_^
2019-12-14T21:21:34.0672971Z 
2019-12-14T21:21:34.0673035Z error: missing documentation for a variant
2019-12-14T21:21:34.0673035Z error: missing documentation for a variant
2019-12-14T21:21:34.0673333Z   --> $DIR/missing-doc.rs:124:5
2019-12-14T21:21:34.0673407Z    |
2019-12-14T21:21:34.0673485Z LL | /     PubBaz2A {
2019-12-14T21:21:34.0673569Z LL | |         /// dox
2019-12-14T21:21:34.0673637Z LL | |         a: isize,
2019-12-14T21:21:34.0673787Z    | |_____^
2019-12-14T21:21:34.0673827Z 
2019-12-14T21:21:34.0673911Z error: missing documentation for a struct field
2019-12-14T21:21:34.0674179Z   --> $DIR/missing-doc.rs:126:9
2019-12-14T21:21:34.0674179Z   --> $DIR/missing-doc.rs:126:9
2019-12-14T21:21:34.0674259Z    |
2019-12-14T21:21:34.0674338Z LL |         a: isize,
2019-12-14T21:21:34.0674449Z 
2019-12-14T21:21:34.0674533Z error: missing documentation for a constant
2019-12-14T21:21:34.0674798Z   --> $DIR/missing-doc.rs:138:1
2019-12-14T21:21:34.0674956Z    |
---
2019-12-14T21:21:34.0695168Z 
2019-12-14T21:21:34.0695252Z error: missing documentation for a module
2019-12-14T21:21:34.0696416Z   --> $DIR/missing-doc.rs:163:5
2019-12-14T21:21:34.0696531Z    |
2019-12-14T21:21:34.0697348Z LL | /     pub mod globbed {
2019-12-14T21:21:34.0697483Z LL | |         /// dox
2019-12-14T21:21:34.0697572Z LL | |         pub fn also_documented() {}
2019-12-14T21:21:34.0697651Z LL | |         pub fn also_undocumented1() {}
2019-12-14T21:21:34.0698463Z LL | |         fn also_undocumented2() {}
2019-12-14T21:21:34.0698781Z    | |_____^
2019-12-14T21:21:34.0698822Z 
2019-12-14T21:21:34.0699693Z error: missing documentation for a function
2019-12-14T21:21:34.0701081Z   --> $DIR/missing-doc.rs:165:9
---
2019-12-14T21:21:34.0707088Z 
2019-12-14T21:21:34.0707168Z error: missing documentation for a module
2019-12-14T21:21:34.0708246Z   --> $DIR/missing-doc.rs:171:1
2019-12-14T21:21:34.0709165Z    |
2019-12-14T21:21:34.0709257Z LL | / pub mod public_interface {
2019-12-14T21:21:34.0709360Z LL | |     pub use internal_impl::documented as foo;
2019-12-14T21:21:34.0709457Z LL | |     pub use internal_impl::globbed::*;
2019-12-14T21:21:34.0710274Z LL | |     pub use internal_impl::undocumented1 as bar;
2019-12-14T21:21:34.0710495Z LL | |     pub use internal_impl::{documented, undocumented2};
2019-12-14T21:21:34.0711370Z    | |_^
2019-12-14T21:21:34.0711429Z 
2019-12-14T21:21:34.0711516Z error: aborting due to 57 previous errors
2019-12-14T21:21:34.0711564Z 
---
2019-12-14T21:21:34.0772798Z 
2019-12-14T21:21:34.0772887Z error: missing documentation for a type alias
2019-12-14T21:21:34.0773150Z   --> $DIR/missing-doc.rs:11:1
2019-12-14T21:21:34.0773242Z    |
2019-12-14T21:21:34.0773322Z LL | pub type PubTypedef = String;
2019-12-14T21:21:34.0773618Z 
2019-12-14T21:21:34.0773720Z error: missing documentation for a struct
2019-12-14T21:21:34.0774029Z   --> $DIR/missing-doc.rs:13:1
2019-12-14T21:21:34.0774103Z    |
2019-12-14T21:21:34.0774103Z    |
2019-12-14T21:21:34.0774184Z LL | / struct Foo {
2019-12-14T21:21:34.0774255Z LL | |     a: isize,
2019-12-14T21:21:34.0774350Z LL | |     b: isize,
2019-12-14T21:21:34.0774602Z    | |_^
2019-12-14T21:21:34.0774639Z 
2019-12-14T21:21:34.0774724Z error: missing documentation for a struct field
2019-12-14T21:21:34.0774980Z   --> $DIR/missing-doc.rs:14:5
2019-12-14T21:21:34.0774980Z   --> $DIR/missing-doc.rs:14:5
2019-12-14T21:21:34.0775070Z    |
2019-12-14T21:21:34.0775134Z LL |     a: isize,
2019-12-14T21:21:34.0775258Z 
2019-12-14T21:21:34.0775343Z error: missing documentation for a struct field
2019-12-14T21:21:34.0775590Z   --> $DIR/missing-doc.rs:15:5
2019-12-14T21:21:34.0775678Z    |
2019-12-14T21:21:34.0775678Z    |
2019-12-14T21:21:34.0775739Z LL |     b: isize,
2019-12-14T21:21:34.0775875Z 
2019-12-14T21:21:34.0775954Z error: missing documentation for a struct
2019-12-14T21:21:34.0776203Z   --> $DIR/missing-doc.rs:18:1
2019-12-14T21:21:34.0776291Z    |
2019-12-14T21:21:34.0776291Z    |
2019-12-14T21:21:34.0776353Z LL | / pub struct PubFoo {
2019-12-14T21:21:34.0776441Z LL | |     pub a: isize,
2019-12-14T21:21:34.0776628Z LL | |     b: isize,
2019-12-14T21:21:34.0776772Z    | |_^
2019-12-14T21:21:34.0776827Z 
2019-12-14T21:21:34.0776895Z error: missing documentation for a struct field
2019-12-14T21:21:34.0777193Z   --> $DIR/missing-doc.rs:19:5
2019-12-14T21:21:34.0777193Z   --> $DIR/missing-doc.rs:19:5
2019-12-14T21:21:34.0777266Z    |
2019-12-14T21:21:34.0777343Z LL |     pub a: isize,
2019-12-14T21:21:34.0777472Z 
2019-12-14T21:21:34.0777541Z error: missing documentation for a struct field
2019-12-14T21:21:34.0778009Z   --> $DIR/missing-doc.rs:20:5
2019-12-14T21:21:34.0778163Z    |
2019-12-14T21:21:34.0778163Z    |
2019-12-14T21:21:34.0778236Z LL |     b: isize,
2019-12-14T21:21:34.0778636Z 
2019-12-14T21:21:34.0778856Z error: missing documentation for a module
2019-12-14T21:21:34.0779093Z   --> $DIR/missing-doc.rs:29:1
2019-12-14T21:21:34.0779155Z    |
2019-12-14T21:21:34.0779155Z    |
2019-12-14T21:21:34.0779225Z LL | mod module_no_dox {}
2019-12-14T21:21:34.0779348Z 
2019-12-14T21:21:34.0779403Z error: missing documentation for a module
2019-12-14T21:21:34.0779637Z   --> $DIR/missing-doc.rs:30:1
2019-12-14T21:21:34.0779715Z    |
2019-12-14T21:21:34.0779715Z    |
2019-12-14T21:21:34.0779770Z LL | pub mod pub_module_no_dox {}
2019-12-14T21:21:34.0779889Z 
2019-12-14T21:21:34.0779945Z error: missing documentation for a function
2019-12-14T21:21:34.0780177Z   --> $DIR/missing-doc.rs:34:1
2019-12-14T21:21:34.0780255Z    |
---
2019-12-14T21:21:34.0781011Z 
2019-12-14T21:21:34.0781081Z error: missing documentation for a trait
2019-12-14T21:21:34.0781307Z   --> $DIR/missing-doc.rs:53:1
2019-12-14T21:21:34.0781387Z    |
2019-12-14T21:21:34.0781441Z LL | / pub trait C {
2019-12-14T21:21:34.0781516Z LL | |     fn foo(&self);
2019-12-14T21:21:34.0781578Z LL | |     fn foo_with_impl(&self) {}
2019-12-14T21:21:34.0781710Z    | |_^
2019-12-14T21:21:34.0781758Z 
2019-12-14T21:21:34.0781816Z error: missing documentation for a trait method
2019-12-14T21:21:34.0782050Z   --> $DIR/missing-doc.rs:54:5
---
2019-12-14T21:21:34.0788088Z 
2019-12-14T21:21:34.0788167Z error: missing documentation for an enum
2019-12-14T21:21:34.0788417Z   --> $DIR/missing-doc.rs:112:1
2019-12-14T21:21:34.0788627Z    |
2019-12-14T21:21:34.0788696Z LL | / enum Baz {
2019-12-14T21:21:34.0788863Z LL | |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0788935Z LL | |     BarB,
2019-12-14T21:21:34.0789080Z    | |_^
2019-12-14T21:21:34.0789133Z 
2019-12-14T21:21:34.0789198Z error: missing documentation for a variant
2019-12-14T21:21:34.0789468Z   --> $DIR/missing-doc.rs:113:5
2019-12-14T21:21:34.0789468Z   --> $DIR/missing-doc.rs:113:5
2019-12-14T21:21:34.0789542Z    |
2019-12-14T21:21:34.0789623Z LL |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0789757Z 
2019-12-14T21:21:34.0789825Z error: missing documentation for a struct field
2019-12-14T21:21:34.0790853Z   --> $DIR/missing-doc.rs:113:12
2019-12-14T21:21:34.0790934Z    |
2019-12-14T21:21:34.0790934Z    |
2019-12-14T21:21:34.0791026Z LL |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0791160Z 
2019-12-14T21:21:34.0791229Z error: missing documentation for a struct field
2019-12-14T21:21:34.0791503Z   --> $DIR/missing-doc.rs:113:22
2019-12-14T21:21:34.0791584Z    |
2019-12-14T21:21:34.0791584Z    |
2019-12-14T21:21:34.0791665Z LL |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0791799Z 
2019-12-14T21:21:34.0791863Z error: missing documentation for a variant
2019-12-14T21:21:34.0792129Z   --> $DIR/missing-doc.rs:114:5
2019-12-14T21:21:34.0792200Z    |
2019-12-14T21:21:34.0792200Z    |
2019-12-14T21:21:34.0792278Z LL |     BarB,
2019-12-14T21:21:34.0792360Z    |     ^^^^
2019-12-14T21:21:34.0792400Z 
2019-12-14T21:21:34.0792463Z error: missing documentation for an enum
2019-12-14T21:21:34.0792726Z   --> $DIR/missing-doc.rs:117:1
2019-12-14T21:21:34.0792813Z    |
2019-12-14T21:21:34.0792874Z LL | / pub enum PubBaz {
2019-12-14T21:21:34.0793072Z LL | |     PubBazA { a: isize },
2019-12-14T21:21:34.0793232Z    | |_^
2019-12-14T21:21:34.0793270Z 
2019-12-14T21:21:34.0793351Z error: missing documentation for a variant
2019-12-14T21:21:34.0793638Z   --> $DIR/missing-doc.rs:118:5
2019-12-14T21:21:34.0793638Z   --> $DIR/missing-doc.rs:118:5
2019-12-14T21:21:34.0793846Z    |
2019-12-14T21:21:34.0793908Z LL |     PubBazA { a: isize },
2019-12-14T21:21:34.0794040Z 
2019-12-14T21:21:34.0794193Z error: missing documentation for a struct field
2019-12-14T21:21:34.0794536Z   --> $DIR/missing-doc.rs:118:15
2019-12-14T21:21:34.0794625Z    |
2019-12-14T21:21:34.0794625Z    |
2019-12-14T21:21:34.0794687Z LL |     PubBazA { a: isize },
2019-12-14T21:21:34.0794820Z 
2019-12-14T21:21:34.0794902Z error: missing documentation for a constant
2019-12-14T21:21:34.0795152Z   --> $DIR/missing-doc.rs:138:1
2019-12-14T21:21:34.0795401Z    |
---
2019-12-14T21:21:34.0803524Z  
2019-12-14T21:21:34.0803592Z  error: missing documentation for a type alias
2019-12-14T21:21:34.0803854Z    --> $DIR/missing-doc.rs:11:1
2019-12-14T21:21:34.0803927Z     |
2019-12-14T21:21:34.0804014Z  LL | pub type PubTypedef = String;
2019-12-14T21:21:34.0804328Z  
2019-12-14T21:21:34.0804548Z  error: missing documentation for a struct
2019-12-14T21:21:34.0804837Z    --> $DIR/missing-doc.rs:13:1
2019-12-14T21:21:34.0805014Z     |
2019-12-14T21:21:34.0805014Z     |
2019-12-14T21:21:34.0805089Z  LL | / struct Foo {
2019-12-14T21:21:34.0805171Z  LL | |     a: isize,
2019-12-14T21:21:34.0805236Z  LL | |     b: isize,
2019-12-14T21:21:34.0805378Z     | |_^
2019-12-14T21:21:34.0805452Z  
2019-12-14T21:21:34.0805518Z  error: missing documentation for a struct field
2019-12-14T21:21:34.0805811Z    --> $DIR/missing-doc.rs:14:5
2019-12-14T21:21:34.0805811Z    --> $DIR/missing-doc.rs:14:5
2019-12-14T21:21:34.0805881Z     |
2019-12-14T21:21:34.0805956Z  LL |     a: isize,
2019-12-14T21:21:34.0806307Z  
2019-12-14T21:21:34.0806546Z  error: missing documentation for a struct field
2019-12-14T21:21:34.0806775Z    --> $DIR/missing-doc.rs:15:5
2019-12-14T21:21:34.0811643Z     |
2019-12-14T21:21:34.0811643Z     |
2019-12-14T21:21:34.0811717Z  LL |     b: isize,
2019-12-14T21:21:34.0811861Z  
2019-12-14T21:21:34.0811940Z  error: missing documentation for a struct
2019-12-14T21:21:34.0812469Z    --> $DIR/missing-doc.rs:18:1
2019-12-14T21:21:34.0812573Z     |
2019-12-14T21:21:34.0812573Z     |
2019-12-14T21:21:34.0812652Z  LL | / pub struct PubFoo {
2019-12-14T21:21:34.0812724Z  LL | |     pub a: isize,
2019-12-14T21:21:34.0812810Z  LL | |     b: isize,
2019-12-14T21:21:34.0812956Z     | |_^
2019-12-14T21:21:34.0813016Z  
2019-12-14T21:21:34.0813173Z  error: missing documentation for a struct field
2019-12-14T21:21:34.0813430Z    --> $DIR/missing-doc.rs:19:5
2019-12-14T21:21:34.0813430Z    --> $DIR/missing-doc.rs:19:5
2019-12-14T21:21:34.0813522Z     |
2019-12-14T21:21:34.0813632Z  LL |     pub a: isize,
2019-12-14T21:21:34.0813799Z  
2019-12-14T21:21:34.0813866Z  error: missing documentation for a struct field
2019-12-14T21:21:34.0814300Z    --> $DIR/missing-doc.rs:20:5
2019-12-14T21:21:34.0814372Z     |
2019-12-14T21:21:34.0814372Z     |
2019-12-14T21:21:34.0814449Z  LL |     b: isize,
2019-12-14T21:21:34.0814591Z  
2019-12-14T21:21:34.0814654Z  error: missing documentation for a module
2019-12-14T21:21:34.0815165Z    --> $DIR/missing-doc.rs:29:1
2019-12-14T21:21:34.0815236Z     |
2019-12-14T21:21:34.0815236Z     |
2019-12-14T21:21:34.0815314Z  LL | mod module_no_dox {}
2019-12-14T21:21:34.0815467Z  
2019-12-14T21:21:34.0815547Z  error: missing documentation for a module
2019-12-14T21:21:34.0815794Z    --> $DIR/missing-doc.rs:30:1
2019-12-14T21:21:34.0815882Z     |
2019-12-14T21:21:34.0815882Z     |
2019-12-14T21:21:34.0815945Z  LL | pub mod pub_module_no_dox {}
2019-12-14T21:21:34.0816103Z  
2019-12-14T21:21:34.0816185Z  error: missing documentation for a function
2019-12-14T21:21:34.0816435Z +  --> $DIR/missing-doc.rs:33:1
2019-12-14T21:21:34.0816523Z +   |
---
2019-12-14T21:21:34.0821813Z +
2019-12-14T21:21:34.0821873Z +error: missing documentation for a trait
2019-12-14T21:21:34.0822369Z    --> $DIR/missing-doc.rs:53:1
2019-12-14T21:21:34.0822449Z     |
2019-12-14T21:21:34.0822524Z  LL | / pub trait C {
2019-12-14T21:21:34.0822590Z  LL | |     fn foo(&self);
2019-12-14T21:21:34.0822676Z  LL | |     fn foo_with_impl(&self) {}
2019-12-14T21:21:34.0822820Z     | |_^
2019-12-14T21:21:34.0822900Z  
2019-12-14T21:21:34.0822965Z  error: missing documentation for a trait method
2019-12-14T21:21:34.0823490Z    --> $DIR/missing-doc.rs:54:5
---
2019-12-14T21:21:34.0824437Z  
2019-12-14T21:21:34.0824517Z +error: missing documentation for a trait
2019-12-14T21:21:34.0824774Z +  --> $DIR/missing-doc.rs:64:1
2019-12-14T21:21:34.0824865Z +   |
2019-12-14T21:21:34.0824925Z +LL | / pub trait E {
2019-12-14T21:21:34.0825011Z +LL | |     type AssociatedType;
2019-12-14T21:21:34.0825084Z +LL | |     type AssociatedTypeDef = Self;
2019-12-14T21:21:34.0825317Z +...  |
2019-12-14T21:21:34.0825317Z +...  |
2019-12-14T21:21:34.0825378Z +LL | |     fn dummy(&self) {}
2019-12-14T21:21:34.0825525Z +   | |_^
2019-12-14T21:21:34.0825718Z +
2019-12-14T21:21:34.0825788Z  error: missing documentation for an associated type
2019-12-14T21:21:34.0826066Z    --> $DIR/missing-doc.rs:65:5
---
2019-12-14T21:21:34.0828445Z +
2019-12-14T21:21:34.0828511Z +error: missing documentation for a trait method
2019-12-14T21:21:34.0828774Z +  --> $DIR/missing-doc.rs:73:5
2019-12-14T21:21:34.0828870Z +   |
2019-12-14T21:21:34.0828932Z +LL |     fn dummy(&self) {}
2019-12-14T21:21:34.0829086Z +
2019-12-14T21:21:34.0829166Z  error: missing documentation for a method
2019-12-14T21:21:34.0829416Z    --> $DIR/missing-doc.rs:77:5
2019-12-14T21:21:34.0829591Z     |
---
2019-12-14T21:21:34.0833412Z  
2019-12-14T21:21:34.0833493Z  error: missing documentation for an enum
2019-12-14T21:21:34.0833808Z    --> $DIR/missing-doc.rs:112:1
2019-12-14T21:21:34.0833882Z     |
2019-12-14T21:21:34.0833959Z  LL | / enum Baz {
2019-12-14T21:21:34.0834029Z  LL | |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0834277Z  LL | |     BarB,
2019-12-14T21:21:34.0834427Z     | |_^
2019-12-14T21:21:34.0834484Z  
2019-12-14T21:21:34.0834739Z  error: missing documentation for a variant
2019-12-14T21:21:34.0835049Z    --> $DIR/missing-doc.rs:113:5
2019-12-14T21:21:34.0835049Z    --> $DIR/missing-doc.rs:113:5
2019-12-14T21:21:34.0835140Z     |
2019-12-14T21:21:34.0835413Z  LL |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0835567Z  
2019-12-14T21:21:34.0835631Z  error: missing documentation for a struct field
2019-12-14T21:21:34.0836084Z    --> $DIR/missing-doc.rs:113:12
2019-12-14T21:21:34.0836156Z     |
2019-12-14T21:21:34.0836156Z     |
2019-12-14T21:21:34.0836238Z  LL |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0836392Z  
2019-12-14T21:21:34.0836477Z  error: missing documentation for a struct field
2019-12-14T21:21:34.0836731Z    --> $DIR/missing-doc.rs:113:22
2019-12-14T21:21:34.0836820Z     |
2019-12-14T21:21:34.0836820Z     |
2019-12-14T21:21:34.0836883Z  LL |     BazA { a: isize, b: isize },
2019-12-14T21:21:34.0837190Z  
2019-12-14T21:21:34.0837272Z  error: missing documentation for a variant
2019-12-14T21:21:34.0837557Z    --> $DIR/missing-doc.rs:114:5
2019-12-14T21:21:34.0837646Z     |
2019-12-14T21:21:34.0837646Z     |
2019-12-14T21:21:34.0837708Z  LL |     BarB,
2019-12-14T21:21:34.0837801Z     |     ^^^^
2019-12-14T21:21:34.0837880Z  
2019-12-14T21:21:34.0837943Z  error: missing documentation for an enum
2019-12-14T21:21:34.0838371Z    --> $DIR/missing-doc.rs:117:1
2019-12-14T21:21:34.0838443Z     |
2019-12-14T21:21:34.0838518Z  LL | / pub enum PubBaz {
2019-12-14T21:21:34.0838586Z  LL | |     PubBazA { a: isize },
2019-12-14T21:21:34.0838730Z     | |_^
2019-12-14T21:21:34.0838804Z  
2019-12-14T21:21:34.0838866Z  error: missing documentation for a variant
2019-12-14T21:21:34.0839124Z    --> $DIR/missing-doc.rs:118:5
2019-12-14T21:21:34.0839124Z    --> $DIR/missing-doc.rs:118:5
2019-12-14T21:21:34.0839210Z     |
2019-12-14T21:21:34.0839271Z  LL |     PubBazA { a: isize },
2019-12-14T21:21:34.0839430Z  
2019-12-14T21:21:34.0839513Z  error: missing documentation for a struct field
2019-12-14T21:21:34.0839759Z    --> $DIR/missing-doc.rs:118:15
2019-12-14T21:21:34.0839846Z     |
2019-12-14T21:21:34.0839846Z     |
2019-12-14T21:21:34.0839907Z  LL |     PubBazA { a: isize },
2019-12-14T21:21:34.0840146Z  
2019-12-14T21:21:34.0840224Z +error: missing documentation for an enum
2019-12-14T21:21:34.0840504Z +  --> $DIR/missing-doc.rs:122:1
2019-12-14T21:21:34.0840574Z +   |
2019-12-14T21:21:34.0840574Z +   |
2019-12-14T21:21:34.0840650Z +LL | / pub enum PubBaz2 {
2019-12-14T21:21:34.0840716Z +LL | |     /// dox
2019-12-14T21:21:34.0840798Z +LL | |     PubBaz2A {
2019-12-14T21:21:34.0840864Z +LL | |         /// dox
2019-12-14T21:21:34.0840946Z +LL | |         a: isize,
2019-12-14T21:21:34.0841091Z +LL | | }
2019-12-14T21:21:34.0841149Z +   | |_^
2019-12-14T21:21:34.0841222Z +
2019-12-14T21:21:34.0841297Z +error: missing documentation for a variant
2019-12-14T21:21:34.0841297Z +error: missing documentation for a variant
2019-12-14T21:21:34.0841572Z +  --> $DIR/missing-doc.rs:124:5
2019-12-14T21:21:34.0841658Z +   |
2019-12-14T21:21:34.0841717Z +LL | /     PubBaz2A {
2019-12-14T21:21:34.0841970Z +LL | |         /// dox
2019-12-14T21:21:34.0842034Z +LL | |         a: isize,
2019-12-14T21:21:34.0842185Z +   | |_____^
2019-12-14T21:21:34.0842259Z +
2019-12-14T21:21:34.0842322Z +error: missing documentation for a struct field
2019-12-14T21:21:34.0842576Z +  --> $DIR/missing-doc.rs:126:9
2019-12-14T21:21:34.0842576Z +  --> $DIR/missing-doc.rs:126:9
2019-12-14T21:21:34.0842659Z +   |
2019-12-14T21:21:34.0842716Z +LL |         a: isize,
2019-12-14T21:21:34.0842857Z +
2019-12-14T21:21:34.0842936Z  error: missing documentation for a constant
2019-12-14T21:21:34.0843170Z    --> $DIR/missing-doc.rs:138:1
2019-12-14T21:21:34.0843254Z     |
---
2019-12-14T21:21:34.0851872Z  
2019-12-14T21:21:34.0851947Z +error: missing documentation for a module
2019-12-14T21:21:34.0852194Z +  --> $DIR/missing-doc.rs:163:5
2019-12-14T21:21:34.0852260Z +   |
2019-12-14T21:21:34.0852331Z +LL | /     pub mod globbed {
2019-12-14T21:21:34.0852393Z +LL | |         /// dox
2019-12-14T21:21:34.0852475Z +LL | |         pub fn also_documented() {}
2019-12-14T21:21:34.0852546Z +LL | |         pub fn also_undocumented1() {}
2019-12-14T21:21:34.0852631Z +LL | |         fn also_undocumented2() {}
2019-12-14T21:21:34.0852769Z +   | |_____^
2019-12-14T21:21:34.0852851Z +
2019-12-14T21:21:34.0852909Z  error: missing documentation for a function
2019-12-14T21:21:34.0853319Z +  --> $DIR/missing-doc.rs:165:9
---
2019-12-14T21:21:34.0855766Z -error: aborting due to 39 previous errors
2019-12-14T21:21:34.0855856Z +error: missing documentation for a module
2019-12-14T21:21:34.0856446Z +  --> $DIR/missing-doc.rs:171:1
2019-12-14T21:21:34.0856535Z +   |
2019-12-14T21:21:34.0856607Z +LL | / pub mod public_interface {
2019-12-14T21:21:34.0856705Z +LL | |     pub use internal_impl::documented as foo;
2019-12-14T21:21:34.0856786Z +LL | |     pub use internal_impl::globbed::*;
2019-12-14T21:21:34.0856887Z +LL | |     pub use internal_impl::undocumented1 as bar;
2019-12-14T21:21:34.0856974Z +LL | |     pub use internal_impl::{documented, undocumented2};
2019-12-14T21:21:34.0857153Z +   | |_^
2019-12-14T21:21:34.0857213Z +
2019-12-14T21:21:34.0857294Z +error: aborting due to 57 previous errors
2019-12-14T21:21:34.0857363Z  
---
2019-12-14T21:21:34.0863727Z 
2019-12-14T21:21:34.0864148Z ------------------------------------------
2019-12-14T21:21:34.0864236Z stderr:
2019-12-14T21:21:34.0864460Z ------------------------------------------
2019-12-14T21:21:34.0866390Z {"message":"missing documentation for a type alias","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":307,"byte_end":329,"line_start":10,"line_end":10,"column_start":1,"column_end":23,"is_primary":true,"text":[{"text":"type Typedef = String;","highlight_start":1,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::missing-docs-in-private-items` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: missing documentation for a type alias\n  --> tests/ui/missing-doc.rs:10:1\n   |\nLL | type Typedef = String;\n   | ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::missing-docs-in-private-items` implied by `-D warnings`\n\n"}
2019-12-14T21:21:34.0867862Z {"message":"missing documentation for a type alias","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":330,"byte_end":359,"line_start":11,"line_end":11,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"pub type PubTypedef = String;","highlight_start":1,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a type alias\n  --> tests/ui/missing-doc.rs:11:1\n   |\nLL | pub type PubTypedef = String;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0869435Z {"message":"missing documentation for a struct","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":361,"byte_end":403,"line_start":13,"line_end":16,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"struct Foo {","highlight_start":1,"highlight_end":13},{"text":"    a: isize,","highlight_start":1,"highlight_end":14},{"text":"    b: isize,","highlight_start":1,"highlight_end":14},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct\n  --> tests/ui/missing-doc.rs:13:1\n   |\nLL | / struct Foo {\nLL | |     a: isize,\nLL | |     b: isize,\nLL | | }\n   | |_^\n\n"}
2019-12-14T21:21:34.0870901Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":378,"byte_end":386,"line_start":14,"line_end":14,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    a: isize,","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:14:5\n   |\nLL |     a: isize,\n   |     ^^^^^^^^\n\n"}
2019-12-14T21:21:34.0872192Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":392,"byte_end":400,"line_start":15,"line_end":15,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    b: isize,","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:15:5\n   |\nLL |     b: isize,\n   |     ^^^^^^^^\n\n"}
2019-12-14T21:21:34.0873850Z {"message":"missing documentation for a struct","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":405,"byte_end":458,"line_start":18,"line_end":21,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub struct PubFoo {","highlight_start":1,"highlight_end":20},{"text":"    pub a: isize,","highlight_start":1,"highlight_end":18},{"text":"    b: isize,","highlight_start":1,"highlight_end":14},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct\n  --> tests/ui/missing-doc.rs:18:1\n   |\nLL | / pub struct PubFoo {\nLL | |     pub a: isize,\nLL | |     b: isize,\nLL | | }\n   | |_^\n\n"}
2019-12-14T21:21:34.0875482Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":429,"byte_end":441,"line_start":19,"line_end":19,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    pub a: isize,","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:19:5\n   |\nLL |     pub a: isize,\n   |     ^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0877379Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":447,"byte_end":455,"line_start":20,"line_end":20,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    b: isize,","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:20:5\n   |\nLL |     b: isize,\n   |     ^^^^^^^^\n\n"}
2019-12-14T21:21:34.0878741Z {"message":"missing documentation for a module","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":568,"byte_end":588,"line_start":29,"line_end":29,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"mod module_no_dox {}","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a module\n  --> tests/ui/missing-doc.rs:29:1\n   |\nLL | mod module_no_dox {}\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0880008Z {"message":"missing documentation for a module","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":589,"byte_end":617,"line_start":30,"line_end":30,"column_start":1,"column_end":29,"is_primary":true,"text":[{"text":"pub mod pub_module_no_dox {}","highlight_start":1,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a module\n  --> tests/ui/missing-doc.rs:30:1\n   |\nLL | pub mod pub_module_no_dox {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0881426Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":627,"byte_end":642,"line_start":33,"line_end":33,"column_start":1,"column_end":16,"is_primary":true,"text":[{"text":"pub fn foo() {}","highlight_start":1,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:33:1\n   |\nLL | pub fn foo() {}\n   | ^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0882771Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":643,"byte_end":659,"line_start":34,"line_end":34,"column_start":1,"column_end":17,"is_primary":true,"text":[{"text":"pub fn foo2() {}","highlight_start":1,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:34:1\n   |\nLL | pub fn foo2() {}\n   | ^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0964596Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":660,"byte_end":672,"line_start":35,"line_end":35,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"fn foo3() {}","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:35:1\n   |\nLL | fn foo3() {}\n   | ^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0966896Z {"message":"missing documentation for a trait","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":747,"byte_end":836,"line_start":40,"line_end":45,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub trait A {","highlight_start":1,"highlight_end":14},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    fn foo(&self);","highlight_start":1,"highlight_end":19},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    fn foo_with_impl(&self) {}","highlight_start":1,"highlight_end":31},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait\n  --> tests/ui/missing-doc.rs:40:1\n   |\nLL | / pub trait A {\nLL | |     /// dox\nLL | |     fn foo(&self);\nLL | |     /// dox\nLL | |     fn foo_with_impl(&self) {}\nLL | | }\n   | |_^\n\n"}
2019-12-14T21:21:34.0968942Z {"message":"missing documentation for a trait method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":777,"byte_end":791,"line_start":42,"line_end":42,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    fn foo(&self);","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait method\n  --> tests/ui/missing-doc.rs:42:5\n   |\nLL |     fn foo(&self);\n   |     ^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0970494Z {"message":"missing documentation for a trait method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":808,"byte_end":834,"line_start":44,"line_end":44,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    fn foo_with_impl(&self) {}","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait method\n  --> tests/ui/missing-doc.rs:44:5\n   |\nLL |     fn foo_with_impl(&self) {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0972159Z {"message":"missing documentation for a trait","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":949,"byte_end":1014,"line_start":53,"line_end":56,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub trait C {","highlight_start":1,"highlight_end":14},{"text":"    fn foo(&self);","highlight_start":1,"highlight_end":19},{"text":"    fn foo_with_impl(&self) {}","highlight_start":1,"highlight_end":31},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait\n  --> tests/ui/missing-doc.rs:53:1\n   |\nLL | / pub trait C {\nLL | |     fn foo(&self);\nLL | |     fn foo_with_impl(&self) {}\nLL | | }\n   | |_^\n\n"}
2019-12-14T21:21:34.0974041Z {"message":"missing documentation for a trait method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":967,"byte_end":981,"line_start":54,"line_end":54,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    fn foo(&self);","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait method\n  --> tests/ui/missing-doc.rs:54:5\n   |\nLL |     fn foo(&self);\n   |     ^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0975311Z {"message":"missing documentation for a trait method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":986,"byte_end":1012,"line_start":55,"line_end":55,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    fn foo_with_impl(&self) {}","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait method\n  --> tests/ui/missing-doc.rs:55:5\n   |\nLL |     fn foo_with_impl(&self) {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0978126Z {"message":"missing documentation for a trait","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1112,"byte_end":1307,"line_start":64,"line_end":74,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub trait E {","highlight_start":1,"highlight_end":14},{"text":"    type AssociatedType;","highlight_start":1,"highlight_end":25},{"text":"    type AssociatedTypeDef = Self;","highlight_start":1,"highlight_end":35},{"text":"","highlight_start":1,"highlight_end":1},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    type DocumentedType;","highlight_start":1,"highlight_end":25},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    type DocumentedTypeDef = Self;","highlight_start":1,"highlight_end":35},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    fn dummy(&self) {}","highlight_start":1,"highlight_end":23},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait\n  --> tests/ui/missing-doc.rs:64:1\n   |\nLL | / pub trait E {\nLL | |     type AssociatedType;\nLL | |     type AssociatedTypeDef = Self;\nLL | |\n...  |\nLL | |     fn dummy(&self) {}\nLL | | }\n   | |_^\n\n"}
2019-12-14T21:21:34.0980003Z {"message":"missing documentation for an associated type","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1130,"byte_end":1150,"line_start":65,"line_end":65,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    type AssociatedType;","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an associated type\n  --> tests/ui/missing-doc.rs:65:5\n   |\nLL |     type AssociatedType;\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0981464Z {"message":"missing documentation for an associated type","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1155,"byte_end":1185,"line_start":66,"line_end":66,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    type AssociatedTypeDef = Self;","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an associated type\n  --> tests/ui/missing-doc.rs:66:5\n   |\nLL |     type AssociatedTypeDef = Self;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0982961Z {"message":"missing documentation for an associated type","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1203,"byte_end":1223,"line_start":69,"line_end":69,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    type DocumentedType;","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an associated type\n  --> tests/ui/missing-doc.rs:69:5\n   |\nLL |     type DocumentedType;\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0984387Z {"message":"missing documentation for an associated type","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1240,"byte_end":1270,"line_start":71,"line_end":71,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    type DocumentedTypeDef = Self;","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an associated type\n  --> tests/ui/missing-doc.rs:71:5\n   |\nLL |     type DocumentedTypeDef = Self;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0986394Z {"message":"missing documentation for a trait method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1287,"byte_end":1305,"line_start":73,"line_end":73,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    fn dummy(&self) {}","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a trait method\n  --> tests/ui/missing-doc.rs:73:5\n   |\nLL |     fn dummy(&self) {}\n   |     ^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0987683Z {"message":"missing documentation for a method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1324,"byte_end":1339,"line_start":77,"line_end":77,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    pub fn foo() {}","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a method\n  --> tests/ui/missing-doc.rs:77:5\n   |\nLL |     pub fn foo() {}\n   |     ^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0988944Z {"message":"missing documentation for a method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1344,"byte_end":1355,"line_start":78,"line_end":78,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    fn bar() {}","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a method\n  --> tests/ui/missing-doc.rs:78:5\n   |\nLL |     fn bar() {}\n   |     ^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0990334Z {"message":"missing documentation for a method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1377,"byte_end":1392,"line_start":82,"line_end":82,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    pub fn foo() {}","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a method\n  --> tests/ui/missing-doc.rs:82:5\n   |\nLL |     pub fn foo() {}\n   |     ^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0991658Z {"message":"missing documentation for a method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1409,"byte_end":1425,"line_start":84,"line_end":84,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    pub fn foo1() {}","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a method\n  --> tests/ui/missing-doc.rs:84:5\n   |\nLL |     pub fn foo1() {}\n   |     ^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0992936Z {"message":"missing documentation for a method","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1430,"byte_end":1442,"line_start":85,"line_end":85,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    fn foo2() {}","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a method\n  --> tests/ui/missing-doc.rs:85:5\n   |\nLL |     fn foo2() {}\n   |     ^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0995343Z {"message":"missing documentation for an enum","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1923,"byte_end":1978,"line_start":112,"line_end":115,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"enum Baz {","highlight_start":1,"highlight_end":11},{"text":"    BazA { a: isize, b: isize },","highlight_start":1,"highlight_end":33},{"text":"    BarB,","highlight_start":1,"highlight_end":10},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an enum\n  --> tests/ui/missing-doc.rs:112:1\n   |\nLL | / enum Baz {\nLL | |     BazA { a: isize, b: isize },\nLL | |     BarB,\nLL | | }\n   | |_^\n\n"}
2019-12-14T21:21:34.0996795Z {"message":"missing documentation for a variant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1938,"byte_end":1965,"line_start":113,"line_end":113,"column_start":5,"column_end":32,"is_primary":true,"text":[{"text":"    BazA { a: isize, b: isize },","highlight_start":5,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a variant\n  --> tests/ui/missing-doc.rs:113:5\n   |\nLL |     BazA { a: isize, b: isize },\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.0998141Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1945,"byte_end":1953,"line_start":113,"line_end":113,"column_start":12,"column_end":20,"is_primary":true,"text":[{"text":"    BazA { a: isize, b: isize },","highlight_start":12,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:113:12\n   |\nLL |     BazA { a: isize, b: isize },\n   |            ^^^^^^^^\n\n"}
2019-12-14T21:21:34.0999660Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1955,"byte_end":1963,"line_start":113,"line_end":113,"column_start":22,"column_end":30,"is_primary":true,"text":[{"text":"    BazA { a: isize, b: isize },","highlight_start":22,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:113:22\n   |\nLL |     BazA { a: isize, b: isize },\n   |                      ^^^^^^^^\n\n"}
2019-12-14T21:21:34.1001043Z {"message":"missing documentation for a variant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1971,"byte_end":1975,"line_start":114,"line_end":114,"column_start":5,"column_end":9,"is_primary":true,"text":[{"text":"    BarB,","highlight_start":5,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a variant\n  --> tests/ui/missing-doc.rs:114:5\n   |\nLL |     BarB,\n   |     ^^^^\n\n"}
2019-12-14T21:21:34.1002566Z {"message":"missing documentation for an enum","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":1980,"byte_end":2025,"line_start":117,"line_end":119,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub enum PubBaz {","highlight_start":1,"highlight_end":18},{"text":"    PubBazA { a: isize },","highlight_start":1,"highlight_end":26},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an enum\n  --> tests/ui/missing-doc.rs:117:1\n   |\nLL | / pub enum PubBaz {\nLL | |     PubBazA { a: isize },\nLL | | }\n   | |_^\n\n"}
2019-12-14T21:21:34.1004218Z {"message":"missing documentation for a variant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2002,"byte_end":2022,"line_start":118,"line_end":118,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    PubBazA { a: isize },","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a variant\n  --> tests/ui/missing-doc.rs:118:5\n   |\nLL |     PubBazA { a: isize },\n   |     ^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1005645Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2012,"byte_end":2020,"line_start":118,"line_end":118,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"    PubBazA { a: isize },","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:118:15\n   |\nLL |     PubBazA { a: isize },\n   |               ^^^^^^^^\n\n"}
2019-12-14T21:21:34.1007740Z {"message":"missing documentation for an enum","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2035,"byte_end":2123,"line_start":122,"line_end":128,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub enum PubBaz2 {","highlight_start":1,"highlight_end":19},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    PubBaz2A {","highlight_start":1,"highlight_end":15},{"text":"        /// dox","highlight_start":1,"highlight_end":16},{"text":"        a: isize,","highlight_start":1,"highlight_end":18},{"text":"    },","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for an enum\n  --> tests/ui/missing-doc.rs:122:1\n   |\nLL | / pub enum PubBaz2 {\nLL | |     /// dox\nLL | |     PubBaz2A {\nLL | |         /// dox\nLL | |         a: isize,\nLL | |     },\nLL | | }\n   | |_^\n\n"}
2019-12-14T21:21:34.1009667Z {"message":"missing documentation for a variant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2070,"byte_end":2120,"line_start":124,"line_end":127,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    PubBaz2A {","highlight_start":5,"highlight_end":15},{"text":"        /// dox","highlight_start":1,"highlight_end":16},{"text":"        a: isize,","highlight_start":1,"highlight_end":18},{"text":"    },","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a variant\n  --> tests/ui/missing-doc.rs:124:5\n   |\nLL | /     PubBaz2A {\nLL | |         /// dox\nLL | |         a: isize,\nLL | |     },\n   | |_____^\n\n"}
2019-12-14T21:21:34.1011224Z {"message":"missing documentation for a struct field","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2105,"byte_end":2113,"line_start":126,"line_end":126,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        a: isize,","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a struct field\n  --> tests/ui/missing-doc.rs:126:9\n   |\nLL |         a: isize,\n   |         ^^^^^^^^\n\n"}
2019-12-14T21:21:34.1012555Z {"message":"missing documentation for a constant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2254,"byte_end":2273,"line_start":138,"line_end":138,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"const FOO: u32 = 0;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a constant\n  --> tests/ui/missing-doc.rs:138:1\n   |\nLL | const FOO: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1013922Z {"message":"missing documentation for a constant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2282,"byte_end":2306,"line_start":140,"line_end":140,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"pub const FOO1: u32 = 0;","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a constant\n  --> tests/ui/missing-doc.rs:140:1\n   |\nLL | pub const FOO1: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1015514Z {"message":"missing documentation for a constant","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2420,"byte_end":2444,"line_start":145,"line_end":145,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"pub const FOO4: u32 = 0;","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a constant\n  --> tests/ui/missing-doc.rs:145:1\n   |\nLL | pub const FOO4: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1017122Z {"message":"missing documentation for a static","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2446,"byte_end":2466,"line_start":147,"line_end":147,"column_start":1,"column_end":21,"is_primary":true,"text":[{"text":"static BAR: u32 = 0;","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a static\n  --> tests/ui/missing-doc.rs:147:1\n   |\nLL | static BAR: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1018334Z {"message":"missing documentation for a static","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2475,"byte_end":2500,"line_start":149,"line_end":149,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"pub static BAR1: u32 = 0;","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a static\n  --> tests/ui/missing-doc.rs:149:1\n   |\nLL | pub static BAR1: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1019972Z {"message":"missing documentation for a static","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2616,"byte_end":2641,"line_start":154,"line_end":154,"column_start":1,"column_end":26,"is_primary":true,"text":[{"text":"pub static BAR4: u32 = 0;","highlight_start":1,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a static\n  --> tests/ui/missing-doc.rs:154:1\n   |\nLL | pub static BAR4: u32 = 0;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1022495Z {"message":"missing documentation for a module","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2643,"byte_end":2955,"line_start":156,"line_end":169,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"mod internal_impl {","highlight_start":1,"highlight_end":20},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    pub fn documented() {}","highlight_start":1,"highlight_end":27},{"text":"    pub fn undocumented1() {}","highlight_start":1,"highlight_end":30},{"text":"    pub fn undocumented2() {}","highlight_start":1,"highlight_end":30},{"text":"    fn undocumented3() {}","highlight_start":1,"highlight_end":26},{"text":"    /// dox","highlight_start":1,"highlight_end":12},{"text":"    pub mod globbed {","highlight_start":1,"highlight_end":22},{"text":"        /// dox","highlight_start":1,"highlight_end":16},{"text":"        pub fn also_documented() {}","highlight_start":1,"highlight_end":36},{"text":"        pub fn also_undocumented1() {}","highlight_start":1,"highlight_end":39},{"text":"        fn also_undocumented2() {}","highlight_start":1,"highlight_end":35},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a module\n  --> tests/ui/missing-doc.rs:156:1\n   |\nLL | / mod internal_impl {\nLL | |     /// dox\nLL | |     pub fn documented() {}\nLL | |     pub fn undocumented1() {}\n...  |\nLL | |     }\nLL | | }\n   | |_^\n\n"}
2019-12-14T21:21:34.1024658Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2679,"byte_end":2701,"line_start":158,"line_end":158,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    pub fn documented() {}","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:158:5\n   |\nLL |     pub fn documented() {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1025998Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2706,"byte_end":2731,"line_start":159,"line_end":159,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    pub fn undocumented1() {}","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:159:5\n   |\nLL |     pub fn undocumented1() {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1027454Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2736,"byte_end":2761,"line_start":160,"line_end":160,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    pub fn undocumented2() {}","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:160:5\n   |\nLL |     pub fn undocumented2() {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1029194Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2766,"byte_end":2787,"line_start":161,"line_end":161,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    fn undocumented3() {}","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:161:5\n   |\nLL |     fn undocumented3() {}\n   |     ^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1031349Z {"message":"missing documentation for a module","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2804,"byte_end":2953,"line_start":163,"line_end":168,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub mod globbed {","highlight_start":5,"highlight_end":22},{"text":"        /// dox","highlight_start":1,"highlight_end":16},{"text":"        pub fn also_documented() {}","highlight_start":1,"highlight_end":36},{"text":"        pub fn also_undocumented1() {}","highlight_start":1,"highlight_end":39},{"text":"        fn also_undocumented2() {}","highlight_start":1,"highlight_end":35},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a module\n  --> tests/ui/missing-doc.rs:163:5\n   |\nLL | /     pub mod globbed {\nLL | |         /// dox\nLL | |         pub fn also_documented() {}\nLL | |         pub fn also_undocumented1() {}\nLL | |         fn also_undocumented2() {}\nLL | |     }\n   | |_____^\n\n"}
2019-12-14T21:21:34.1032989Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2846,"byte_end":2873,"line_start":165,"line_end":165,"column_start":9,"column_end":36,"is_primary":true,"text":[{"text":"        pub fn also_documented() {}","highlight_start":9,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:165:9\n   |\nLL |         pub fn also_documented() {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1034364Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2882,"byte_end":2912,"line_start":166,"line_end":166,"column_start":9,"column_end":39,"is_primary":true,"text":[{"text":"        pub fn also_undocumented1() {}","highlight_start":9,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:166:9\n   |\nLL |         pub fn also_undocumented1() {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1035685Z {"message":"missing documentation for a function","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2921,"byte_end":2947,"line_start":167,"line_end":167,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"        fn also_undocumented2() {}","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a function\n  --> tests/ui/missing-doc.rs:167:9\n   |\nLL |         fn also_undocumented2() {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
2019-12-14T21:21:34.1038638Z {"message":"missing documentation for a module","code":{"code":"clippy::missing_docs_in_private_items","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing-doc.rs","byte_start":2964,"byte_end":3182,"line_start":171,"line_end":176,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub mod public_interface {","highlight_start":1,"highlight_end":27},{"text":"    pub use internal_impl::documented as foo;","highlight_start":1,"highlight_end":46},{"text":"    pub use internal_impl::globbed::*;","highlight_start":1,"highlight_end":39},{"text":"    pub use internal_impl::undocumented1 as bar;","highlight_start":1,"highlight_end":49},{"text":"    pub use internal_impl::{documented, undocumented2};","highlight_start":1,"highlight_end":56},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: missing documentation for a module\n  --> tests/ui/missing-doc.rs:171:1\n   |\nLL | / pub mod public_interface {\nLL | |     pub use internal_impl::documented as foo;\nLL | |     pub use internal_impl::globbed::*;\nLL | |     pub use internal_impl::undocumented1 as bar;\nLL | |     pub use internal_impl::{documented, undocumented2};\nLL | | }\n   | |_^\n\n"}
2019-12-14T21:21:34.1039405Z 
2019-12-14T21:21:34.1039672Z ------------------------------------------
2019-12-14T21:21:34.1039894Z 
2019-12-14T21:21:34.1040117Z test [ui] ui/missing-doc.rs ... FAILED
---
2019-12-14T22:01:53.9207676Z Verifying status of clippy-driver...
2019-12-14T22:01:53.9207768Z Verifying status of miri...
2019-12-14T22:01:53.9208012Z Verifying status of embedded-book...
2019-12-14T22:01:53.9208272Z Verifying status of rustc-guide...
2019-12-14T22:01:53.9208614Z error: Tool `clippy-driver` should be test-pass but is test-fail during beta week.
2019-12-14T22:01:53.9226364Z Build completed unsuccessfully in 0:00:01
2019-12-14T22:01:53.9264571Z == clock drift check ==
2019-12-14T22:01:53.9283961Z   local time: Sat Dec 14 22:01:53 UTC 2019
2019-12-14T22:01:54.4738231Z   network time: Sat, 14 Dec 2019 22:01:54 GMT
2019-12-14T22:01:54.4738231Z   network time: Sat, 14 Dec 2019 22:01:54 GMT
2019-12-14T22:01:54.4740236Z == end clock drift check ==
2019-12-14T22:01:54.9775997Z 
2019-12-14T22:01:54.9885895Z ##[error]Bash exited with code '1'.
2019-12-14T22:01:54.9929376Z ##[section]Starting: Checkout
2019-12-14T22:01:54.9932132Z ==============================================================================
2019-12-14T22:01:54.9932243Z Task         : Get sources
2019-12-14T22:01:54.9932348Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
