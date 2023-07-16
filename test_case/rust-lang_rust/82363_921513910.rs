rust
// no reexport
pub trait IntegerPrimitive: Sized + Copy + Ord + Hash + Default + BitOr<Output=Self> + Debug + Display + Binary + Octal + LowerHex + UpperHex {}

#[unstable(...)] 
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_nonnull_optimization_guaranteed]
pub struct NonZero<T: IntegerPrimitive>(T);

impl<T: IntegerPrimitive> NonZero<T> {
    #[stable(..)]
    pub const unsafe fn new_unchecked(n: T) -> Self { unsafe { Self(n) } }
    pub const fn new(n: T) -> Option<Self> { (n != T::default()).then(|| unsafe { Self(n) }) }
    pub const fn get(self) -> T { self.0 }
}

#[stable(..)]
impl<T: IntegerPrimitive> BitOr for NonZero<T> {}

//... etc ...

#[stable(..)]
pub type NonZeroU8 = NonZero<u8>;
pub type NonZeroU16 = NonZero<u16>;
...
pub type NonZeroI128 = NonZero<i128>;
pub type NonZeroIsize = NonZero<isize>;
