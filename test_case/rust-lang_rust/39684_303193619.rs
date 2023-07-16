
...
DEBUG:rustc_trans::collector:  => recursion depth=18
DEBUG:rustc_trans::collector:  => type length=56
DEBUG:rustc_trans::collector: visiting rvalue const false
DEBUG:rustc_trans::collector: visiting constant const false @ bb0[0]
DEBUG:rustc_trans::collector: visiting rvalue _1
DEBUG:rustc_trans::collector: visiting rvalue const true
DEBUG:rustc_trans::collector: visiting constant const true @ bb0[5]
DEBUG:rustc_trans::collector: visiting rvalue _2
DEBUG:rustc_trans::monomorphize: resolve(def_id=DefId { krate: CrateNum(2), node: DefIndex(1660) => core/3e7145548a52c52830e3a99346688b33::iter[0]::sources[0]::empty[0] }, substs=Slice([()]))
DEBUG:rustc_trans::monomorphize:  => free item
DEBUG:rustc_trans::monomorphize: resolve(def_id=DefId { krate: CrateNum(2), node: DefIndex(1660) => core/3e7145548a52c52830e3a99346688b33::iter[0]::sources[0]::empty[0] }, substs=Slice([()])) = std::iter::empty::<()>
DEBUG:rustc_trans::collector: visit_item_use(Instance { def: Item(DefId { krate: CrateNum(2), node: DefIndex(1660) => core/3e7145548a52c52830e3a99346688b33::iter[0]::sources[0]::empty[0] }), substs: Slice([()]) }, is_direct_call=true)
DEBUG:rustc_trans::collector: create_fn_trans_item(instance=std::iter::empty::<()>)
DEBUG:rustc_trans::collector: visiting constant const std::iter::empty @ bb0[8]
DEBUG:rustc_trans::collector: visiting rvalue const false
DEBUG:rustc_trans::collector: visiting constant const false @ bb2[0]
DEBUG:rustc_trans::monomorphize: resolve(def_id=DefId { krate: CrateNum(2), node: DefIndex(1508) => core/3e7145548a52c52830e3a99346688b33::iter[0]::iterator[0]::Iterator[0]::chain[0] }, substs=Slice([std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Empty<()>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>]))
DEBUG:rustc_trans::monomorphize:  => associated item, attempting to find impl
DEBUG:rustc_trans::monomorphize: resolve_associated_item(trait_item=DefId { krate: CrateNum(2), node: DefIndex(1508) => core/3e7145548a52c52830e3a99346688b33::iter[0]::iterator[0]::Iterator[0]::chain[0] }, trait_id=DefId { krate: CrateNum(2), node: DefIndex(1500) => core/3e7145548a52c52830e3a99346688b33::iter[0]::iterator[0]::Iterator[0] }, rcvr_substs=Slice([std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Chain<std::iter::Empty<()>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>>, std::iter::Empty<()>]))
