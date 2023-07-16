
DW_TAG_structure_type
  DW_AT_name <RegularEnum>
  DW_AT_byte_size <12>

  DW_TAG_variant_part
    DW_AT_discr <ref to disr member below>

    DW_TAG_member // the discriminant
      DW_AT_type <u32>
      DW_AT_artificial <true>
      DW_AT_data_member_location <0>

    DW_TAG_variant
      DW_AT_name <One>
      DW_AT_discr_value <0>

      DW_TAG_member
        DW_AT_name <field1>
        DW_AT_type <u32>
        DW_AT_data_member_location <4>

      DW_TAG_member
        DW_AT_name <field2>
        DW_AT_type <f32>
        DW_AT_data_member_location <8>

    DW_TAG_variant
      DW_AT_name <Two>
      DW_AT_discr_value <1>

      DW_TAG_member
        DW_AT_type <i32>
        DW_AT_data_member_location <4>
