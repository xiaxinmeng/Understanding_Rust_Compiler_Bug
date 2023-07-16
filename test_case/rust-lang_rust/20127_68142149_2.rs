
extern crate cellular_maps;

use cellular_maps::CellularMap;

fn print_map(map: &CellularMap) {
    for c in range(0u,(map.get_width())) {
        println!("{} {}",c);
    }
}

fn main() {
    let cm = CellularMap::new(12u,12u,10u);
    print_map(&cm);
}
