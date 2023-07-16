 rust
#[derive(Hash)]
enum SawItemComponent {
    SawItemExternCrate,
    SawItemUse,
    SawItemStatic(Mutability),
    SawItemConst,
    SawItemFn(Unsafety, Constness, Abi),
    ...
}

// See saw_expr() for reference
fn saw_item(node: &Item_) -> SawItemComponent {
    match *node {
        ItemExternCrate(..) => SawItemExternCrate,
        ItemUse(..) => SawItemUse,
        ItemStatic(_, mutability, _) => SawItemStatic,
        ItemConst(..) =>SawItemConst,
        ItemFn(_, unsafety, constness, abi, _, _) => SawItemFn(unsafety, constness, abi),
        ...
    }
}

fn visit_item(&mut self, i: &'tcx Item) {
    debug!("visit_item: {:?} st={:?}", i, self.st);
    SawItem(saw_item(&i.node)).hash(self.st);
    hash_span!(self, i.span);
    hash_attrs!(self, &i.attrs);
    visit::walk_item(self, i)
}
