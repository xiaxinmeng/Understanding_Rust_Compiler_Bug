rust
struct Something;

trait MyTrait {}

impl MyTrait for (Something) {} // warning: unnecessary parentheses around type
