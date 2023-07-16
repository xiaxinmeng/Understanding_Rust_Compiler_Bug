 rust
use std::path::Path;

struct Root<'a> {
    font_path: Box<AsRef<Path> + 'a>
}

fn main() {
    let path = Path::new("/");
    let root = Root {
        font_path: Box::new(&path)
    };

    let path_ref : &Path = root.font_path.as_ref();
}
