 rust
#[derive(Debug)]
pub struct Node<T>{
    pub item: T,
    pub next: Option<Box<Node<T>>>
}

impl <T>Node<T>{
   pub fn new(item: T) -> Node<T> {
       Node::<T> {item: item, next: None}
   }
}

pub fn main() {
    let mut first = Node::new("to");
    let mut second = Node::new("be");
    let mut third = Node::new("or");
    let mut fourth = Node::new("not");
    let fifth = Node::new("to");

    first.next = Some(Box::new(second));
    second.next = Some(Box::new(third));
    third.next = Some(Box::new(fourth));
    fourth.next = Some(Box::new(fifth));

    println!("{:?}", first);
}
