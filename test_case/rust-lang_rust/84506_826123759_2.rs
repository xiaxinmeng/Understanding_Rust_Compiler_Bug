rust
 #[repr(C, align(4))]                                                                                                                          
 pub struct Node<T> {                                                                                                                          
     next: *mut Node<T>,                                                                                                                       
     /// The value attached to this node.                                                                                                      
     pub value: T,                                                                                                                             
 }

 #[repr(C, align(4))]
 pub struct Node<F> {
     advance: unsafe fn(*mut ListNode<Node<()>>) -> bool,
     deallocate: unsafe fn(*mut ListNode<Node<()>>),
     fib: F,
 }
