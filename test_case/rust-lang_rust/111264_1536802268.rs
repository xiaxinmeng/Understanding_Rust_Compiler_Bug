rust
#![feature(
    no_core,
    lang_items,
    intrinsics,
    unboxed_closures,
    type_ascription,
    extern_types,
    decl_macro,
    rustc_attrs,
    transparent_unions,
    auto_traits,
    thread_local,
    start,
    associated_type_bounds,
    arbitrary_self_types
)]
#![no_core]
#![allow(dead_code)]

struct Map<Fut, F> {
    future: Option<Fut>,
    f: Option<F>,
}

impl<Fut, F> Map<Fut, F> {
    pub(crate) fn new(future: Fut, f: F) -> Self {
        Self { future: Some(future), f: Some(f) }
    }
}

impl<Fut: Future, F: FnOnce(Fut::Output) -> U, U> Future for Map<Fut, F> {
    type Output = U;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Poll::Pending
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut cx = Context;
    let mut fut = pin!(fut);

    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(value) => break value,
            Poll::Pending => {}
        }
    }
}

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    let future = async { 1u8 };
    let future = future.map(|x| x + 3u8);
    if block_on(future) == 4u8 { 0 } else { 1 }
}

// ---------- CORE ---------- //

#[no_mangle]
unsafe extern "C" fn _Unwind_Resume() {
    intrinsics::unreachable();
}

#[lang = "sized"]
pub trait Sized {}

#[lang = "destruct"]
pub trait Destruct {}

#[lang = "tuple_trait"]
pub trait Tuple {}

#[lang = "unsize"]
pub trait Unsize<T: ?Sized> {}

#[lang = "coerce_unsized"]
pub trait CoerceUnsized<T> {}

impl<'a, 'b: 'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b T {}
impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<&'a mut U> for &'a mut T {}
impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *const T {}
impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for *mut T {}

#[lang = "dispatch_from_dyn"]
pub trait DispatchFromDyn<T> {}

// &T -> &U
impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<&'a U> for &'a T {}
// &mut T -> &mut U
impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<&'a mut U> for &'a mut T {}
// *const T -> *const U
impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<*const U> for *const T {}
// *mut T -> *mut U
impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<*mut U> for *mut T {}
impl<T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Box<U, ()>> for Box<T, ()> {}

#[lang = "receiver"]
pub trait Receiver {}

impl<T: ?Sized> Receiver for &T {}
impl<T: ?Sized> Receiver for &mut T {}
impl<T: ?Sized, A: Allocator> Receiver for Box<T, A> {}
impl<P: Receiver> Receiver for Pin<P> {}

#[lang = "copy"]
pub unsafe trait Copy {}

unsafe impl Copy for bool {}
unsafe impl Copy for u8 {}
unsafe impl Copy for u16 {}
unsafe impl Copy for u32 {}
unsafe impl Copy for u64 {}
unsafe impl Copy for usize {}
unsafe impl Copy for i8 {}
unsafe impl Copy for i16 {}
unsafe impl Copy for i32 {}
unsafe impl Copy for isize {}
unsafe impl Copy for f32 {}
unsafe impl Copy for f64 {}
unsafe impl Copy for char {}
unsafe impl<'a, T: ?Sized> Copy for &'a T {}
unsafe impl<T: ?Sized> Copy for *const T {}
unsafe impl<T: ?Sized> Copy for *mut T {}

#[lang = "sync"]
pub unsafe trait Sync {}

unsafe impl Sync for bool {}
unsafe impl Sync for u8 {}
unsafe impl Sync for u16 {}
unsafe impl Sync for u32 {}
unsafe impl Sync for u64 {}
unsafe impl Sync for usize {}
unsafe impl Sync for i8 {}
unsafe impl Sync for i16 {}
unsafe impl Sync for i32 {}
unsafe impl Sync for isize {}
unsafe impl Sync for char {}
unsafe impl<'a, T: ?Sized> Sync for &'a T {}
unsafe impl Sync for [u8; 16] {}

#[lang = "freeze"]
unsafe auto trait Freeze {}

unsafe impl<T: ?Sized> Freeze for PhantomData<T> {}
unsafe impl<T: ?Sized> Freeze for *const T {}
unsafe impl<T: ?Sized> Freeze for *mut T {}
unsafe impl<T: ?Sized> Freeze for &T {}
unsafe impl<T: ?Sized> Freeze for &mut T {}

#[lang = "structural_peq"]
pub trait StructuralPartialEq {}

#[lang = "structural_teq"]
pub trait StructuralEq {}

#[lang = "not"]
pub trait Not {
    type Output;

    fn not(self) -> Self::Output;
}

impl Not for bool {
    type Output = bool;

    fn not(self) -> bool {
        !self
    }
}

#[lang = "mul"]
pub trait Mul<RHS = Self> {
    type Output;

    #[must_use]
    fn mul(self, rhs: RHS) -> Self::Output;
}

impl Mul for u8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl Mul for usize {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

#[lang = "add"]
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for u8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Add for i8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Add for usize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

#[lang = "sub"]
pub trait Sub<RHS = Self> {
    type Output;

    fn sub(self, rhs: RHS) -> Self::Output;
}

impl Sub for usize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for u8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for i8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for i16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

#[lang = "rem"]
pub trait Rem<RHS = Self> {
    type Output;

    fn rem(self, rhs: RHS) -> Self::Output;
}

impl Rem for usize {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self % rhs
    }
}

#[lang = "bitor"]
pub trait BitOr<RHS = Self> {
    type Output;

    #[must_use]
    fn bitor(self, rhs: RHS) -> Self::Output;
}

impl BitOr for bool {
    type Output = bool;

    fn bitor(self, rhs: bool) -> bool {
        self | rhs
    }
}

impl<'a> BitOr<bool> for &'a bool {
    type Output = bool;

    fn bitor(self, rhs: bool) -> bool {
        *self | rhs
    }
}

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool;
}

impl PartialEq for u8 {
    fn eq(&self, other: &u8) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u8) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for u16 {
    fn eq(&self, other: &u16) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u16) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for u32 {
    fn eq(&self, other: &u32) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u32) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for u64 {
    fn eq(&self, other: &u64) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u64) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for usize {
    fn eq(&self, other: &usize) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &usize) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for i8 {
    fn eq(&self, other: &i8) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &i8) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for i32 {
    fn eq(&self, other: &i32) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &i32) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for isize {
    fn eq(&self, other: &isize) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &isize) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for char {
    fn eq(&self, other: &char) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &char) -> bool {
        (*self) != (*other)
    }
}

impl<T: ?Sized> PartialEq for *const T {
    fn eq(&self, other: &*const T) -> bool {
        *self == *other
    }
    fn ne(&self, other: &*const T) -> bool {
        *self != *other
    }
}

#[lang = "neg"]
pub trait Neg {
    type Output;

    fn neg(self) -> Self::Output;
}

impl Neg for i8 {
    type Output = i8;

    fn neg(self) -> i8 {
        -self
    }
}

impl Neg for i16 {
    type Output = i16;

    fn neg(self) -> i16 {
        self
    }
}

impl Neg for isize {
    type Output = isize;

    fn neg(self) -> isize {
        -self
    }
}

impl Neg for f32 {
    type Output = f32;

    fn neg(self) -> f32 {
        -self
    }
}

pub enum Option<T> {
    Some(T),
    None,
}

pub use Option::*;

#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;

#[lang = "fn_once"]
#[rustc_paren_sugar]
pub trait FnOnce<Args: Tuple> {
    #[lang = "fn_once_output"]
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

#[lang = "fn_mut"]
#[rustc_paren_sugar]
pub trait FnMut<Args: Tuple>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

#[lang = "panic"]
#[track_caller]
pub fn panic(_msg: &'static str) -> ! {
    unsafe {
        libc::puts("Panicking\n\0" as *const str as *const u8);
        intrinsics::abort();
    }
}

#[lang = "panic_cannot_unwind"]
fn panic_cannot_unwind() -> ! {
    unsafe {
        libc::puts("Panicking\n\0" as *const str as *const u8);
        intrinsics::abort();
    }
}

#[lang = "panic_bounds_check"]
#[track_caller]
fn panic_bounds_check(index: usize, len: usize) -> ! {
    unsafe {
        libc::printf(
            "index out of bounds: the len is %d but the index is %d\n\0" as *const str as *const i8,
            len,
            index,
        );
        intrinsics::abort();
    }
}

#[lang = "eh_personality"]
fn eh_personality() -> ! {
    loop {}
}

#[lang = "drop_in_place"]
#[allow(unconditional_recursion)]
pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    // Code here does not matter - this is replaced by the
    // real drop glue by the compiler.
    drop_in_place(to_drop);
}

#[lang = "deref"]
pub trait Deref {
    #[lang = "deref_target"]
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

pub trait Allocator {}

impl Allocator for () {}

pub struct Global;

impl Allocator for Global {}

#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_nonnull_optimization_guaranteed]
pub struct NonNull<T: ?Sized>(pub *const T);

impl<T: ?Sized, U: ?Sized> CoerceUnsized<NonNull<U>> for NonNull<T> where T: Unsize<U> {}
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<NonNull<U>> for NonNull<T> where T: Unsize<U> {}

pub struct Unique<T: ?Sized> {
    pub pointer: NonNull<T>,
    pub _marker: PhantomData<T>,
}

impl<T: ?Sized, U: ?Sized> CoerceUnsized<Unique<U>> for Unique<T> where T: Unsize<U> {}
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<Unique<U>> for Unique<T> where T: Unsize<U> {}

#[lang = "owned_box"]
pub struct Box<T: ?Sized, A: Allocator = Global>(Unique<T>, A);

impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> CoerceUnsized<Box<U, A>> for Box<T, A> {}

impl<T: ?Sized, A: Allocator> Drop for Box<T, A> {
    fn drop(&mut self) {
        // drop is currently performed by compiler.
    }
}

impl<T: ?Sized, A: Allocator> Deref for Box<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &**self
    }
}

#[lang = "exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
    libc::malloc(size)
}

#[lang = "box_free"]
unsafe fn box_free<T: ?Sized>(ptr: Unique<T>, _alloc: ()) {
    libc::free(ptr.pointer.0 as *mut u8);
}

#[lang = "drop"]
pub trait Drop {
    fn drop(&mut self);
}

#[lang = "manually_drop"]
#[repr(transparent)]
pub struct ManuallyDrop<T: ?Sized> {
    pub value: T,
}

#[lang = "maybe_uninit"]
#[repr(transparent)]
pub union MaybeUninit<T> {
    pub uninit: (),
    pub value: ManuallyDrop<T>,
}

pub mod intrinsics {
    use crate::Sized;

    extern "rust-intrinsic" {
        #[rustc_safe_intrinsic]
        pub fn abort() -> !;
        #[rustc_safe_intrinsic]
        pub fn size_of<T>() -> usize;
        pub fn size_of_val<T: ?Sized>(val: *const T) -> usize;
        #[rustc_safe_intrinsic]
        pub fn min_align_of<T>() -> usize;
        pub fn min_align_of_val<T: ?Sized>(val: *const T) -> usize;
        pub fn copy<T>(src: *const T, dst: *mut T, count: usize);
        pub fn transmute<T, U>(e: T) -> U;
        pub fn ctlz_nonzero<T>(x: T) -> T;
        #[rustc_safe_intrinsic]
        pub fn needs_drop<T: ?Sized>() -> bool;
        #[rustc_safe_intrinsic]
        pub fn bitreverse<T>(x: T) -> T;
        #[rustc_safe_intrinsic]
        pub fn bswap<T>(x: T) -> T;
        pub fn write_bytes<T>(dst: *mut T, val: u8, count: usize);
        pub fn unreachable() -> !;
    }
}

pub mod libc {
    #[link(name = "c")]
    extern "C" {
        pub fn puts(s: *const u8) -> i32;
        pub fn printf(format: *const i8, ...) -> i32;
        pub fn malloc(size: usize) -> *mut u8;
        pub fn free(ptr: *mut u8);
        pub fn memcpy(dst: *mut u8, src: *const u8, size: usize);
        pub fn memmove(dst: *mut u8, src: *const u8, size: usize);
        pub fn strncpy(dst: *mut u8, src: *const u8, size: usize);
    }
}

#[lang = "index"]
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}

impl<T> Index<usize> for [T; 3] {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index]
    }
}

impl<T> Index<usize> for [T] {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index]
    }
}

extern "C" {
    type VaListImpl;
}

#[lang = "va_list"]
#[repr(transparent)]
pub struct VaList<'a>(&'a mut VaListImpl);

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro stringify($($t:tt)*) {
    /* compiler built-in */
}

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro file() {
    /* compiler built-in */
}

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro line() {
    /* compiler built-in */
}

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro cfg() {
    /* compiler built-in */
}

pub static A_STATIC: u8 = 42;

#[lang = "panic_location"]
struct PanicLocation {
    file: &'static str,
    line: u32,
    column: u32,
}

#[no_mangle]
pub fn get_tls() -> u8 {
    #[thread_local]
    static A: u8 = 42;

    A
}

macro pin($value:expr $(,)?) {
    Pin::<&mut _> { pointer: &mut { $value } }
}

#[lang = "pin"]
struct Pin<P> {
    pointer: P,
}

#[lang = "unpin"]
pub auto trait Unpin {}

use Option::*;

#[lang = "Poll"]
enum Poll<T> {
    #[lang = "Ready"]
    Ready(T),
    #[lang = "Pending"]
    Pending,
}

#[lang = "future_trait"]
trait Future {
    type Output;

    #[lang = "poll"]
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;

    fn map<U, F>(self, f: F) -> Map<Self, F>
    where
        F: FnOnce(Self::Output) -> U,
        Self: Sized,
    {
        Map::new(self, f)
    }
}

#[lang = "Context"]
struct Context;

impl<T: ?Sized> Deref for &T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}

impl<T: ?Sized> Deref for &mut T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}

impl<T: ?Sized> DerefMut for &mut T {
    fn deref_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<P: Deref> Deref for Pin<P> {
    type Target = P::Target;
    fn deref(&self) -> &P::Target {
        &*self.pointer
    }
}

#[lang = "get_context"]
unsafe fn get_context<'a>(cx: ResumeTy) -> &'a mut Context {
    &mut *(cx.0 as *mut Context)
}

#[lang = "deref_mut"]
pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}

impl<P: DerefMut<Target: Unpin>> DerefMut for Pin<P> {
    fn deref_mut(&mut self) -> &mut P::Target {
        &mut *self.pointer
    }
}

impl<P: DerefMut> Pin<P> {
    fn as_mut(&mut self) -> Pin<&mut P::Target> {
        unsafe { Pin { pointer: &mut *self.pointer } }
    }
}

#[lang = "ResumeTy"]
struct ResumeTy(*const ());
