
$ grep -R "pub trait .*Rhs" src/
src/libcore/cmp.rs:pub trait PartialEq<Rhs: ?Sized = Self> {
src/libcore/cmp.rs:pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
src/libcore/ops/arith.rs:pub trait AddAssign<Rhs=Self> {
src/libcore/ops/arith.rs:pub trait SubAssign<Rhs=Self> {
src/libcore/ops/arith.rs:pub trait MulAssign<Rhs=Self> {
src/libcore/ops/arith.rs:pub trait DivAssign<Rhs=Self> {
src/libcore/ops/arith.rs:pub trait RemAssign<Rhs=Self> {
src/libcore/ops/bit.rs:pub trait BitAndAssign<Rhs=Self> {
src/libcore/ops/bit.rs:pub trait BitOrAssign<Rhs=Self> {
src/libcore/ops/bit.rs:pub trait BitXorAssign<Rhs=Self> {
src/libcore/ops/bit.rs:pub trait ShlAssign<Rhs> {
src/libcore/ops/bit.rs:pub trait ShrAssign<Rhs=Self> {
