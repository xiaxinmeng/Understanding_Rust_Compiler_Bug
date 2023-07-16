rust
ty::Closure(..) => { 
    use rustc_data_structures::base_n;
    output.push_str("closure-"); // <-- could also use a short prefix here
    let hash = tcx.def_path_hash(def_id).0.to_smaller_hash(); // 64 bits is enough
    base_n::push_str(hash as u128, 62, output); // base62 only contains alphanumeric chars
}
