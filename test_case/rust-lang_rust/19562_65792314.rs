
trait IntoIndex {
    fn into_index(self) -> uint;
}

impl IntoIndex for BinarySearchResult {
    fn into_index(self) -> uint {
        match self {
            BinarySearchResult::Found(n) => n,
            BinarySearchResult::NotFound(n) => n
        } 
    }
}
