rust
struct OnPizza<const WHAT:&'static str>;
type NoooooooA=OnPizza<"🍍">;
type NoooooooB=OnPizza<"pineapple">;
