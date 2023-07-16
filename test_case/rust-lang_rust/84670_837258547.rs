
0x1015 : Length = 18, Leaf = 0x1203 LF_FIELDLIST
    list[0] = LF_MEMBER, public, type = T_64PUSHORT(0621), offset = 0
        member name = '__0'

0x1016 : Length = 78, Leaf = 0x1505 LF_STRUCTURE
    # members = 1,  field list type 0x1015, 
    Derivation list type 0x0000, VT shape type 0x0000
    Size = 8, class name = core::option::Some, unique name = 614d2e1de3c5e373cf909c161e5f6384::Some, UDT(0x00001037)

0x1035 : Length = 18, Leaf = 0x1203 LF_FIELDLIST
    list[0] = LF_MEMBER, public, type = T_64PQUAD(0613), offset = 0
        member name = '__0'

0x1037 : Length = 78, Leaf = 0x1505 LF_STRUCTURE
    # members = 1,  field list type 0x1035, 
    Derivation list type 0x0000, VT shape type 0x0000
    Size = 8, class name = core::option::Some, unique name = 2dfef4176b9038163574c2ab2dbdc08f::Some, UDT(0x00001037)

WARNING: UDT mismatch for core::option::Some
<<<<<<
0x1016 : Length = 78, Leaf = 0x1505 LF_STRUCTURE
    # members = 1,  field list type 0x1015, 
    Derivation list type 0x0000, VT shape type 0x0000
    Size = 8, class name = core::option::Some, unique name = 614d2e1de3c5e373cf909c161e5f6384::Some, UDT(0x00001037)

******
0x1037 : Length = 78, Leaf = 0x1505 LF_STRUCTURE
    # members = 1,  field list type 0x1035, 
    Derivation list type 0x0000, VT shape type 0x0000
    Size = 8, class name = core::option::Some, unique name = 2dfef4176b9038163574c2ab2dbdc08f::Some, UDT(0x00001037)

>>>>>>
