
// basic interrogators
fn len(&self) -> usize
fn is_empty(&self) -> bool
fn first(&self) -> Option<&T>
fn first_mut(&mut self) -> Option<&mut T>
fn tail(&self) -> &[T]
fn tail_mut(&mut self) -> &mut [T]
fn init(&self) -> &[T]
fn init_mut(&mut self) -> &mut [T]
fn last(&self) -> Option<&T>
fn last_mut(&mut self) -> Option<&mut T>
fn get(&self, index: usize) -> Option<&T>
fn get_mut(&mut self, index: usize) -> Option<&mut T>
unsafe fn get_unchecked(&self, index: usize) -> &T
unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T
fn as_ptr(&self) -> *const T
fn as_mut_ptr(&mut self) -> *mut T

// mutation
fn swap(&mut self, a: usize, b: usize)
fn reverse(&mut self)

// iterators
fn iter(&self) -> Iter<T>
fn iter_mut(&mut self) -> IterMut<T>
fn windows(&self, size: usize) -> Windows<T>
fn chunks(&self, size: usize) -> Chunks<T>
fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<T>

// segmentation
fn split_at(&self, mid: usize) -> (&[T], &[T])
fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T])

fn split<F>(&self, pred: F) -> Split<T, F> where F: FnMut(&T) -> bool
fn split_mut<F>(&mut self, pred: F) -> SplitMut<T, F> where F: FnMut(&T) -> bool
fn splitn<F>(&self, n: usize, pred: F) -> SplitN<T, F> where F: FnMut(&T) -> bool
fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<T, F> where F: FnMut(&T) -> bool
fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<T, F> where F: FnMut(&T) -> bool
fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<T, F> where F: FnMut(&T) -> bool

// sorting and searching
fn contains(&self, x: &T) -> bool where T: PartialEq<T>
fn starts_with(&self, needle: &[T]) -> bool where T: PartialEq<T>
fn ends_with(&self, needle: &[T]) -> bool where T: PartialEq<T>
fn position_elem(&self, t: &T) -> Option<usize> where T: PartialEq<T>
fn rposition_elem(&self, t: &T) -> Option<usize> where T: PartialEq<T>
fn binary_search(&self, x: &T) -> Result<usize, usize> where T: Ord
fn binary_search_by<F>(&self, f: F) -> Result<usize, usize> where F: FnMut(&T) -> Ordering
fn sort(&mut self) where T: Ord
fn sort_by<F>(&mut self, compare: F) where F: FnMut(&T, &T) -> Ordering

// permut
fn permutations(&self) -> Permutations<T> where T: Clone
fn next_permutation(&mut self) -> bool where T: Ord
fn prev_permutation(&mut self) -> bool where T: Ord

// misc
fn clone_from_slice(&mut self, src: &[T]) -> usize where T: Clone
fn move_from(&mut self, src: Vec<T>, start: usize, end: usize) -> usize
fn to_vec(&self) -> Vec<T> where T: Clone
fn into_vec(self: Box<[T]>) -> Vec<T>

