plain

---- [ui] src/test/ui/privacy/access_levels.rs stdout ----
diff of stderr:

- error: access lvl: None, Public vis: pub(access_levels), Exported vis: pub(access_levels), Reachable vis: pub(access_levels), ReachableFromImplTrait vis: Invisible
+ error: access lvl: None, Public vis: pub(access_levels), Exported vis: pub(access_levels), Reachable vis: pub(access_levels), ReachableFromImplTrait vis: pub(access_levels)
3    |
3    |
4 LL | mod outer {
5    | ^^^^^^^^^
6 
6 
- error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(access_levels)
9    |
9    |
10 LL |     pub mod inner1 {
11    |     ^^^^^^^^^^^^^^
12 
12 
- error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Exported), Public vis: pub(inner1), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(inner1)
15    |
16 LL | ...   extern "C" {}

17    |       ^^^^^^^^^^^^^
17    |       ^^^^^^^^^^^^^
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
18 
- error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Exported), Public vis: pub(inner1), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(inner1)
21    |
22 LL | ...   pub trait PubTrait {

23    |       ^^^^^^^^^^^^^^^^^^
23    |       ^^^^^^^^^^^^^^^^^^
24 
- error: access lvl: None, Public vis: pub(inner1), Exported vis: pub(inner1), Reachable vis: pub(inner1), ReachableFromImplTrait vis: Invisible
+ error: access lvl: None, Public vis: pub(inner1), Exported vis: pub(inner1), Reachable vis: pub(inner1), ReachableFromImplTrait vis: pub(inner1)
27    |
27    |
28 LL | ...   struct PrivStruct;
29    |       ^^^^^^^^^^^^^^^^^
30 
30 
- error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Exported), Public vis: pub(inner1), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(inner1)
33    |
33    |
34 LL | ...   pub union PubUnion {
35    |       ^^^^^^^^^^^^^^^^^^
36 
36 
- error: access lvl: None, Public vis: pub(inner1), Exported vis: pub(inner1), Reachable vis: pub(inner1), ReachableFromImplTrait vis: Invisible
+ error: access lvl: None, Public vis: pub(PubUnion), Exported vis: pub(PubUnion), Reachable vis: pub(PubUnion), ReachableFromImplTrait vis: pub(PubUnion)
39    |
39    |
40 LL | ...   a: u8,
41    |       ^^^^^
42 
42 
- error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Exported), Public vis: pub(PubUnion), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(PubUnion)
45    |
45    |
46 LL | ...   pub b: u8,
47    |       ^^^^^^^^^
48 
48 
- error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Exported), Public vis: pub(inner1), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(inner1)
51    |
52 LL | ...   pub enum Enum {

53    |       ^^^^^^^^^^^^^
53    |       ^^^^^^^^^^^^^
54 
- error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Exported), Public vis: pub(Enum), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(Enum)
57    |
58 LL | ...   A(

59    |       ^
59    |       ^
60 
- error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Exported), Public vis: pub(A), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(A)
63    |
64 LL | ...   PubUnion,

65    |       ^^^^^^^^
65    |       ^^^^^^^^
66 
- error: access lvl: None, Public vis: pub(access_levels), Exported vis: pub(access_levels), Reachable vis: pub(access_levels), ReachableFromImplTrait vis: Invisible
+ error: access lvl: None, Public vis: pub(outer), Exported vis: pub(outer), Reachable vis: pub(outer), ReachableFromImplTrait vis: pub(outer)
69    |
70 LL |     macro_rules! none_macro {

71    |     ^^^^^^^^^^^^^^^^^^^^^^^
71    |     ^^^^^^^^^^^^^^^^^^^^^^^
72 
- error: access lvl: Some(Public), Public vis: Public, Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Public), Public vis: Public, Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(access_levels)
75    |
76 LL |     macro_rules! public_macro {

77    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
77    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
78 
- error: access lvl: Some(Reachable), Public vis: pub(access_levels), Exported vis: pub(access_levels), Reachable vis: pub(access_levels), ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Reachable), Public vis: pub(outer), Exported vis: pub(outer), Reachable vis: pub(outer), ReachableFromImplTrait vis: pub(outer)
81    |
82 LL |     pub struct ReachableStruct {

83    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
83    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
84 
- error: access lvl: Some(Reachable), Public vis: pub(access_levels), Exported vis: pub(access_levels), Reachable vis: pub(access_levels), ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Reachable), Public vis: pub(ReachableStruct), Exported vis: pub(ReachableStruct), Reachable vis: pub(ReachableStruct), ReachableFromImplTrait vis: pub(ReachableStruct)
87    |
88 LL | ...   pub a: u8,

89    |       ^^^^^^^^^
89    |       ^^^^^^^^^
90 
- error: access lvl: None, Public vis: pub(outer), Exported vis: pub(outer), Reachable vis: pub(outer), ReachableFromImplTrait vis: Invisible
+ error: access lvl: None, Public vis: pub(outer), Exported vis: pub(outer), Reachable vis: pub(outer), ReachableFromImplTrait vis: pub(outer)
93    |
93    |
94 LL |     mod priv_mod {
95    |     ^^^^^^^^^^^^
96 
96 
- error: access lvl: None, Public vis: pub(outer), Exported vis: pub(outer), Reachable vis: pub(outer), ReachableFromImplTrait vis: Invisible
+ error: access lvl: None, Public vis: pub(priv_mod), Exported vis: pub(priv_mod), Reachable vis: pub(priv_mod), ReachableFromImplTrait vis: pub(priv_mod)
99    |
99    |
100 LL | ...   pub mod pub_mod {
101    |       ^^^^^^^^^^^^^^^
102 
102 
- error: access lvl: None, Public vis: pub(outer), Exported vis: pub(priv_mod), Reachable vis: pub(priv_mod), ReachableFromImplTrait vis: Invisible
+ error: access lvl: None, Public vis: pub(priv_mod), Exported vis: pub(priv_mod), Reachable vis: pub(priv_mod), ReachableFromImplTrait vis: pub(priv_mod)
105    |
105    |
106 LL | ...   pub struct PubStruct2;
107    |       ^^^^^^^^^^^^^^^^^^^^^
108 
108 
- error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Exported), Public vis: pub(PubTrait), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(PubTrait)
111    |
111    |
112 LL | ...   const A: i32;
113    |       ^^^^^^^^^^^^
114 
114 
- error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: Invisible
+ error: access lvl: Some(Exported), Public vis: pub(PubTrait), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(PubTrait)
117    |
118 LL | ...   type B;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/access_levels/access_levels.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/access_levels.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/access_levels.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/access_levels" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/access_levels/auxiliary"
stdout: none
--- stderr -------------------------------
error: access lvl: None, Public vis: pub(access_levels), Exported vis: pub(access_levels), Reachable vis: pub(access_levels), ReachableFromImplTrait vis: pub(access_levels)
   |
   |
LL | mod outer { //~ ERROR access lvl: None, Public vis: pub(access_levels), Exported vis: pub(access_levels), Reachable vis: pub(access_level...


error: access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(access_levels)
   |
   |
LL |     pub mod inner1 { //~ ERROR access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, R...


error: access lvl: Some(Exported), Public vis: pub(inner1), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(inner1)
   |
   |
LL | ...   extern "C" {} //~ ERROR Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFromI...


error: access lvl: Some(Exported), Public vis: pub(inner1), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(inner1)
   |
   |
LL | ...   pub trait PubTrait { //~ ERROR access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Pub...


error: access lvl: None, Public vis: pub(inner1), Exported vis: pub(inner1), Reachable vis: pub(inner1), ReachableFromImplTrait vis: pub(inner1)
   |
   |
LL | ...   struct PrivStruct; //~ ERROR access lvl: None, Public vis: pub(inner1), Exported vis: pub(inner1), Reachable vis: pub(inner1), Reac...


error: access lvl: Some(Exported), Public vis: pub(inner1), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(inner1)
   |
   |
LL | ...   pub union PubUnion { //~ ERROR access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Pub...


error: access lvl: None, Public vis: pub(PubUnion), Exported vis: pub(PubUnion), Reachable vis: pub(PubUnion), ReachableFromImplTrait vis: pub(PubUnion)
   |
   |
LL | ...   a: u8, //~ ERROR access lvl: None, Public vis: pub(inner1), Exported vis: pub(inner1), Reachable vis: pub(inner1), ReachableFromImp...


error: access lvl: Some(Exported), Public vis: pub(PubUnion), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(PubUnion)
   |
   |
LL | ...   pub b: u8, //~ ERROR access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, Reach...


error: access lvl: Some(Exported), Public vis: pub(inner1), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(inner1)
   |
   |
LL | ...   pub enum Enum { //~ ERROR access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ...


error: access lvl: Some(Exported), Public vis: pub(Enum), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(Enum)
   |
   |
LL | ...   A( //~ ERROR access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, ReachableFrom...


error: access lvl: Some(Exported), Public vis: pub(A), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(A)
   |
   |
LL | ...   PubUnion,  //~ ERROR access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, Reach...


error: access lvl: None, Public vis: pub(outer), Exported vis: pub(outer), Reachable vis: pub(outer), ReachableFromImplTrait vis: pub(outer)
   |
   |
LL |     macro_rules! none_macro { //~ access lvl: None, Public vis: pub(access_levels), Exported vis: pub(access_levels), Reachable vis: pub(...


error: access lvl: Some(Public), Public vis: Public, Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(access_levels)
   |
   |
LL |     macro_rules! public_macro { //~ access lvl: Some(Public), Public vis: Public, Exported vis: Public, Reachable vis: Public, ReachableF...


error: access lvl: Some(Reachable), Public vis: pub(outer), Exported vis: pub(outer), Reachable vis: pub(outer), ReachableFromImplTrait vis: pub(outer)
   |
   |
LL |     pub struct ReachableStruct { //~ ERROR access lvl: Some(Reachable), Public vis: pub(access_levels), Exported vis: pub(access_levels),...


error: access lvl: Some(Reachable), Public vis: pub(ReachableStruct), Exported vis: pub(ReachableStruct), Reachable vis: pub(ReachableStruct), ReachableFromImplTrait vis: pub(ReachableStruct)
   |
   |
LL | ...   pub a: u8, //~ ERROR access lvl: Some(Reachable), Public vis: pub(access_levels), Exported vis: pub(access_levels), Reachable vis: ...


error: access lvl: None, Public vis: pub(outer), Exported vis: pub(outer), Reachable vis: pub(outer), ReachableFromImplTrait vis: pub(outer)
   |
   |
LL |     mod priv_mod { //~ ERROR access lvl: None, Public vis: pub(outer), Exported vis: pub(outer), Reachable vis: pub(outer), ReachableFrom...


error: access lvl: None, Public vis: pub(priv_mod), Exported vis: pub(priv_mod), Reachable vis: pub(priv_mod), ReachableFromImplTrait vis: pub(priv_mod)
   |
   |
LL | ...   pub mod pub_mod {//~ ERROR access lvl: None, Public vis: pub(outer), Exported vis: pub(outer), Reachable vis: pub(outer), Reachable...


error: access lvl: None, Public vis: pub(priv_mod), Exported vis: pub(priv_mod), Reachable vis: pub(priv_mod), ReachableFromImplTrait vis: pub(priv_mod)
   |
   |
LL | ...   pub struct PubStruct2; //~ ERROR access lvl: None, Public vis: pub(outer), Exported vis: pub(priv_mod), Reachable vis: pub(priv_mod...


error: access lvl: Some(Exported), Public vis: pub(PubTrait), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(PubTrait)
   |
   |
LL | ...   const A: i32; //~ ERROR access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, Re...


error: access lvl: Some(Exported), Public vis: pub(PubTrait), Exported vis: Public, Reachable vis: Public, ReachableFromImplTrait vis: pub(PubTrait)
   |
   |
LL | ...   type B; //~ ERROR access lvl: Some(Exported), Public vis: pub(access_levels), Exported vis: Public, Reachable vis: Public, Reachabl...

error: aborting due to 20 previous errors
------------------------------------------

