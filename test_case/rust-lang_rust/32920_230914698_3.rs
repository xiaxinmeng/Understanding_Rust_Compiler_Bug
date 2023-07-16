
DW_TAG_structure_type
  DW_AT_name <OptimizedEnum>
  DW_AT_byte_size <16>

  DW_TAG_variant_part
    DW_AT_discr <ref to disr member below>

    DW_TAG_member // the discriminant
      DW_AT_type <&u32>
      DW_AT_artificial <true>
      DW_AT_data_member_location <8> // <-- just point to where the value is

    DW_TAG_variant
      DW_AT_name <Some>
      // Note the omitted DW_AT_discr_value, making this the default case

      DW_TAG_member
        DW_AT_type <i32>
        DW_AT_data_member_location <0>

      DW_TAG_member
        DW_AT_type <&u32>
        DW_AT_data_member_location <8>

    DW_TAG_variant
      DW_AT_name <None>
      DW_AT_discr_value <0>
