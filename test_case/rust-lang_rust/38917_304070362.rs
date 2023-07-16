rust
use std::borrow::Borrow;

trait TNode: Sized {
  type ConcreteElement: TElement<ConcreteNode = Self>;
}

trait TElement: Sized {
    type ConcreteNode: TNode<ConcreteElement = Self>;
}

trait DomTraversal<N: TNode> {
    type BorrowElement: Borrow<N::ConcreteElement>;
}

fn recalc_style_at<E, D>()
    where E: TElement,
          D: DomTraversal<E::ConcreteNode>,
{
}


fn main() {
}
