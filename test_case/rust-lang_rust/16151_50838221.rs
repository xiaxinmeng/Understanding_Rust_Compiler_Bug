 rust
struct Fragment;

impl Drop for Fragment {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn main() {
    let mut fragments = Vec::new();
    fragments.push(Fragment);
    fragments.push(Fragment);

    fragments.move_iter()
             .skip_while(|fragment| {
                 true
             }).collect::<Vec<Fragment>>();
}
