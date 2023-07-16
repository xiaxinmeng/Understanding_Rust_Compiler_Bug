c++
tree
CompileExpr::direct_slice_index_access (Location expr_locus,
					tree slice_reference, tree index)
{
  rust_assert (SLICE_TYPE_P (TREE_TYPE (slice_reference)));
  rust_assert (TREE_CODE (TREE_TYPE (index)) == INTEGER_TYPE);

  // FIXME This needs bounds checking to the panic handler

  // get the data pointer
  tree data_pointer
    = ctx->get_backend ()->struct_field_expression (slice_reference, 0,
						    expr_locus);

  tree element_type = TYPE_POINTER_TO (TREE_TYPE (data_pointer));
  tree pointer_offset_expr
    = pointer_offset_expression (data_pointer, index,
				 expr_locus.gcc_location ());

  return ctx->get_backend ()->indirect_expression (element_type,
						   pointer_offset_expr, true,
						   expr_locus);
}
