
10$: DW_TAG_array_type
        DW_AT_type(reference to real)
        DW_AT_rank(expression=
            DW_OP_push_object_address
            DW_OP_lit<n> ! offset of rank in descriptor
            DW_OP_plus
            DW_OP_deref)
        DW_AT_data_location(expression=
            DW_OP_push_object_address
            DW_OP_lit<n> ! offset of data in descriptor
            DW_OP_plus
            DW_OP_deref)
11$:    DW_TAG_generic_subrange
            DW_AT_type(reference to integer)
            DW_AT_lower_bound(expression=
            ! Looks up the lower bound of dimension i.
            ! Operation ! Stack effect
            ! (implicit) ! i
                DW_OP_lit<n> ! i sizeof(dim)
                DW_OP_mul ! dim[i]
                DW_OP_lit<n> ! dim[i] offsetof(dim)
                DW_OP_plus ! dim[i]+offset
                DW_OP_push_object_address ! dim[i]+offsetof(dim) objptr
                DW_OP_plus ! objptr.dim[i]
                DW_OP_lit<n> ! objptr.dim[i] offsetof(lb)
                DW_OP_plus ! objptr.dim[i].lowerbound
                DW_OP_deref) ! *objptr.dim[i].lowerbound
            DW_AT_upper_bound(expression=
            ! Looks up the upper bound of dimension i.
                DW_OP_lit<n> ! sizeof(dim)
                DW_OP_mul
                DW_OP_lit<n> ! offsetof(dim)
                DW_OP_plus
                DW_OP_push_object_address
                DW_OP_plus
                DW_OP_lit<n> ! offset of upperbound in dim
                DW_OP_plus
                DW_OP_deref)
            DW_AT_byte_stride(expression=
                ! Looks up the byte stride of dimension i.
                ...
                ! (analogous to DW_AT_upper_bound)
            )
