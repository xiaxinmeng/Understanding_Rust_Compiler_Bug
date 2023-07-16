
extern crate rodio;

use std::io::fs::walk_dir;
use std::io::BufReader;

fn main() {
    let directory = Path::new(".");
    let endpoint = rodio::get_default_endpoint().unwrap();

    for path in walk_dir(&directory).unwrap() {
        let file = std::fs::File::open(path.display()).unwrap();
        let music = rodio::play_once(&endpoint, BufReader::new(file));      
    }
}
