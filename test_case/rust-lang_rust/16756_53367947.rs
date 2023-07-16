
#![feature(unsafe_destructor)]

use std::rc::{try_unwrap, Rc};

struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    #[inline]
    fn new(elem: T) -> Node<T> {
        Node { elem: elem, next: None }
    }
}

/// An immutable singly-linked list, as seen in basically every functional language
pub struct ImmutSList<T> {
    front: Option<Rc<Node<T>>>,
    length: uint,
}

impl<T> ImmutSList<T> {
    /// Constructs a new, empty `ImmutSList`
    #[inline]
    pub fn new () -> ImmutSList<T> {
        ImmutSList{ front: None, length: 0 }
    }

    /// Gets the length of the list
    #[inline]
    pub fn len(&self) -> uint {
        self.length
    }

    /// Returns a copy of the list, with `elem` appended to the front
    #[inline]
    pub fn append (&self, elem: T) -> ImmutSList<T>{
        let mut new_node = Node::new(elem);
        new_node.next = self.front.clone();

        ImmutSList{
            front: Some(Rc::new(new_node)),
            length: self.len() + 1,
        }
    }

    /// Returns a reference to the first element in the list
    #[inline]
    pub fn head (&self) -> Option<&T> {
        self.front.as_ref().map(|node| &node.elem)
    }

    /// Returns a copy of the list, with the first element removed
    #[inline]
    pub fn tail (&self) -> ImmutSList<T> {
        let len = self.len();
        if len <= 1 {
            ImmutSList::new()
        } else {
            ImmutSList {
                length: len - 1,
                front: self.front.as_ref().unwrap().next.clone(),
            }
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for ImmutSList<T> {
    fn drop (&mut self) {
        // don't want to blow the stack with destructors,
        // but also don't want to walk the whole list.
        // So walk the list until we find a non-uniquely owned item
        let mut head = self.front.take();
        loop {
            let temp = head;
            match temp {
                Some(node) => match try_unwrap(node) {
                    Ok(mut node) => {
                        head = node.next.take();
                    }
                    _ => return,
                },
                _ => return,
            }
        }
    }
}
