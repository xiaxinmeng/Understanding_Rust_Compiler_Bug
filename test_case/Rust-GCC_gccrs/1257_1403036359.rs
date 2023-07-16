 c++
  TyTy::BaseType *return_type = nullptr;
  if (! fntype.has_return_type ())
	{
	    std::vector<std::unique_ptr<HIR::Type>> empty_tuple;
		HIR::Type* t = new HIR::TupleType(std::move (fntype.get_mappings ()), std::move (empty_tuple), std::move (fntype.get_locus ()));
		return_type = TypeCheckType::Resolve (t);
	}
  else
	return_type = TypeCheckType::Resolve (fntype.get_return_type ().get ());
