 rust
trait Bounds ... // Replaces the Range trait in my proposal. Probably a better choice anyways.

struct Range ... // [m, n)
struct RightRange ... // (m, n]
struct InclusiveRange ... // [m, n]
struct ExclusiveRange ... // (m, n)

struct RangeFrom ... // [m,infinity)
struct ExclusiveRangeFrom ... // (m, infinity)

struct RangeTo ... // (-infinity, n) //
struct InclusiveRangeTo ... // (-infinity, n]

struct FullRange ... // (-infinity, infinity)
