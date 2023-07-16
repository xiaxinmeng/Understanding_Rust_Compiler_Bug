
driver::driver::phase_3_run_analysis_passes
  middle::typeck::check_crate
    middle::typeck::collect::collect_item_types
      collect::convert
        ItemStruct generics has region
        collect::ty_of_item
          in ItemStruct case
          collect::ty_generics_for_type
            collect::ty_generics
          converts ItemStruct to PolyTy

        PolyTy generics still has region
        collect::convert_struct
          converts ast::StructDef to ty::t
          mk_item_substs
          ty::mk_struct

  Somewhere in here Subst on struct is dropped

  middle::kind::check_crate
    middle::kind::check_item
      middle::ty::type_contents
        match on `get(ty).sty`
        find `ty_struct(did, substs)`
        middle::ty::struct_fields
          middle::ty::lookup_field_type
            middle::subst::Subst::subst_spanned
              middle::subst::TypeFolder::fold_region(r: ty::Region)
                match on `r`
                find ty::ReEarlyBound(_, space, i, region_name)
                match on `substs.regions`
                find NonerasedRegions(regions)
                look for region index `i` in substitutions for space `space`
                fail
