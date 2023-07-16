rs
let src = &this.thir[src];
let dst = &this.thir[dst];
let dst = unpack!(block = this.as_place(block, dst));
unpack!(block = this.expr_into_dest(this.tcx.mk_place_deref(dst), block, src));
