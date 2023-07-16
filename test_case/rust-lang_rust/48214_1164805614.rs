rust
trait Trait {}

struct No;
struct Yes;
impl Trait for Yes {}

struct S1;
struct S2;

impl Trait for S1 where Yes: Trait {}
impl Trait for S2 where No: Trait {}
