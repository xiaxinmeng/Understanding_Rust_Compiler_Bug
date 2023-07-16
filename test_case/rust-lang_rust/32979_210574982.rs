 rust
// sdar.rs
use std::str::FromStr;

pub struct TrainingIterResult {
    _a: (),
}

pub fn do_training_iters() -> Vec<TrainingIterResult> {
    Vec::new()
}

pub struct MolsFeaturesTable(());

impl MolsFeaturesTable {
    fn new(mol_ids: Vec<String>) -> MolsFeaturesTable {
        if mol_ids.len() != 3 {
            panic!()
        }

        MolsFeaturesTable(())
    }
}

pub fn read_with_activities()
    -> Result<(Vec<f32>, MolsFeaturesTable), MolsFeaturesTableError>
{
    let _ = f32::from_str("32");
    for occ_str in " 3".split(' ') {
        try!(f32::from_str(occ_str).map_err(|_| parse_err(format!(""))));
    }

    Ok((Vec::new(), MolsFeaturesTable::new(Vec::new())))
}

fn parse_err(s: String) -> MolsFeaturesTableError {
    MolsFeaturesTableError::Parse(s)
}

pub enum MolsFeaturesTableError {
    _Foo(Box<u32>),
    Parse(String),
}

// foo.rs
extern crate sdar;

fn main() {
    let _ = sdar::read_with_activities();
    sdar::do_training_iters();
}
