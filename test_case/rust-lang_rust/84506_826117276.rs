rust
#![feature(rustc_attrs)]

#[rustc_layout(debug)]
type WhateverYourFIs = /* specify */;
#[rustc_layout(debug)]
type ChainNode = chain::Node<WhateverYourFIs>;
#[rustc_layout(debug)]
type LinkedListNode = linked_list::Node<ChainNode>;
