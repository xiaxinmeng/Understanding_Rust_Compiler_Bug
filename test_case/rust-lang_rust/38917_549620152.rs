
error[E0277]: the trait bound `<D as DomTraversal<<E as TElement>::ConcreteNode>>::BorrowElement: std::borrow::Borrow<E>` is not satisfied
  --> src/main.rs:15:1
   |
11 |   trait DomTraversal<N: TNode> {
   |   ---------------------------- required by `DomTraversal`
...
15 | / fn recalc_style_at<E, D>()
16 | |     where E: TElement,
17 | |           D: DomTraversal<E::ConcreteNode>,
   | |                                            - help: consider further restricting the associated type: `, <D as DomTraversal<<E as TElement>::ConcreteNode>>::BorrowElement: std::borrow::Borrow<E>`
18 | | {
19 | | }
   | |_^ the trait `std::borrow::Borrow<E>` is not implemented for `<D as DomTraversal<<E as TElement>::ConcreteNode>>::BorrowElement`
