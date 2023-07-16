
enum E { V }
type Alias = E;
impl E { const V: u8 = 0; }

// Variants have higher priority than other associated items
E::V - resolves to variant
Alias::V - resolves to variant
<E>::V - STILL resolves to variant in "associated item" interpretation
<Alias>::V - STILL resolves to variant in "associated item" interpretation
Drawback - there seems to be no way to refer to the inherent associated constant V
Drawback - this is a breaking change
Alternative - variants on non-aliases have higher priority and variants on aliases have lower priority (horrible)
Alternative - make "duplication" between variants and inherent items an error, similarly to how inherent item duplication is already an error

// Both variant "type" and variant constructors are associated items
Alias::V { .. } - valid, resolves to a variant type
Alias::V - valid, resolves to a variant constructor
Catch - we don't have inherent associated types yet and resolution for non-UFCS associated types is really bad

// Variants work at "type level", so all aliases are substituted, including Self and associated types
Type::AssociatedType::Variant - works
Self::Variant - works
Catch - some normalization can suddenly result in a variant type? (not sure)

// The type parameter question - still unclear
type Alias<T> = Option<T>;
Option::<u8>::None // Currently prohibited
Option::None::<u8> // Ok
Alias::<u8>::None // ?
Alias::None::<u8> // ?
Alias::<u8>::None::<u8> // ??
Alias::<u8>::None::<u16> // ???
