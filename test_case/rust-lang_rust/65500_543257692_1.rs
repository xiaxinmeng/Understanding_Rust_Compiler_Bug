rust
use tiberius::transaction::Transaction;

async fn suivi_log(
    conn: Transaction<std::boxed::Box<dyn tiberius::BoxableIo>>,
) -> Transaction<std::boxed::Box<dyn tiberius::BoxableIo>> {
    conn
}

fn main() {}
