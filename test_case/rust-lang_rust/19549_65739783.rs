
src/librustc/middle/ty.rs:1145:1: 1182:2 note: total size: 32 bytes
src/librustc/middle/ty.rs:1145 pub enum sty<'tcx> {
src/librustc/middle/ty.rs:1146     ty_bool,
src/librustc/middle/ty.rs:1147     ty_char,
src/librustc/middle/ty.rs:1148     ty_int(ast::IntTy),
src/librustc/middle/ty.rs:1149     ty_uint(ast::UintTy),
src/librustc/middle/ty.rs:1150     ty_float(ast::FloatTy),
                               ...
src/librustc/middle/ty.rs:1146:5: 1146:12 note: variant data: 0 bytes
src/librustc/middle/ty.rs:1146     ty_bool,
                                   ^~~~~~~
src/librustc/middle/ty.rs:1147:5: 1147:12 note: variant data: 0 bytes
src/librustc/middle/ty.rs:1147     ty_char,
                                   ^~~~~~~
src/librustc/middle/ty.rs:1148:5: 1148:23 note: variant data: 1 bytes
src/librustc/middle/ty.rs:1148     ty_int(ast::IntTy),
                                   ^~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1149:5: 1149:25 note: variant data: 1 bytes
src/librustc/middle/ty.rs:1149     ty_uint(ast::UintTy),
                                   ^~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1150:5: 1150:27 note: variant data: 1 bytes
src/librustc/middle/ty.rs:1150     ty_float(ast::FloatTy),
                                   ^~~~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1158:5: 1158:39 note: variant data: 16 bytes
src/librustc/middle/ty.rs:1158     ty_enum(DefId, &'tcx Substs<'tcx>),
                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1159:5: 1159:22 note: variant data: 8 bytes
src/librustc/middle/ty.rs:1159     ty_uniq(Ty<'tcx>),
                                   ^~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1160:5: 1160:11 note: variant data: 0 bytes
src/librustc/middle/ty.rs:1160     ty_str,
                                   ^~~~~~
src/librustc/middle/ty.rs:1161:5: 1161:35 note: variant data: 24 bytes
src/librustc/middle/ty.rs:1161     ty_vec(Ty<'tcx>, Option<uint>), // Second field is length.
                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1162:5: 1162:21 note: variant data: 16 bytes
src/librustc/middle/ty.rs:1162     ty_ptr(mt<'tcx>),
                                   ^~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1163:5: 1163:36 note: variant data: 24 bytes
src/librustc/middle/ty.rs:1163     ty_rptr(&'tcx Region, mt<'tcx>),
                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1164:5: 1164:37 note: variant data: 8 bytes
src/librustc/middle/ty.rs:1164     ty_bare_fn(&'tcx BareFnTy<'tcx>),
                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1165:5: 1165:37 note: variant data: 8 bytes
src/librustc/middle/ty.rs:1165     ty_closure(Box<ClosureTy<'tcx>>),
                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1166:5: 1166:33 note: variant data: 8 bytes
src/librustc/middle/ty.rs:1166     ty_trait(Box<TyTrait<'tcx>>),
                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1167:5: 1167:41 note: variant data: 16 bytes
src/librustc/middle/ty.rs:1167     ty_struct(DefId, &'tcx Substs<'tcx>),
                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1168:5: 1168:64 note: variant data: 24 bytes
src/librustc/middle/ty.rs:1168     ty_unboxed_closure(DefId, &'tcx Region, &'tcx Substs<'tcx>),
                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1169:5: 1169:26 note: variant data: 24 bytes
src/librustc/middle/ty.rs:1169     ty_tup(Vec<Ty<'tcx>>),
                                   ^~~~~~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1171:5: 1171:22 note: variant data: 16 bytes
src/librustc/middle/ty.rs:1171     ty_param(ParamTy), // type parameter
                                   ^~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1172:5: 1172:22 note: variant data: 8 bytes
src/librustc/middle/ty.rs:1172     ty_open(Ty<'tcx>), // A deref'ed fat pointer, i.e., a dynamically sized value
                                   ^~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1178:5: 1178:22 note: variant data: 8 bytes
src/librustc/middle/ty.rs:1178     ty_infer(InferTy), // something used only during inference/typeck
                                   ^~~~~~~~~~~~~~~~~
src/librustc/middle/ty.rs:1179:5: 1179:11 note: variant data: 0 bytes
src/librustc/middle/ty.rs:1179     ty_err, // Also only used during inference/typeck, to represent
                                   ^~~~~~
