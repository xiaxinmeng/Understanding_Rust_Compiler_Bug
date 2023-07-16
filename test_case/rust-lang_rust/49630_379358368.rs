
$ grep -Ri "pub trait .*Rhs" src/
src/doc/book/first-edition/src/operators-and-overloading.md:pub trait Add<RHS = Self> {
src/doc/rust-by-example/src/generics/phantom/testcase_units.md:pub trait Add<RHS = Self> {
src/libcore/cmp.rs:pub trait PartialEq<Rhs: ?Sized = Self> {
src/libcore/cmp.rs:pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
src/libcore/ops/arith.rs:pub trait Add<RHS=Self> {
src/libcore/ops/arith.rs:pub trait Sub<RHS=Self> {
src/libcore/ops/arith.rs:pub trait Mul<RHS=Self> {
src/libcore/ops/arith.rs:pub trait Div<RHS=Self> {
src/libcore/ops/arith.rs:pub trait Rem<RHS=Self> {
src/libcore/ops/arith.rs:pub trait AddAssign<Rhs=Self> {
src/libcore/ops/arith.rs:pub trait SubAssign<Rhs=Self> {
src/libcore/ops/arith.rs:pub trait MulAssign<Rhs=Self> {
src/libcore/ops/arith.rs:pub trait DivAssign<Rhs=Self> {
src/libcore/ops/arith.rs:pub trait RemAssign<Rhs=Self> {
src/libcore/ops/bit.rs:pub trait BitAnd<RHS=Self> {
src/libcore/ops/bit.rs:pub trait BitOr<RHS=Self> {
src/libcore/ops/bit.rs:pub trait BitXor<RHS=Self> {
src/libcore/ops/bit.rs:pub trait Shl<RHS=Self> {
src/libcore/ops/bit.rs:pub trait Shr<RHS=Self> {
src/libcore/ops/bit.rs:pub trait BitAndAssign<Rhs=Self> {
src/libcore/ops/bit.rs:pub trait BitOrAssign<Rhs=Self> {
src/libcore/ops/bit.rs:pub trait BitXorAssign<Rhs=Self> {
src/libcore/ops/bit.rs:pub trait ShlAssign<Rhs=Self> {
src/libcore/ops/bit.rs:pub trait ShrAssign<Rhs=Self> {
src/liblibc/src/dox.rs:    pub trait Div<RHS=Self> {
src/liblibc/src/dox.rs:    pub trait Shl<RHS=Self> {
src/liblibc/src/dox.rs:    pub trait Mul<RHS=Self> {
src/liblibc/src/dox.rs:    pub trait Sub<RHS=Self> {
src/liblibc/src/dox.rs:    pub trait Bitor<RHS=Self> {
src/liblibc/src/dox.rs:    pub trait Add<RHS = Self> {
src/test/compile-fail/issue-28576.rs:pub trait Foo<RHS=Self> {
src/test/run-pass/trait-inheritance-subst.rs:pub trait Add<RHS,Result> {
src/test/rustdoc/auxiliary/issue-20727.rs:pub trait Add<RHS = Self> {
src/test/rustdoc/issue-20727-2.rs:pub trait Add<RHS = Self> {
