
stack backtrace:
   1:         0x66be29a8 - ZN2rt9backtrace3imp5write20hc5b481d5cd57af2aE9qE
   2:         0x66be641b - ZN2rt4init20hffee7254612f46fffnrE
   3:         0x668461cd - ZN6unwind18begin_unwind_inner20h484cd690c7389d13QhdE
   4:         0x6a675f76 - ZN10diagnostic11SpanHandler8span_bug20h61400d8c039e9bc6GMFE
   5:         0x6a676587 - ZN10diagnostic7Handler3bug20h637401670639e998ISFE
   6:           0x6ec9a2 - ZN6driver7session7Session3bug20h63d7d8460a66e09cFyxE
   7:           0xa83406 - ZN6middle5trans7type_of14sizing_type_of20hdd8be8c0c6eb6373zH9E
   8:           0xb7db11 - ZN6middle5trans3adt27PointerField...std..cmp..Eq27assert_receiver_is_tota
l_eq20ha32c3709fd4604c4JutE
   9:           0xa7c679 - ZN6middle5trans3adt14represent_type20h1294e6e14e5ff75ey5sE
  10:           0xa77577 - ZN6middle5trans7type_of7type_of20hafb26f53ba254fd53N9E
  11:           0xab45b5 - ZN6middle5trans7closure25make_closure_from_bare_fn20h4f1bcbc87f46e2c3lljE

  12:           0xab503e - ZN6middle5trans7closure25make_closure_from_bare_fn20h4f1bcbc87f46e2c3lljE

  13:           0xaaaf48 - ZN6middle2ty9expr_kind20h949e0064a45a1e82ilGE
  14:           0xa729df - ZN6middle5trans4expr5trans20ha48b94e0db233834pN2E
  15:           0xaaab87 - ZN6middle2ty9expr_kind20h949e0064a45a1e82ilGE
  16:           0xa729df - ZN6middle5trans4expr5trans20ha48b94e0db233834pN2E
  17:           0xaad323 - ZN6middle2ty9expr_kind20h949e0064a45a1e82ilGE
  18:           0xa71723 - ZN6middle5trans4expr10trans_into20h96733f5255894c9dvJ2E
  19:           0xa71a5b - ZN6middle5trans11controlflow11trans_block20hc018d4181ef5114d2WYE
  20:           0xb15c90 - ZN6middle5trans4base13trans_closure20hfb78e3f15b7b784dTQeE
  21:           0xa6411d - ZN6middle5trans4base8trans_fn20h94c1533573410e77h2eE
  22:           0xa658e1 - ZN6middle5trans12monomorphize14monomorphic_fn20h1618738c6d1617aeKoYE
  23:           0xa9921a - ZN6middle5trans6callee24trans_fn_ref_with_substs20h26b3d37002d9819aFD1E
  24:           0xa95e6e - ZN6middle5trans6callee12trans_fn_ref20hc6e86523bc4e67a9fs1E
  25:           0xa9de9c - ZN6middle5trans6common9expr_info20haa29a9eb7befc8d0bZ5E
  26:           0xa78a4b - ZN6middle5trans6callee16trans_call_inner20hf94d81b3cc0dc994nZ1E
  27:           0xa9d5e0 - ZN6middle5trans6callee10trans_call20h61df90c183273a62IT1E
  28:           0xaace3b - ZN6middle2ty9expr_kind20h949e0064a45a1e82ilGE
  29:           0xa71723 - ZN6middle5trans4expr10trans_into20h96733f5255894c9dvJ2E
  30:           0xb40c48 - ZN6middle5trans7cleanup9var_scope20h86fc45f82162f649VtxE
  31:           0xb40b18 - ZN6middle5trans7cleanup9var_scope20h86fc45f82162f649VtxE
  32:           0xb0d88f - ZN6middle5trans6_match11store_local20hac621f306ab9420b9eiE
  33:           0xa70d13 - ZN6middle5trans4base10init_local20hf01d11915bad5b1btZdE
  34:           0xa701e6 - ZN6middle5trans11controlflow10trans_stmt20h6a327ed75a825439WRYE
  35:           0xa71969 - ZN6middle5trans11controlflow11trans_block20hc018d4181ef5114d2WYE
  36:           0xb15c90 - ZN6middle5trans4base13trans_closure20hfb78e3f15b7b784dTQeE
  37:           0xa6411d - ZN6middle5trans4base8trans_fn20h94c1533573410e77h2eE
  38:           0xa617ec - ZN6middle5trans4base10trans_item20h7a9c36ad77c3fd91AlfE
  39:           0xb1f009 - ZN6middle5trans4base11trans_crate20ha6d41362f46fe1abQlgE
  40:           0xf4daf9 - ZN6driver6driver25phase_4_translate_to_llvm20hc8eefbd8dad9706cjZwE
  41:           0xf461f4 - ZN6driver6driver13compile_input20h0eef3c908953a111ewwE
  42:           0xfc2dff - ZN6driver7monitor20hcdaa282087678b71SaBE
  43:           0xfc1170 - ZN6driver7monitor20hcdaa282087678b71SaBE
  44:           0x71bf55 - ZN6driver6driver35OutputFilenames...std..clone..Clone5clone20h22daf11eed5
36512ElxE
  45:           0x71be97 - ZN6driver6driver35OutputFilenames...std..clone..Clone5clone20h22daf11eed5
36512ElxE
  46:         0x7072945c - ZN4task10spawn_opts20hae91090f93dc8031FteE
  47:         0x668aab5f - rust_try
  48:         0x668aab39 - rust_try
  49:         0x6684432e - ZN6unwind3try20hba110656f3557a7bU4cE
  50:         0x66844189 - ZN4task4Task3run20hdb858a047ca67fa4kzcE
  51:         0x707292b6 - ZN4task10spawn_opts20hae91090f93dc8031FteE
  52:         0x66845807 - ZN4task11BlockedTask14cast_from_uint20hffe040d42c989c01XRcE
  53:         0x76ad59ed - BaseThreadInitThunk
