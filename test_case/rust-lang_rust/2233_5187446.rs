
// Before
fn to_str(v: t) -> str {
    let mut i: uint = 0u;
    let mut rs: str = "";
    while i < v.nbits {
        rs +=
            alt tritv_get(v, i) {
              dont_care { "?" }
              ttrue { "1" }
              tfalse { "0" }
            };
        i += 1u;
    }
    ret rs;
}

// After
fn to_str(v: t) -> str {
    let mut i: uint = 0u;
    let mut rs: str = "";
    uint::range(0, v.nbits) {|i|
        rs +=
            alt tritv_get(v, i) {
              dont_care { "?" }
              ttrue { "1" }
              tfalse { "0" }
            };
        i += 1u;
    }
    ret rs;
}


// Before
while find_pre_post_state_fn(fcx, f_decl, f_body) { }

// After
loop {
    if find_pre_post_state_fn(fcx, f_decl, f_body) { break }
}


// Before
let mut i = 0u;
let mut data = [];
let mut offsets = [];
while i < vec::len(ccx.shape_cx.tag_order) {
    let did = ccx.shape_cx.tag_order[i];
    let variants = ty::enum_variants(ccx.tcx, did);
    let item_tyt = ty::lookup_item_type(ccx.tcx, did);
    let ty_param_count = vec::len(*item_tyt.bounds);
    // etc

// After
let mut i = 0u;
let mut data = [];
let mut offsets = [];
for ccx.shape_cx.tag_order.each {||
    let did = ccx.shape_cx.tag_order[i];
    let variants = ty::enum_variants(ccx.tcx, did);
    let item_tyt = ty::lookup_item_type(ccx.tcx, did);
    let ty_param_count = vec::len(*item_tyt.bounds);
    // etc


// Before
let mut cx = cx;
while option::is_none(cx.block_span) {
    alt cx.parent {
      parent_some(b) { cx = b; }
      parent_none { fail; }
    }
}

// After
let mut cx = cx;
loop {
    if option::is_none(cx.block_span) { break }
    alt cx.parent {
      parent_some(b) { cx = b; }
      parent_none { fail; }
    }
}
