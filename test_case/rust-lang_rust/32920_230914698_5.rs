
DW_TAG_structure_type
  DW_AT_name <CLikeEnum>
  DW_AT_byte_size <1>

  DW_TAG_variant_part
    DW_AT_discr <ref to disr member below>

    DW_TAG_member // the discriminant
      DW_AT_type <u8>
      DW_AT_artificial <true>
      DW_AT_data_member_location <0>

    DW_TAG_variant
      DW_AT_name <One>
      DW_AT_discr_value <0>

    DW_TAG_variant
      DW_AT_name <Two>
      DW_AT_discr_value <1>

    DW_TAG_variant
      DW_AT_name <Three>
      DW_AT_discr_value <2>
