
*** IR Dump After Instrument function entry/exit with calls to e.g. mcount() (pre inlining) ***
; Function Attrs: nonlazybind uwtable
define void @_RNvXs_NtCsijNWNMdNqoj_7pyembed18interpreter_configRNtNtB6_6config39ResolvedOxidizedPythonInterpreterConfigINtNtCskrsM4FCwAVA_4core7convert7TryIntoNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigE8try_into(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %0, %"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !78453 {
  %3 = alloca %"std::path::PathBuf"*, align 8
  %4 = alloca %"std::vec::Vec<std::ffi::OsString>"*, align 8
  %5 = alloca { i8*, i32 }, align 8
  %6 = alloca {}, align 1
  %7 = alloca {}, align 1
  %8 = alloca {}, align 1
  %9 = alloca %"config::ResolvedOxidizedPythonInterpreterConfig"*, align 8
  %10 = alloca %"pyo3::ffi::PyConfig", align 8
  %11 = alloca %"error::NewInterpreterError", align 8
  %12 = alloca %"error::NewInterpreterError", align 8
  %13 = alloca %"error::NewInterpreterError", align 8
  %14 = alloca %"std::result::Result<(), error::NewInterpreterError>", align 8
  %15 = alloca %"std::result::Result<(), error::NewInterpreterError>", align 8
  %16 = alloca %"error::NewInterpreterError", align 8
  %17 = alloca %"error::NewInterpreterError", align 8
  %18 = alloca %"error::NewInterpreterError", align 8
  %19 = alloca %"std::result::Result<(), error::NewInterpreterError>", align 8
  %20 = alloca %"std::result::Result<(), error::NewInterpreterError>", align 8
  %21 = alloca {}, align 1
  %22 = alloca {}, align 1
  %23 = alloca %"std::path::PathBuf"*, align 8
  %24 = alloca %"error::NewInterpreterError", align 8
  %25 = alloca %"error::NewInterpreterError", align 8
  %26 = alloca %"error::NewInterpreterError", align 8
  %27 = alloca %"error::NewInterpreterError", align 8
  %28 = alloca %"error::NewInterpreterError", align 8
  %29 = alloca %"std::result::Result<(), error::NewInterpreterError>", align 8
  %30 = alloca %"std::result::Result<(), error::NewInterpreterError>", align 8
  %31 = alloca {}, align 1
  %32 = alloca %"pyo3::ffi::PyConfig", align 8
  %33 = alloca %"error::NewInterpreterError", align 8
  %34 = alloca %"error::NewInterpreterError", align 8
  %35 = alloca %"error::NewInterpreterError", align 8
  %36 = alloca %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>", align 8
  %37 = alloca %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>", align 8
  %38 = alloca %"pyo3::ffi::PyConfig", align 8
  store %"config::ResolvedOxidizedPythonInterpreterConfig"* %1, %"config::ResolvedOxidizedPythonInterpreterConfig"** %9, align 8
  call void @llvm.dbg.declare(metadata %"config::ResolvedOxidizedPythonInterpreterConfig"** %9, metadata !78457, metadata !DIExpression()), !dbg !78482
  call void @llvm.dbg.declare(metadata %"pyo3::ffi::PyConfig"* %38, metadata !78458, metadata !DIExpression()), !dbg !78483
  call void @llvm.dbg.declare(metadata %"error::NewInterpreterError"* %35, metadata !78460, metadata !DIExpression()), !dbg !78484
  call void @llvm.dbg.declare(metadata %"pyo3::ffi::PyConfig"* %32, metadata !78462, metadata !DIExpression()), !dbg !78485
  call void @llvm.dbg.declare(metadata %"error::NewInterpreterError"* %28, metadata !78466, metadata !DIExpression()), !dbg !78486
  call void @llvm.dbg.declare(metadata {}* %8, metadata !78468, metadata !DIExpression()), !dbg !78487
  call void @llvm.dbg.declare(metadata %"std::path::PathBuf"** %23, metadata !78470, metadata !DIExpression()), !dbg !78488
  call void @llvm.dbg.declare(metadata %"error::NewInterpreterError"* %18, metadata !78474, metadata !DIExpression()), !dbg !78489
  call void @llvm.dbg.declare(metadata {}* %7, metadata !78476, metadata !DIExpression()), !dbg !78490
  call void @llvm.dbg.declare(metadata %"error::NewInterpreterError"* %13, metadata !78478, metadata !DIExpression()), !dbg !78491
  call void @llvm.dbg.declare(metadata {}* %6, metadata !78480, metadata !DIExpression()), !dbg !78492
  %39 = bitcast %"pyo3::ffi::PyConfig"* %38 to i8*, !dbg !78493
  call void @llvm.lifetime.start.p0i8(i64 392, i8* %39), !dbg !78493
  %40 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %37 to i8*, !dbg !78494
  call void @llvm.lifetime.start.p0i8(i64 400, i8* %40), !dbg !78494
  %41 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %36 to i8*, !dbg !78494
  call void @llvm.lifetime.start.p0i8(i64 400, i8* %41), !dbg !78494
  %42 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !78495
  br label %43, !dbg !78495

43:                                               ; preds = %2
  %44 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %42, i32 0, i32 5, !dbg !78496
  call void @_RNvNtCsijNWNMdNqoj_7pyembed18interpreter_config38python_interpreter_config_to_py_config(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %36, %"python_packaging::interpreter::PythonInterpreterConfig"* noalias readonly align 8 dereferenceable(560) %44), !dbg !78494
  br label %45, !dbg !78494

45:                                               ; preds = %43
  invoke void @_RNvXsy_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops3try3Try11into_resultB1M_(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %37, %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture dereferenceable(400) %36)
          to label %46 unwind label %58, !dbg !78494

46:                                               ; preds = %45
  %47 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %36 to i8*, !dbg !78494
  call void @llvm.lifetime.end.p0i8(i64 400, i8* %47), !dbg !78494
  %48 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %37 to i64*, !dbg !78497
  %49 = load i64, i64* %48, align 8, !dbg !78497, !range !2867
  switch i64 %49, label %65 [
    i64 0, label %66
    i64 1, label %79
  ], !dbg !78497

50:                                               ; preds = %349, %281, %156, %91, %58
  %51 = bitcast { i8*, i32 }* %5 to i8**, !dbg !78498
  %52 = load i8*, i8** %51, align 8, !dbg !78498
  %53 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1, !dbg !78498
  %54 = load i32, i32* %53, align 8, !dbg !78498
  %55 = bitcast { i8*, i32 }* %5 to i8*, !dbg !78498
  call void @llvm.lifetime.end.p0i8(i64 16, i8* %55), !dbg !78498
  %56 = insertvalue { i8*, i32 } undef, i8* %52, 0, !dbg !78498
  %57 = insertvalue { i8*, i32 } %56, i32 %54, 1, !dbg !78498
  resume { i8*, i32 } %57, !dbg !78498

58:                                               ; preds = %321, %253, %128, %45
  %59 = landingpad { i8*, i32 }
          cleanup
  %60 = bitcast { i8*, i32 }* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %60)
  %61 = extractvalue { i8*, i32 } %59, 0
  %62 = extractvalue { i8*, i32 } %59, 1
  %63 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 0
  store i8* %61, i8** %63, align 8
  %64 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1
  store i32 %62, i32* %64, align 8
  br label %50

65:                                               ; preds = %46
  unreachable, !dbg !78494

66:                                               ; preds = %46
  %67 = bitcast %"pyo3::ffi::PyConfig"* %32 to i8*, !dbg !78494
  call void @llvm.lifetime.start.p0i8(i64 392, i8* %67), !dbg !78494
  %68 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %37 to %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Ok"*, !dbg !78494
  %69 = getelementptr inbounds %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Ok", %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Ok"* %68, i32 0, i32 1, !dbg !78494
  %70 = bitcast %"pyo3::ffi::PyConfig"* %32 to i8*, !dbg !78494
  %71 = bitcast %"pyo3::ffi::PyConfig"* %69 to i8*, !dbg !78494
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %70, i8* align 8 %71, i64 392, i1 false), !dbg !78494
  %72 = bitcast %"pyo3::ffi::PyConfig"* %38 to i8*, !dbg !78485
  %73 = bitcast %"pyo3::ffi::PyConfig"* %32 to i8*, !dbg !78485
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %72, i8* align 8 %73, i64 392, i1 false), !dbg !78485
  %74 = bitcast %"pyo3::ffi::PyConfig"* %32 to i8*, !dbg !78497
  call void @llvm.lifetime.end.p0i8(i64 392, i8* %74), !dbg !78497
  %75 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %37 to i64*, !dbg !78499
  %76 = load i64, i64* %75, align 8, !dbg !78499, !range !2867
  %77 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %37 to i8*, !dbg !78499
  call void @llvm.lifetime.end.p0i8(i64 400, i8* %77), !dbg !78499
  %78 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !78500
  br label %109, !dbg !78500

79:                                               ; preds = %46
  %80 = bitcast %"error::NewInterpreterError"* %35 to i8*, !dbg !78497
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %80), !dbg !78497
  %81 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %37 to %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err"*, !dbg !78497
  %82 = getelementptr inbounds %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err", %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err"* %81, i32 0, i32 1, !dbg !78497
  %83 = bitcast %"error::NewInterpreterError"* %35 to i8*, !dbg !78497
  %84 = bitcast %"error::NewInterpreterError"* %82 to i8*, !dbg !78497
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %83, i8* align 8 %84, i64 32, i1 false), !dbg !78497
  %85 = bitcast %"error::NewInterpreterError"* %34 to i8*, !dbg !78484
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %85), !dbg !78484
  %86 = bitcast %"error::NewInterpreterError"* %33 to i8*, !dbg !78484
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %86), !dbg !78484
  %87 = bitcast %"error::NewInterpreterError"* %33 to i8*, !dbg !78484
  %88 = bitcast %"error::NewInterpreterError"* %35 to i8*, !dbg !78484
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %87, i8* align 8 %88, i64 32, i1 false), !dbg !78484
  invoke void @_RNvXs2_NtCskrsM4FCwAVA_4core7convertNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorINtB5_4FromBy_E4fromBC_(%"error::NewInterpreterError"* noalias nocapture sret(%"error::NewInterpreterError") dereferenceable(32) %34, %"error::NewInterpreterError"* noalias nocapture dereferenceable(32) %33)
          to label %89 unwind label %94, !dbg !78484

89:                                               ; preds = %79
  %90 = bitcast %"error::NewInterpreterError"* %33 to i8*, !dbg !78484
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %90), !dbg !78484
  invoke void @_RNvXsy_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops3try3Try10from_errorB1M_(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %0, %"error::NewInterpreterError"* noalias nocapture dereferenceable(32) %34)
          to label %101 unwind label %94, !dbg !78501

91:                                               ; preds = %94
  %92 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %37 to i64*, !dbg !78499
  %93 = load i64, i64* %92, align 8, !dbg !78499, !range !2867
  br label %50, !dbg !78499

94:                                               ; preds = %89, %79
  %95 = landingpad { i8*, i32 }
          cleanup
  %96 = bitcast { i8*, i32 }* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %96)
  %97 = extractvalue { i8*, i32 } %95, 0
  %98 = extractvalue { i8*, i32 } %95, 1
  %99 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 0
  store i8* %97, i8** %99, align 8
  %100 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1
  store i32 %98, i32* %100, align 8
  br label %91

101:                                              ; preds = %89
  %102 = bitcast %"error::NewInterpreterError"* %34 to i8*, !dbg !78501
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %102), !dbg !78501
  %103 = bitcast %"error::NewInterpreterError"* %35 to i8*, !dbg !78497
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %103), !dbg !78497
  %104 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %37 to i64*, !dbg !78499
  %105 = load i64, i64* %104, align 8, !dbg !78499, !range !2867
  %106 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %37 to i8*, !dbg !78499
  call void @llvm.lifetime.end.p0i8(i64 400, i8* %106), !dbg !78499
  br label %107, !dbg !78502

107:                                              ; preds = %303, %204, %184, %169, %101
  %108 = bitcast %"pyo3::ffi::PyConfig"* %38 to i8*, !dbg !78504
  call void @llvm.lifetime.end.p0i8(i64 392, i8* %108), !dbg !78504
  br label %371, !dbg !78505

109:                                              ; preds = %66
  %110 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %78, i32 0, i32 11, !dbg !78506
  %111 = bitcast %"std::option::Option<std::vec::Vec<std::ffi::OsString>>"* %110 to {}**, !dbg !78507
  %112 = load {}*, {}** %111, align 8, !dbg !78507
  %113 = icmp eq {}* %112, null, !dbg !78507
  %114 = select i1 %113, i64 0, i64 1, !dbg !78507
  %115 = icmp eq i64 %114, 1, !dbg !78507
  br i1 %115, label %116, label %124, !dbg !78507

116:                                              ; preds = %109
  %117 = bitcast %"std::option::Option<std::vec::Vec<std::ffi::OsString>>"* %110 to %"std::option::Option<std::vec::Vec<std::ffi::OsString>>::Some"*, !dbg !78508
  %118 = bitcast %"std::option::Option<std::vec::Vec<std::ffi::OsString>>::Some"* %117 to %"std::vec::Vec<std::ffi::OsString>"*, !dbg !78508
  store %"std::vec::Vec<std::ffi::OsString>"* %118, %"std::vec::Vec<std::ffi::OsString>"** %4, align 8, !dbg !78508
  call void @llvm.dbg.declare(metadata %"std::vec::Vec<std::ffi::OsString>"** %4, metadata !78464, metadata !DIExpression()), !dbg !78509
  %119 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %30 to i8*, !dbg !78510
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %119), !dbg !78510
  %120 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %29 to i8*, !dbg !78510
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %120), !dbg !78510
  %121 = call { [0 x %"std::ffi::OsString"]*, i64 } @_RNvXs8_NtCshECKBmVgND8_5alloc3vecINtB5_3VecNtNtNtCsahqMUB7vfgm_3std3ffi6os_str8OsStringENtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5derefCsijNWNMdNqoj_7pyembed(%"std::vec::Vec<std::ffi::OsString>"* noalias readonly align 8 dereferenceable(24) %118), !dbg !78511
  %122 = extractvalue { [0 x %"std::ffi::OsString"]*, i64 } %121, 0, !dbg !78511
  %123 = extractvalue { [0 x %"std::ffi::OsString"]*, i64 } %121, 1, !dbg !78511
  br label %127, !dbg !78511

124:                                              ; preds = %109
  br label %125, !dbg !78512

125:                                              ; preds = %137, %124
  %126 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !78513
  br label %178, !dbg !78513

127:                                              ; preds = %116
  call void @_RNvNtCsijNWNMdNqoj_7pyembed18interpreter_config8set_argv(%"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<(), error::NewInterpreterError>") dereferenceable(32) %29, %"pyo3::ffi::PyConfig"* noalias align 8 dereferenceable(392) %38, [0 x %"std::ffi::OsString"]* noalias nonnull readonly align 8 %122, i64 %123), !dbg !78510
  br label %128, !dbg !78510

128:                                              ; preds = %127
  invoke void @_RNvXsy_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultuNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops3try3Try11into_resultBP_(%"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<(), error::NewInterpreterError>") dereferenceable(32) %30, %"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture dereferenceable(32) %29)
          to label %129 unwind label %58, !dbg !78510

129:                                              ; preds = %128
  %130 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %29 to i8*, !dbg !78510
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %130), !dbg !78510
  %131 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %30 to i64*, !dbg !78514
  %132 = load i64, i64* %131, align 8, !dbg !78514, !range !29786
  %133 = sub i64 %132, 2, !dbg !78514
  %134 = icmp eq i64 %133, 0, !dbg !78514
  %135 = select i1 %134, i64 0, i64 1, !dbg !78514
  switch i64 %135, label %136 [
    i64 0, label %137
    i64 1, label %144
  ], !dbg !78514

136:                                              ; preds = %129
  unreachable, !dbg !78510

137:                                              ; preds = %129
  %138 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %30 to i64*, !dbg !78515
  %139 = load i64, i64* %138, align 8, !dbg !78515, !range !29786
  %140 = sub i64 %139, 2, !dbg !78515
  %141 = icmp eq i64 %140, 0, !dbg !78515
  %142 = select i1 %141, i64 0, i64 1, !dbg !78515
  %143 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %30 to i8*, !dbg !78515
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %143), !dbg !78515
  br label %125, !dbg !78512

144:                                              ; preds = %129
  %145 = bitcast %"error::NewInterpreterError"* %28 to i8*, !dbg !78514
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %145), !dbg !78514
  %146 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %30 to %"std::result::Result<(), error::NewInterpreterError>::Err"*, !dbg !78514
  %147 = bitcast %"std::result::Result<(), error::NewInterpreterError>::Err"* %146 to %"error::NewInterpreterError"*, !dbg !78514
  %148 = bitcast %"error::NewInterpreterError"* %28 to i8*, !dbg !78514
  %149 = bitcast %"error::NewInterpreterError"* %147 to i8*, !dbg !78514
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %148, i8* align 8 %149, i64 32, i1 false), !dbg !78514
  %150 = bitcast %"error::NewInterpreterError"* %27 to i8*, !dbg !78486
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %150), !dbg !78486
  %151 = bitcast %"error::NewInterpreterError"* %26 to i8*, !dbg !78486
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %151), !dbg !78486
  %152 = bitcast %"error::NewInterpreterError"* %26 to i8*, !dbg !78486
  %153 = bitcast %"error::NewInterpreterError"* %28 to i8*, !dbg !78486
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %152, i8* align 8 %153, i64 32, i1 false), !dbg !78486
  invoke void @_RNvXs2_NtCskrsM4FCwAVA_4core7convertNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorINtB5_4FromBy_E4fromBC_(%"error::NewInterpreterError"* noalias nocapture sret(%"error::NewInterpreterError") dereferenceable(32) %27, %"error::NewInterpreterError"* noalias nocapture dereferenceable(32) %26)
          to label %154 unwind label %162, !dbg !78486

154:                                              ; preds = %144
  %155 = bitcast %"error::NewInterpreterError"* %26 to i8*, !dbg !78486
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %155), !dbg !78486
  invoke void @_RNvXsy_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops3try3Try10from_errorB1M_(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %0, %"error::NewInterpreterError"* noalias nocapture dereferenceable(32) %27)
          to label %169 unwind label %162, !dbg !78516

156:                                              ; preds = %162
  %157 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %30 to i64*, !dbg !78515
  %158 = load i64, i64* %157, align 8, !dbg !78515, !range !29786
  %159 = sub i64 %158, 2, !dbg !78515
  %160 = icmp eq i64 %159, 0, !dbg !78515
  %161 = select i1 %160, i64 0, i64 1, !dbg !78515
  br label %50, !dbg !78515

162:                                              ; preds = %154, %144
  %163 = landingpad { i8*, i32 }
          cleanup
  %164 = bitcast { i8*, i32 }* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %164)
  %165 = extractvalue { i8*, i32 } %163, 0
  %166 = extractvalue { i8*, i32 } %163, 1
  %167 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 0
  store i8* %165, i8** %167, align 8
  %168 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1
  store i32 %166, i32* %168, align 8
  br label %156

169:                                              ; preds = %154
  %170 = bitcast %"error::NewInterpreterError"* %27 to i8*, !dbg !78516
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %170), !dbg !78516
  %171 = bitcast %"error::NewInterpreterError"* %28 to i8*, !dbg !78514
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %171), !dbg !78514
  %172 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %30 to i64*, !dbg !78515
  %173 = load i64, i64* %172, align 8, !dbg !78515, !range !29786
  %174 = sub i64 %173, 2, !dbg !78515
  %175 = icmp eq i64 %174, 0, !dbg !78515
  %176 = select i1 %175, i64 0, i64 1, !dbg !78515
  %177 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %30 to i8*, !dbg !78515
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %177), !dbg !78515
  br label %107, !dbg !78517

178:                                              ; preds = %125
  %179 = bitcast %"config::OxidizedPythonInterpreterConfig"* %126 to %"std::option::Option<std::path::PathBuf>"*, !dbg !78513
  %180 = call zeroext i1 @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE7is_noneCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %179), !dbg !78513
  br label %181, !dbg !78513

181:                                              ; preds = %178
  br i1 %180, label %184, label %182, !dbg !78519

182:                                              ; preds = %181
  %183 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !78520
  br label %197, !dbg !78520

184:                                              ; preds = %181
  %185 = bitcast %"error::NewInterpreterError"* %25 to i8*, !dbg !78521
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %185), !dbg !78521
  %186 = bitcast %"error::NewInterpreterError"* %25 to %"error::NewInterpreterError::Simple"*, !dbg !78521
  %187 = getelementptr inbounds %"error::NewInterpreterError::Simple", %"error::NewInterpreterError::Simple"* %186, i32 0, i32 1, !dbg !78521
  %188 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %187, i32 0, i32 0, !dbg !78521
  store [0 x i8]* bitcast (<{ [57 x i8] }>* @293 to [0 x i8]*), [0 x i8]** %188, align 8, !dbg !78521
  %189 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %187, i32 0, i32 1, !dbg !78521
  store i64 57, i64* %189, align 8, !dbg !78521
  %190 = bitcast %"error::NewInterpreterError"* %25 to i64*, !dbg !78521
  store i64 0, i64* %190, align 8, !dbg !78521
  %191 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err"*, !dbg !78522
  %192 = getelementptr inbounds %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err", %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err"* %191, i32 0, i32 1, !dbg !78522
  %193 = bitcast %"error::NewInterpreterError"* %192 to i8*, !dbg !78522
  %194 = bitcast %"error::NewInterpreterError"* %25 to i8*, !dbg !78522
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %193, i8* align 8 %194, i64 32, i1 false), !dbg !78522
  %195 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to i64*, !dbg !78522
  store i64 1, i64* %195, align 8, !dbg !78522
  %196 = bitcast %"error::NewInterpreterError"* %25 to i8*, !dbg !78523
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %196), !dbg !78523
  br label %107, !dbg !78517

197:                                              ; preds = %182
  %198 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %183, i32 0, i32 3, !dbg !78520
  %199 = call zeroext i1 @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE7is_noneCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %198), !dbg !78520
  br label %200, !dbg !78520

200:                                              ; preds = %197
  br i1 %199, label %204, label %201, !dbg !78524

201:                                              ; preds = %200
  %202 = bitcast %"std::path::PathBuf"** %23 to i8*, !dbg !78525
  call void @llvm.lifetime.start.p0i8(i64 8, i8* %202), !dbg !78525
  %203 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !78526
  br label %217, !dbg !78526

204:                                              ; preds = %200
  %205 = bitcast %"error::NewInterpreterError"* %24 to i8*, !dbg !78527
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %205), !dbg !78527
  %206 = bitcast %"error::NewInterpreterError"* %24 to %"error::NewInterpreterError::Simple"*, !dbg !78527
  %207 = getelementptr inbounds %"error::NewInterpreterError::Simple", %"error::NewInterpreterError::Simple"* %206, i32 0, i32 1, !dbg !78527
  %208 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %207, i32 0, i32 0, !dbg !78527
  store [0 x i8]* bitcast (<{ [45 x i8] }>* @294 to [0 x i8]*), [0 x i8]** %208, align 8, !dbg !78527
  %209 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %207, i32 0, i32 1, !dbg !78527
  store i64 45, i64* %209, align 8, !dbg !78527
  %210 = bitcast %"error::NewInterpreterError"* %24 to i64*, !dbg !78527
  store i64 0, i64* %210, align 8, !dbg !78527
  %211 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err"*, !dbg !78528
  %212 = getelementptr inbounds %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err", %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err"* %211, i32 0, i32 1, !dbg !78528
  %213 = bitcast %"error::NewInterpreterError"* %212 to i8*, !dbg !78528
  %214 = bitcast %"error::NewInterpreterError"* %24 to i8*, !dbg !78528
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %213, i8* align 8 %214, i64 32, i1 false), !dbg !78528
  %215 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to i64*, !dbg !78528
  store i64 1, i64* %215, align 8, !dbg !78528
  %216 = bitcast %"error::NewInterpreterError"* %24 to i8*, !dbg !78529
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %216), !dbg !78529
  br label %107, !dbg !78517

217:                                              ; preds = %201
  %218 = bitcast %"config::OxidizedPythonInterpreterConfig"* %203 to %"std::option::Option<std::path::PathBuf>"*, !dbg !78526
  %219 = call align 8 dereferenceable_or_null(24) i64* @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE6as_refCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %218), !dbg !78526
  br label %220, !dbg !78526

220:                                              ; preds = %217
  %221 = call align 8 dereferenceable(24) %"std::path::PathBuf"* @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionRNtNtCsahqMUB7vfgm_3std4path7PathBufE6unwrapCsijNWNMdNqoj_7pyembed(i64* noalias readonly align 8 dereferenceable_or_null(24) %219, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @296 to %"std::panic::Location"*)), !dbg !78526
  store %"std::path::PathBuf"* %221, %"std::path::PathBuf"** %23, align 8, !dbg !78526
  br label %222, !dbg !78526

222:                                              ; preds = %220
  %223 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !78530
  br label %224, !dbg !78530

224:                                              ; preds = %222
  %225 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %223, i32 0, i32 3, !dbg !78530
  %226 = call align 8 dereferenceable_or_null(24) i64* @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE6as_refCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %225), !dbg !78530
  br label %227, !dbg !78530

227:                                              ; preds = %224
  %228 = call align 8 dereferenceable(24) %"std::path::PathBuf"* @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionRNtNtCsahqMUB7vfgm_3std4path7PathBufE6unwrapCsijNWNMdNqoj_7pyembed(i64* noalias readonly align 8 dereferenceable_or_null(24) %226, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @297 to %"std::panic::Location"*)), !dbg !78530
  store %"std::path::PathBuf"* %228, %"std::path::PathBuf"** %3, align 8, !dbg !78530
  call void @llvm.dbg.declare(metadata %"std::path::PathBuf"** %3, metadata !78472, metadata !DIExpression()), !dbg !78531
  br label %229, !dbg !78530

229:                                              ; preds = %227
  %230 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !78532
  br label %231, !dbg !78532

231:                                              ; preds = %229
  %232 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %230, i32 0, i32 31, !dbg !78532
  %233 = load i8, i8* %232, align 2, !dbg !78532, !range !7200
  %234 = trunc i8 %233 to i1, !dbg !78532
  br i1 %234, label %236, label %235, !dbg !78533

235:                                              ; preds = %231
  br label %373, !dbg !78533

236:                                              ; preds = %231
  %237 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !78534
  br label %238, !dbg !78534

238:                                              ; preds = %236
  %239 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %237, i32 0, i32 5, !dbg !78534
  %240 = getelementptr inbounds %"python_packaging::interpreter::PythonInterpreterConfig", %"python_packaging::interpreter::PythonInterpreterConfig"* %239, i32 0, i32 25, !dbg !78534
  %241 = call zeroext i1 @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE7is_noneCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %240), !dbg !78534
  br label %242, !dbg !78534

242:                                              ; preds = %238
  br i1 %241, label %244, label %243, !dbg !78535

243:                                              ; preds = %242
  br label %305, !dbg !78535

244:                                              ; preds = %242
  %245 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %20 to i8*, !dbg !78536
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %245), !dbg !78536
  %246 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %19 to i8*, !dbg !78536
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %246), !dbg !78536
  %247 = getelementptr inbounds %"pyo3::ffi::PyConfig", %"pyo3::ffi::PyConfig"* %38, i32 0, i32 39, !dbg !78537
  %248 = load %"std::path::PathBuf"*, %"std::path::PathBuf"** %23, align 8, !dbg !78538, !nonnull !4
  %249 = call { %"std::path::Path"*, i64 } @"_ZN62_$LT$std..path..PathBuf$u20$as$u20$core..ops..deref..Deref$GT$5deref17he7f3f920ba1b2649E"(%"std::path::PathBuf"* noalias readonly align 8 dereferenceable(24) %248), !dbg !78538
  %250 = extractvalue { %"std::path::Path"*, i64 } %249, 0, !dbg !78538
  %251 = extractvalue { %"std::path::Path"*, i64 } %249, 1, !dbg !78538
  br label %252, !dbg !78538

252:                                              ; preds = %244
  call void @_RNvNtCsijNWNMdNqoj_7pyembed18interpreter_config27set_config_string_from_path(%"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<(), error::NewInterpreterError>") dereferenceable(32) %19, %"pyo3::ffi::PyConfig"* noalias readonly align 8 dereferenceable(392) %38, i32** noalias readonly align 8 dereferenceable(8) %247, %"std::path::Path"* noalias nonnull readonly align 1 %250, i64 %251, [0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [20 x i8] }>* @298 to [0 x i8]*), i64 20), !dbg !78536
  br label %253, !dbg !78536

253:                                              ; preds = %252
  invoke void @_RNvXsy_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultuNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops3try3Try11into_resultBP_(%"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<(), error::NewInterpreterError>") dereferenceable(32) %20, %"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture dereferenceable(32) %19)
          to label %254 unwind label %58, !dbg !78536

254:                                              ; preds = %253
  %255 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %19 to i8*, !dbg !78536
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %255), !dbg !78536
  %256 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %20 to i64*, !dbg !78539
  %257 = load i64, i64* %256, align 8, !dbg !78539, !range !29786
  %258 = sub i64 %257, 2, !dbg !78539
  %259 = icmp eq i64 %258, 0, !dbg !78539
  %260 = select i1 %259, i64 0, i64 1, !dbg !78539
  switch i64 %260, label %261 [
    i64 0, label %262
    i64 1, label %269
  ], !dbg !78539

261:                                              ; preds = %254
  unreachable, !dbg !78536

262:                                              ; preds = %254
  %263 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %20 to i64*, !dbg !78540
  %264 = load i64, i64* %263, align 8, !dbg !78540, !range !29786
  %265 = sub i64 %264, 2, !dbg !78540
  %266 = icmp eq i64 %265, 0, !dbg !78540
  %267 = select i1 %266, i64 0, i64 1, !dbg !78540
  %268 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %20 to i8*, !dbg !78540
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %268), !dbg !78540
  br label %305, !dbg !78535

269:                                              ; preds = %254
  %270 = bitcast %"error::NewInterpreterError"* %18 to i8*, !dbg !78539
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %270), !dbg !78539
  %271 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %20 to %"std::result::Result<(), error::NewInterpreterError>::Err"*, !dbg !78539
  %272 = bitcast %"std::result::Result<(), error::NewInterpreterError>::Err"* %271 to %"error::NewInterpreterError"*, !dbg !78539
  %273 = bitcast %"error::NewInterpreterError"* %18 to i8*, !dbg !78539
  %274 = bitcast %"error::NewInterpreterError"* %272 to i8*, !dbg !78539
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %273, i8* align 8 %274, i64 32, i1 false), !dbg !78539
  %275 = bitcast %"error::NewInterpreterError"* %17 to i8*, !dbg !78489
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %275), !dbg !78489
  %276 = bitcast %"error::NewInterpreterError"* %16 to i8*, !dbg !78489
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %276), !dbg !78489
  %277 = bitcast %"error::NewInterpreterError"* %16 to i8*, !dbg !78489
  %278 = bitcast %"error::NewInterpreterError"* %18 to i8*, !dbg !78489
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %277, i8* align 8 %278, i64 32, i1 false), !dbg !78489
  invoke void @_RNvXs2_NtCskrsM4FCwAVA_4core7convertNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorINtB5_4FromBy_E4fromBC_(%"error::NewInterpreterError"* noalias nocapture sret(%"error::NewInterpreterError") dereferenceable(32) %17, %"error::NewInterpreterError"* noalias nocapture dereferenceable(32) %16)
          to label %279 unwind label %287, !dbg !78489

279:                                              ; preds = %269
  %280 = bitcast %"error::NewInterpreterError"* %16 to i8*, !dbg !78489
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %280), !dbg !78489
  invoke void @_RNvXsy_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops3try3Try10from_errorB1M_(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %0, %"error::NewInterpreterError"* noalias nocapture dereferenceable(32) %17)
          to label %294 unwind label %287, !dbg !78541

281:                                              ; preds = %287
  %282 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %20 to i64*, !dbg !78540
  %283 = load i64, i64* %282, align 8, !dbg !78540, !range !29786
  %284 = sub i64 %283, 2, !dbg !78540
  %285 = icmp eq i64 %284, 0, !dbg !78540
  %286 = select i1 %285, i64 0, i64 1, !dbg !78540
  br label %50, !dbg !78540

287:                                              ; preds = %279, %269
  %288 = landingpad { i8*, i32 }
          cleanup
  %289 = bitcast { i8*, i32 }* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %289)
  %290 = extractvalue { i8*, i32 } %288, 0
  %291 = extractvalue { i8*, i32 } %288, 1
  %292 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 0
  store i8* %290, i8** %292, align 8
  %293 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1
  store i32 %291, i32* %293, align 8
  br label %281

294:                                              ; preds = %279
  %295 = bitcast %"error::NewInterpreterError"* %17 to i8*, !dbg !78541
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %295), !dbg !78541
  %296 = bitcast %"error::NewInterpreterError"* %18 to i8*, !dbg !78539
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %296), !dbg !78539
  %297 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %20 to i64*, !dbg !78540
  %298 = load i64, i64* %297, align 8, !dbg !78540, !range !29786
  %299 = sub i64 %298, 2, !dbg !78540
  %300 = icmp eq i64 %299, 0, !dbg !78540
  %301 = select i1 %300, i64 0, i64 1, !dbg !78540
  %302 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %20 to i8*, !dbg !78540
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %302), !dbg !78540
  br label %303, !dbg !78542

303:                                              ; preds = %362, %294
  %304 = bitcast %"std::path::PathBuf"** %23 to i8*, !dbg !78544
  call void @llvm.lifetime.end.p0i8(i64 8, i8* %304), !dbg !78544
  br label %107, !dbg !78517

305:                                              ; preds = %243, %262
  %306 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !78545
  br label %307, !dbg !78545

307:                                              ; preds = %305
  %308 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %306, i32 0, i32 5, !dbg !78545
  %309 = getelementptr inbounds %"python_packaging::interpreter::PythonInterpreterConfig", %"python_packaging::interpreter::PythonInterpreterConfig"* %308, i32 0, i32 19, !dbg !78545
  %310 = call zeroext i1 @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE7is_noneCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %309), !dbg !78545
  br label %311, !dbg !78545

311:                                              ; preds = %307
  br i1 %310, label %313, label %312, !dbg !78546

312:                                              ; preds = %311
  br label %372, !dbg !78546

313:                                              ; preds = %311
  %314 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %15 to i8*, !dbg !78547
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %314), !dbg !78547
  %315 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %14 to i8*, !dbg !78547
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %315), !dbg !78547
  %316 = getelementptr inbounds %"pyo3::ffi::PyConfig", %"pyo3::ffi::PyConfig"* %38, i32 0, i32 79, !dbg !78548
  %317 = call { %"std::path::Path"*, i64 } @"_ZN62_$LT$std..path..PathBuf$u20$as$u20$core..ops..deref..Deref$GT$5deref17he7f3f920ba1b2649E"(%"std::path::PathBuf"* noalias readonly align 8 dereferenceable(24) %228), !dbg !78549
  %318 = extractvalue { %"std::path::Path"*, i64 } %317, 0, !dbg !78549
  %319 = extractvalue { %"std::path::Path"*, i64 } %317, 1, !dbg !78549
  br label %320, !dbg !78549

320:                                              ; preds = %313
  call void @_RNvNtCsijNWNMdNqoj_7pyembed18interpreter_config27set_config_string_from_path(%"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<(), error::NewInterpreterError>") dereferenceable(32) %14, %"pyo3::ffi::PyConfig"* noalias readonly align 8 dereferenceable(392) %38, i32** noalias readonly align 8 dereferenceable(8) %316, %"std::path::Path"* noalias nonnull readonly align 1 %318, i64 %319, [0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [12 x i8] }>* @299 to [0 x i8]*), i64 12), !dbg !78547
  br label %321, !dbg !78547

321:                                              ; preds = %320
  invoke void @_RNvXsy_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultuNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops3try3Try11into_resultBP_(%"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<(), error::NewInterpreterError>") dereferenceable(32) %15, %"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture dereferenceable(32) %14)
          to label %322 unwind label %58, !dbg !78547

322:                                              ; preds = %321
  %323 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %14 to i8*, !dbg !78547
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %323), !dbg !78547
  %324 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %15 to i64*, !dbg !78550
  %325 = load i64, i64* %324, align 8, !dbg !78550, !range !29786
  %326 = sub i64 %325, 2, !dbg !78550
  %327 = icmp eq i64 %326, 0, !dbg !78550
  %328 = select i1 %327, i64 0, i64 1, !dbg !78550
  switch i64 %328, label %329 [
    i64 0, label %330
    i64 1, label %337
  ], !dbg !78550

329:                                              ; preds = %322
  unreachable, !dbg !78547

330:                                              ; preds = %322
  %331 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %15 to i64*, !dbg !78551
  %332 = load i64, i64* %331, align 8, !dbg !78551, !range !29786
  %333 = sub i64 %332, 2, !dbg !78551
  %334 = icmp eq i64 %333, 0, !dbg !78551
  %335 = select i1 %334, i64 0, i64 1, !dbg !78551
  %336 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %15 to i8*, !dbg !78551
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %336), !dbg !78551
  br label %372, !dbg !78546

337:                                              ; preds = %322
  %338 = bitcast %"error::NewInterpreterError"* %13 to i8*, !dbg !78550
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %338), !dbg !78550
  %339 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %15 to %"std::result::Result<(), error::NewInterpreterError>::Err"*, !dbg !78550
  %340 = bitcast %"std::result::Result<(), error::NewInterpreterError>::Err"* %339 to %"error::NewInterpreterError"*, !dbg !78550
  %341 = bitcast %"error::NewInterpreterError"* %13 to i8*, !dbg !78550
  %342 = bitcast %"error::NewInterpreterError"* %340 to i8*, !dbg !78550
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %341, i8* align 8 %342, i64 32, i1 false), !dbg !78550
  %343 = bitcast %"error::NewInterpreterError"* %12 to i8*, !dbg !78491
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %343), !dbg !78491
  %344 = bitcast %"error::NewInterpreterError"* %11 to i8*, !dbg !78491
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %344), !dbg !78491
  %345 = bitcast %"error::NewInterpreterError"* %11 to i8*, !dbg !78491
  %346 = bitcast %"error::NewInterpreterError"* %13 to i8*, !dbg !78491
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %345, i8* align 8 %346, i64 32, i1 false), !dbg !78491
  invoke void @_RNvXs2_NtCskrsM4FCwAVA_4core7convertNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorINtB5_4FromBy_E4fromBC_(%"error::NewInterpreterError"* noalias nocapture sret(%"error::NewInterpreterError") dereferenceable(32) %12, %"error::NewInterpreterError"* noalias nocapture dereferenceable(32) %11)
          to label %347 unwind label %355, !dbg !78491

347:                                              ; preds = %337
  %348 = bitcast %"error::NewInterpreterError"* %11 to i8*, !dbg !78491
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %348), !dbg !78491
  invoke void @_RNvXsy_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops3try3Try10from_errorB1M_(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %0, %"error::NewInterpreterError"* noalias nocapture dereferenceable(32) %12)
          to label %362 unwind label %355, !dbg !78552

349:                                              ; preds = %355
  %350 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %15 to i64*, !dbg !78551
  %351 = load i64, i64* %350, align 8, !dbg !78551, !range !29786
  %352 = sub i64 %351, 2, !dbg !78551
  %353 = icmp eq i64 %352, 0, !dbg !78551
  %354 = select i1 %353, i64 0, i64 1, !dbg !78551
  br label %50, !dbg !78551

355:                                              ; preds = %347, %337
  %356 = landingpad { i8*, i32 }
          cleanup
  %357 = bitcast { i8*, i32 }* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %357)
  %358 = extractvalue { i8*, i32 } %356, 0
  %359 = extractvalue { i8*, i32 } %356, 1
  %360 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 0
  store i8* %358, i8** %360, align 8
  %361 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1
  store i32 %359, i32* %361, align 8
  br label %349

362:                                              ; preds = %347
  %363 = bitcast %"error::NewInterpreterError"* %12 to i8*, !dbg !78552
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %363), !dbg !78552
  %364 = bitcast %"error::NewInterpreterError"* %13 to i8*, !dbg !78550
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %364), !dbg !78550
  %365 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %15 to i64*, !dbg !78551
  %366 = load i64, i64* %365, align 8, !dbg !78551, !range !29786
  %367 = sub i64 %366, 2, !dbg !78551
  %368 = icmp eq i64 %367, 0, !dbg !78551
  %369 = select i1 %368, i64 0, i64 1, !dbg !78551
  %370 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %15 to i8*, !dbg !78551
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %370), !dbg !78551
  br label %303, !dbg !78542

371:                                              ; preds = %373, %107
  ret void, !dbg !78505

372:                                              ; preds = %312, %330
  br label %373, !dbg !78533

373:                                              ; preds = %235, %372
  %374 = bitcast %"pyo3::ffi::PyConfig"* %10 to i8*, !dbg !78553
  call void @llvm.lifetime.start.p0i8(i64 392, i8* %374), !dbg !78553
  %375 = bitcast %"pyo3::ffi::PyConfig"* %10 to i8*, !dbg !78553
  %376 = bitcast %"pyo3::ffi::PyConfig"* %38 to i8*, !dbg !78553
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %375, i8* align 8 %376, i64 392, i1 false), !dbg !78553
  %377 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Ok"*, !dbg !78554
  %378 = getelementptr inbounds %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Ok", %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Ok"* %377, i32 0, i32 1, !dbg !78554
  %379 = bitcast %"pyo3::ffi::PyConfig"* %378 to i8*, !dbg !78554
  %380 = bitcast %"pyo3::ffi::PyConfig"* %10 to i8*, !dbg !78554
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %379, i8* align 8 %380, i64 392, i1 false), !dbg !78554
  %381 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to i64*, !dbg !78554
  store i64 0, i64* %381, align 8, !dbg !78554
  %382 = bitcast %"pyo3::ffi::PyConfig"* %10 to i8*, !dbg !78555
  call void @llvm.lifetime.end.p0i8(i64 392, i8* %382), !dbg !78555
  %383 = bitcast %"std::path::PathBuf"** %23 to i8*, !dbg !78544
  call void @llvm.lifetime.end.p0i8(i64 8, i8* %383), !dbg !78544
  %384 = bitcast %"pyo3::ffi::PyConfig"* %38 to i8*, !dbg !78504
  call void @llvm.lifetime.end.p0i8(i64 392, i8* %384), !dbg !78504
  br label %371, !dbg !78505
