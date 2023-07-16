
pub fn get_trait_def(cdata: Cmd,
                     item_id: ast::NodeId,
                     tcx: ty::ctxt) -> ty::TraitDef
{
    let item_doc = lookup_item(item_id, cdata.data);
    let tp_defs = item_ty_param_defs(item_doc, tcx, cdata,
                                     tag_items_data_item_ty_param_bounds);
    let rp_defs = item_region_param_defs(item_doc, tcx, cdata);
...
