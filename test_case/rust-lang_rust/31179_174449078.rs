
type Alias<T> = Option<T>;

Option::<u8>::None // Currently prohibited
Option::None::<u8> // Ok
Alias::<u8>::None // ?
Alias::None::<u8> // ?
Alias::<u8>::None::<u8> // ??
Alias::<u8>::None::<u16> // ???
