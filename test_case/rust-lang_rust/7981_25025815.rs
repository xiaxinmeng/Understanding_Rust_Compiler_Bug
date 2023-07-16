
struct A;           // allow
struct A{}          // forbid because ambiguity
impl A;             // allow
impl A{}            // allow
trait B;            // allow
trait B{}           // allow
impl B for A;       // allow
impl B for A{}      // allow
enum C;             // allow
enum C{}            // allow
extern;             // forbid, useless
extern{}            // forbid, useless
fn foo(){}          // allow
fn foo();           // forbid, ambiguous in traits
