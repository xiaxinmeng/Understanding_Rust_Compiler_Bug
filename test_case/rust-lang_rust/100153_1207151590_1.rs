rust
#[cfg(any(feature = "arrayvec", feature = "slicevec"))]
use std::cell::Cell;
#[cfg(any(feature = "slicevec"))]
use std::mem::MaybeUninit;
#[cfg(feature = "arrayvec")]
use arrayvec::ArrayVec;
#[cfg(feature = "slicevec")]
use slicevec::SliceVec;

use std::cell::Cell;

#[derive(Debug, Clone)]
struct DropTracker<'a>(&'a Cell<u32>);

impl<'a> Drop for DropTracker<'a> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[derive(Debug, Clone)]
struct Node<'a, 'b: 'a>(Option<&'a Node<'a, 'b>>, u32, DropTracker<'b>);

#[cfg(feature = "arrayvec")]
#[test]
fn array_arena() {
    let drop_counter = Cell::new(0);
    {
        let arena = Arena::new(ArrayVec::<_, 2>::new());

        let mut node = arena.alloc(Node(None, 1, DropTracker(&drop_counter))).unwrap();
        node = arena.alloc(Node(Some(node), 2, DropTracker(&drop_counter))).unwrap();

        assert_eq!(node.1, 2);
        assert_eq!(node.0.unwrap().1, 1);
        assert!(node.0.unwrap().0.is_none());
        assert_eq!(arena.len(), 2);

        let error = arena.alloc(Node(Some(node), 3, DropTracker(&drop_counter))).unwrap_err();
        let error_elem = error.element();
        assert_eq!(error_elem.1, 3);

        assert_eq!(drop_counter.get(), 0);
        drop(error_elem);
        assert_eq!(drop_counter.get(), 1);

        drop(node);
    }
    assert_eq!(drop_counter.get(), 3);
    drop_counter.set(0);

    {
        let arena = Arena::new(ArrayVec::<_, 25>::new());

        let mut node = arena.alloc(Node(None, 1, DropTracker(&drop_counter))).unwrap();
        node = arena.alloc(Node(Some(node), 2, DropTracker(&drop_counter))).unwrap();
        node = arena.alloc(Node(Some(node), 3, DropTracker(&drop_counter))).unwrap();

        assert_eq!(node.1, 3);
        assert_eq!(node.0.unwrap().1, 2);
        assert_eq!(arena.len(), 3);

        let mut node = arena.alloc(Node(None, 4, DropTracker(&drop_counter))).unwrap();
        node = arena.alloc(Node(Some(node), 5, DropTracker(&drop_counter))).unwrap();

        assert_eq!(drop_counter.get(), 0);
        assert_eq!(node.1, 5);
        assert_eq!(node.0.unwrap().1, 4);
        assert!(node.0.unwrap().0.unwrap().0.is_none());
    }
    assert_eq!(drop_counter.get(), 7);
}

fn main() { }
