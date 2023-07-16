rust
struct WalkableList<'a, T> {
    list: &'a mut LinkedList<T>,
    deletedNodes: Option<NonNull<Node<T>>, // element already dropped when in this list
    walkerTracker: Rc<()>,
}

#[derive(Clone)]
struct Walker<T> {
    node: Option<NonNull<Node<T>>>,
    walkerTracker: Rc<()>,
}

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: ManuallyDrop<T>,
}

impl<T> Drop for WalkableList<'_, T> {
    fn drop(&mut self) {
        assert!(self.walkerTracker.get_mut().is_some(), "Walkers left");
        while let Some(node) = self.deletedNodes {
            let node = Box::from_raw(node.as_ptr());
            self.deletedNodes = node.next;
            // don't drop node.element; already dropped
        }
    }
}

impl<T> WalkableList<'_, T> {
    pub fn begin(&self) -> Walker<T> {
        Walker { node: self.list.head, walkerTracker: self.walkerTracker.clone(), }
    }
    pub fn remove(&mut self, w: &Walker<T>) -> Option<T> {
        assert!(Rc::ptr_eq(&self.walkerTracker, &w.walkerTracker);
        let node = w.node?;
        unsafe {
            if node.as_ref().prev == Some(node) {
                // node is deleted
                return None;
            }
            self.list.unlink_node(node);
            let retval = ManuallyDrop::take(&mut node.as_mut().element);
            node.prev = Some(node); // mark as deleted
            node.next = self.deletedNodes;
            self.deletedNodes = node;
            Some(retval)
        }
    }
    pub fn get(&self, w: &Walker<T>) -> Option<&T> {
        // ...
    }
    pub fn get_mut(&mut self, w: &Walker<T>) -> Option<&mut T> {
        assert!(Rc::ptr_eq(&self.walkerTracker, &w.walkerTracker);
        let node = w.node?;
        unsafe {
            if node.as_ref().prev == Some(node) {
                // node is deleted
                return None;
            }
            Some(&mut *(*node.as_ptr()).element)
        }
    }
    // other members
}
