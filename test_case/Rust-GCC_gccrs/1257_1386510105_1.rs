 c++
void
TypeCheckType::visit (HIR::BareFunctionType &fntype)
{
  TyTy::BaseType *return_type
    = fntype.has_return_type ()
	? TypeCheckType::Resolve (fntype.get_return_type ().get ())
	: TyTy::TupleType::get_unit_type (fntype.get_mappings ().get_hirid ());

