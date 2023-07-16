
struct Bump { ... }
impl Clone for Bump { ... }

struct Thing(Box<u32, Bump>);
