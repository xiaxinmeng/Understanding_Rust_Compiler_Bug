rust
// ok:

fn f1(a: &ExternType) {}
fn f2() -> *mut ExternType {}
fn f3<T: ?Sized>(a: &T, b: *const ExternType) {}
fn f4<A, R, F: Fn(A) -> R>(func: F, a: A) -> R {}
let w1 = size_of::<*const ExternType>();
let w2 = size_of_val(&f2());
let w3 = f4(|x| x, &*f2());
//^ substituting `&ExternType` is ok.

// errors (gated):

fn g1<T: ?Sized>(a: &T) {}
g1(&*f2()); //~ ERROR: Cannot substitute an extern type to `T`

fn g2() -> Box<ExternType> {} //~ ERROR: Cannot substitute an extern type to `T`

struct S1(ExternType);  //~ ERROR: Cannot use an extern type as struct field.

struct S2<R: ?Sized>(R);
let s2: S2<ExternType>; //~ ERROR: Cannot substitute an extern type to `R`

struct S3;
impl Deref for S3 {
    type Target = ExternType; //~ ERROR: Cannot use extern type as associated type.
    fn deref(&self) -> &ExternType {}
}
