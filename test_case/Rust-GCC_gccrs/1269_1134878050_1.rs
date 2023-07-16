c++
  // is this the array index of a slice?
  if (SLICE_TYPE_P (TREE_TYPE (array_reference))
      && TREE_CODE (TREE_TYPE (index)) == INTEGER_TYPE)
    {
      // See:
      // https://users.rust-lang.org/t/why-this-does-not-lead-to-recursion/50306
      translated
	= direct_slice_index_access (expr.get_locus (), array_reference, index);
      return;
    }
