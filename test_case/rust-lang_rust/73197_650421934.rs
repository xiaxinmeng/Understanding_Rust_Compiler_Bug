rust
// For whatever reason, <Range<u32> as Default>::default() = 0..u32::MAX is a good default
impl Default for Range<u32> { fn default() -> Self { 0..u32::MAX } }

// Problem: There isn't a Range implementation for arbitrary types
#[derive(Default)]
struct FancyDecimalType;
#[derive(Default)]
struct UserTypeWithRange(Range<FancyDecimalType>); // error!

// Because: Generic implementation will conflict with <Range<u32> as Default>
impl<T: Default> Default for Range<T> { fn default() -> Self { Default::default()..Default::default() } } // error!
