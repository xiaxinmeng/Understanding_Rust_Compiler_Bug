rust
use std::panic::RefUnwindSafe;

trait Database { type Storage; }
trait HasQueryGroup { }
trait Query<DB> { type Data; }
trait SourceDatabase { fn parse(&self) { loop { } } }

struct ParseQuery;
struct RootDatabase { _runtime: Runtime<RootDatabase>, }
struct Runtime<DB: Database> { _storage: Box<DB::Storage> }
struct SalsaStorage { _parse: <ParseQuery as Query<RootDatabase>>::Data, }

impl Database for RootDatabase  { type Storage = SalsaStorage; }
impl HasQueryGroup for RootDatabase {}
impl<DB> Query<DB> for ParseQuery where DB: SourceDatabase, DB: Database { type Data = RootDatabase; }
impl<T> SourceDatabase for T where T: RefUnwindSafe, T: HasQueryGroup {}

pub(crate) fn goto_implementation(db: &RootDatabase) -> u32 { db.parse(); loop { } }

fn main() { }
