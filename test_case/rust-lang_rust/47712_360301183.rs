rust
#[cfg(debug_assertions)]
mod imports {
    use std::slice;
    use std::fs::File;    
    use std::io::prelude::*;    
    use std::convert::AsRef;    
    use std::path::Path;
}
#[cfg(debug_assertions)]
use self::imports::*;
