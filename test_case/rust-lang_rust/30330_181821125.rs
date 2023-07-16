 rust
// from librustc_trans/trans/monomorphize.rs
let hash;
let s = {
    let mut state = SipHasher::new();
    hash_id.hash(&mut state);
    mono_ty.hash(&mut state); 
 // ^^^^^^^^^^^^^^^^^^^^^^^^^ 
 // the hash of a ty::Ty is derived from a memory address,
 // hash_id above also contains a vector of ty::Ty

    hash = format!("h{}", state.finish());
    let path = ccx.tcx().map.def_path_from_id(fn_node_id);
    exported_name(path, &hash[..])
};
