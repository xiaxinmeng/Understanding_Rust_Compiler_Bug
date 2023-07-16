
enum E { V } // <- this should be selected for `E::V` and `Alias::V`
type Alias = E;
impl E { const V: u8 = 0; } // <- this should be selected for `<E>::V` and `<Alias>::V`
