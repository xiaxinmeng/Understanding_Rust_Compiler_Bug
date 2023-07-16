 rust
use std::iter::Iterator;

pub struct PreOrderNodes<'a, 'b> {
  queue: Vec<&'a Node<'b>>
}

impl<'a, 'b> Iterator<&'a Node<'b>> for PreOrderNodes<'a, 'b> {
  fn next( &mut self ) -> Option<&'a Node<'b>> {
    match self.queue.pop() {
      ex @ Some( node ) => {
        match node.contents {
          Children( ref x ) => {
            for child in x.as_slice().iter().rev() {
              self.queue.push( child )
            }
          }
          _ => ()
        };
        ex
      }
      _ => None
    }
  }
}

pub enum NodeContents<'a> {
  Data( &'a [u8] ),
  Children( Vec<Node<'a>> )
}


pub struct Node<'a> {
  pub name: &'static str,
  pub contents: NodeContents<'a>
}

impl<'a> Node<'a> {
  pub fn preOrder<'b>( &'b self ) -> PreOrderNodes<'b, 'a> {
    PreOrderNodes { queue: vec!( self ) }
  }
}

#[cfg(test)]
mod tests {
  use super::{Node, Data};

  #[test]
  fn preOrder() {
    static data: &'static [u8] = &[];
    let root = Node { name: "a", contents: Data( data ) };
    let mut names : Vec<char> = vec!();
    root.preOrder().map( |x| {
      names.push( x.name.char_at( 0 ) )
    });

    assert_eq!( names, vec!( 'a' ) )
  }
}
