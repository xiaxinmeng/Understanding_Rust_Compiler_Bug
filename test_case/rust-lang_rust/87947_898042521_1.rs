
*** IR Dump After Instrument function entry/exit with calls to e.g. mcount() (pre inlining) ***
; Function Attrs: nonlazybind uwtable
define void @_RNvXs_NtCsijNWNMdNqoj_7pyembed18interpreter_configRNtNtB6_6config39ResolvedOxidizedPythonInterpreterConfigINtNtCskrsM4FCwAVA_4core7convert7TryIntoNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigE8try_into(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %0, %"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !80111 {
  %3 = alloca %"std::path::PathBuf"*, align 8
  %4 = alloca %"std::vec::Vec<std::ffi::OsString>"*, align 8
  %5 = alloca { i8*, i32 }, align 8
  %6 = alloca {}, align 1
  %7 = alloca {}, align 1
  %8 = alloca {}, align 1
  %9 = alloca %"config::ResolvedOxidizedPythonInterpreterConfig"*, align 8
  %10 = alloca %"pyo3::ffi::PyConfig", align 8
  %11 = alloca %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err", align 8
  %12 = alloca %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err", align 8
  %13 = alloca %"std::result::Result<(), error::NewInterpreterError>", align 8
  %14 = alloca %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>", align 8
  %15 = alloca %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err", align 8
  %16 = alloca %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err", align 8
  %17 = alloca %"std::result::Result<(), error::NewInterpreterError>", align 8
  %18 = alloca %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>", align 8
  %19 = alloca {}, align 1
  %20 = alloca {}, align 1
  %21 = alloca %"std::path::PathBuf"*, align 8
  %22 = alloca %"error::NewInterpreterError", align 8
  %23 = alloca %"error::NewInterpreterError", align 8
  %24 = alloca %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err", align 8
  %25 = alloca %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err", align 8
  %26 = alloca %"std::result::Result<(), error::NewInterpreterError>", align 8
  %27 = alloca %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>", align 8
  %28 = alloca {}, align 1
  %29 = alloca %"pyo3::ffi::PyConfig", align 8
  %30 = alloca %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err", align 8
  %31 = alloca %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err", align 8
  %32 = alloca %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>", align 8
  %33 = alloca %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>", align 8
  %34 = alloca %"pyo3::ffi::PyConfig", align 8
  store %"config::ResolvedOxidizedPythonInterpreterConfig"* %1, %"config::ResolvedOxidizedPythonInterpreterConfig"** %9, align 8
  call void @llvm.dbg.declare(metadata %"config::ResolvedOxidizedPythonInterpreterConfig"** %9, metadata !80115, metadata !DIExpression()), !dbg !80140
  call void @llvm.dbg.declare(metadata %"pyo3::ffi::PyConfig"* %34, metadata !80116, metadata !DIExpression()), !dbg !80141
  call void @llvm.dbg.declare(metadata %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %31, metadata !80118, metadata !DIExpression()), !dbg !80142
  call void @llvm.dbg.declare(metadata %"pyo3::ffi::PyConfig"* %29, metadata !80120, metadata !DIExpression()), !dbg !80143
  call void @llvm.dbg.declare(metadata %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %25, metadata !80124, metadata !DIExpression()), !dbg !80144
  call void @llvm.dbg.declare(metadata {}* %8, metadata !80126, metadata !DIExpression()), !dbg !80145
  call void @llvm.dbg.declare(metadata %"std::path::PathBuf"** %21, metadata !80128, metadata !DIExpression()), !dbg !80146
  call void @llvm.dbg.declare(metadata %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %16, metadata !80132, metadata !DIExpression()), !dbg !80147
  call void @llvm.dbg.declare(metadata {}* %7, metadata !80134, metadata !DIExpression()), !dbg !80148
  call void @llvm.dbg.declare(metadata %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %12, metadata !80136, metadata !DIExpression()), !dbg !80149
  call void @llvm.dbg.declare(metadata {}* %6, metadata !80138, metadata !DIExpression()), !dbg !80150
  %35 = bitcast %"pyo3::ffi::PyConfig"* %34 to i8*, !dbg !80151
  call void @llvm.lifetime.start.p0i8(i64 392, i8* %35), !dbg !80151
  %36 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>"* %33 to i8*, !dbg !80152
  call void @llvm.lifetime.start.p0i8(i64 400, i8* %36), !dbg !80152
  %37 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %32 to i8*, !dbg !80152
  call void @llvm.lifetime.start.p0i8(i64 400, i8* %37), !dbg !80152
  %38 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !80153
  br label %39, !dbg !80153

39:                                               ; preds = %2
  %40 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %38, i32 0, i32 5, !dbg !80154
  call void @_RNvNtCsijNWNMdNqoj_7pyembed18interpreter_config38python_interpreter_config_to_py_config(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %32, %"python_packaging::interpreter::PythonInterpreterConfig"* noalias readonly align 8 dereferenceable(560) %40), !dbg !80152
  br label %41, !dbg !80152

41:                                               ; preds = %39
  invoke void @_RNvXsz_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops9try_trait3Try6branchB1M_(%"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>"* noalias nocapture sret(%"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>") dereferenceable(400) %33, %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture dereferenceable(400) %32)
          to label %42 unwind label %54, !dbg !80152

42:                                               ; preds = %41
  %43 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %32 to i8*, !dbg !80152
  call void @llvm.lifetime.end.p0i8(i64 400, i8* %43), !dbg !80152
  %44 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>"* %33 to i64*, !dbg !80155
  %45 = load i64, i64* %44, align 8, !dbg !80155, !range !2867
  switch i64 %45, label %61 [
    i64 0, label %62
    i64 1, label %75
  ], !dbg !80155

46:                                               ; preds = %342, %277, %155, %90, %54
  %47 = bitcast { i8*, i32 }* %5 to i8**, !dbg !80156
  %48 = load i8*, i8** %47, align 8, !dbg !80156
  %49 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1, !dbg !80156
  %50 = load i32, i32* %49, align 8, !dbg !80156
  %51 = bitcast { i8*, i32 }* %5 to i8*, !dbg !80156
  call void @llvm.lifetime.end.p0i8(i64 16, i8* %51), !dbg !80156
  %52 = insertvalue { i8*, i32 } undef, i8* %48, 0, !dbg !80156
  %53 = insertvalue { i8*, i32 } %52, i32 %50, 1, !dbg !80156
  resume { i8*, i32 } %53, !dbg !80156

54:                                               ; preds = %308, %243, %121, %41
  %55 = landingpad { i8*, i32 }
          cleanup
  %56 = bitcast { i8*, i32 }* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %56)
  %57 = extractvalue { i8*, i32 } %55, 0
  %58 = extractvalue { i8*, i32 } %55, 1
  %59 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 0
  store i8* %57, i8** %59, align 8
  %60 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1
  store i32 %58, i32* %60, align 8
  br label %46

61:                                               ; preds = %42
  unreachable, !dbg !80152

62:                                               ; preds = %42
  %63 = bitcast %"pyo3::ffi::PyConfig"* %29 to i8*, !dbg !80152
  call void @llvm.lifetime.start.p0i8(i64 392, i8* %63), !dbg !80152
  %64 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>"* %33 to %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>::Continue"*, !dbg !80152
  %65 = getelementptr inbounds %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>::Continue", %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>::Continue"* %64, i32 0, i32 1, !dbg !80152
  %66 = bitcast %"pyo3::ffi::PyConfig"* %29 to i8*, !dbg !80152
  %67 = bitcast %"pyo3::ffi::PyConfig"* %65 to i8*, !dbg !80152
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %66, i8* align 8 %67, i64 392, i1 false), !dbg !80152
  %68 = bitcast %"pyo3::ffi::PyConfig"* %34 to i8*, !dbg !80143
  %69 = bitcast %"pyo3::ffi::PyConfig"* %29 to i8*, !dbg !80143
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %68, i8* align 8 %69, i64 392, i1 false), !dbg !80143
  %70 = bitcast %"pyo3::ffi::PyConfig"* %29 to i8*, !dbg !80155
  call void @llvm.lifetime.end.p0i8(i64 392, i8* %70), !dbg !80155
  %71 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>"* %33 to i64*, !dbg !80157
  %72 = load i64, i64* %71, align 8, !dbg !80157, !range !2867
  %73 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>"* %33 to i8*, !dbg !80157
  call void @llvm.lifetime.end.p0i8(i64 400, i8* %73), !dbg !80157
  %74 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !80158
  br label %102, !dbg !80158

75:                                               ; preds = %42
  %76 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %31 to i8*, !dbg !80155
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %76), !dbg !80155
  %77 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>"* %33 to %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>::Break"*, !dbg !80155
  %78 = getelementptr inbounds %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>::Break", %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>::Break"* %77, i32 0, i32 1, !dbg !80155
  %79 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %31 to i8*, !dbg !80155
  %80 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %78 to i8*, !dbg !80155
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %79, i8* align 8 %80, i64 32, i1 false), !dbg !80155
  %81 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %30 to i8*, !dbg !80142
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %81), !dbg !80142
  %82 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %30 to i8*, !dbg !80142
  %83 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %31 to i8*, !dbg !80142
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %82, i8* align 8 %83, i64 32, i1 false), !dbg !80142
  invoke void @_RNvXsA_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorEINtNtNtB7_3ops9try_trait12FromResidualIBy_NtNtB7_7convert10InfallibleB1I_EE13from_residualB1M_(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %0, %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* noalias nocapture dereferenceable(32) %30)
          to label %84 unwind label %93, !dbg !80159

84:                                               ; preds = %75
  %85 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %30 to i8*, !dbg !80159
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %85), !dbg !80159
  %86 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %31 to i8*, !dbg !80155
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %86), !dbg !80155
  %87 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>"* %33 to i64*, !dbg !80157
  %88 = load i64, i64* %87, align 8, !dbg !80157, !range !2867
  %89 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>"* %33 to i8*, !dbg !80157
  call void @llvm.lifetime.end.p0i8(i64 400, i8* %89), !dbg !80157
  br label %100, !dbg !80160

90:                                               ; preds = %93
  %91 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>, pyo3::ffi::PyConfig>"* %33 to i64*, !dbg !80157
  %92 = load i64, i64* %91, align 8, !dbg !80157, !range !2867
  br label %46, !dbg !80157

93:                                               ; preds = %75
  %94 = landingpad { i8*, i32 }
          cleanup
  %95 = bitcast { i8*, i32 }* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %95)
  %96 = extractvalue { i8*, i32 } %94, 0
  %97 = extractvalue { i8*, i32 } %94, 1
  %98 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 0
  store i8* %96, i8** %98, align 8
  %99 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1
  store i32 %97, i32* %99, align 8
  br label %90

100:                                              ; preds = %290, %194, %174, %146, %84
  %101 = bitcast %"pyo3::ffi::PyConfig"* %34 to i8*, !dbg !80162
  call void @llvm.lifetime.end.p0i8(i64 392, i8* %101), !dbg !80162
  br label %355, !dbg !80163

102:                                              ; preds = %62
  %103 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %74, i32 0, i32 11, !dbg !80164
  %104 = bitcast %"std::option::Option<std::vec::Vec<std::ffi::OsString>>"* %103 to {}**, !dbg !80165
  %105 = load {}*, {}** %104, align 8, !dbg !80165
  %106 = icmp eq {}* %105, null, !dbg !80165
  %107 = select i1 %106, i64 0, i64 1, !dbg !80165
  %108 = icmp eq i64 %107, 1, !dbg !80165
  br i1 %108, label %109, label %117, !dbg !80165

109:                                              ; preds = %102
  %110 = bitcast %"std::option::Option<std::vec::Vec<std::ffi::OsString>>"* %103 to %"std::option::Option<std::vec::Vec<std::ffi::OsString>>::Some"*, !dbg !80166
  %111 = bitcast %"std::option::Option<std::vec::Vec<std::ffi::OsString>>::Some"* %110 to %"std::vec::Vec<std::ffi::OsString>"*, !dbg !80166
  store %"std::vec::Vec<std::ffi::OsString>"* %111, %"std::vec::Vec<std::ffi::OsString>"** %4, align 8, !dbg !80166
  call void @llvm.dbg.declare(metadata %"std::vec::Vec<std::ffi::OsString>"** %4, metadata !80122, metadata !DIExpression()), !dbg !80167
  %112 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %27 to i8*, !dbg !80168
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %112), !dbg !80168
  %113 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %26 to i8*, !dbg !80168
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %113), !dbg !80168
  %114 = call { [0 x %"std::ffi::OsString"]*, i64 } @_RNvXs8_NtCshECKBmVgND8_5alloc3vecINtB5_3VecNtNtNtCsahqMUB7vfgm_3std3ffi6os_str8OsStringENtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5derefCsijNWNMdNqoj_7pyembed(%"std::vec::Vec<std::ffi::OsString>"* noalias readonly align 8 dereferenceable(24) %111), !dbg !80169
  %115 = extractvalue { [0 x %"std::ffi::OsString"]*, i64 } %114, 0, !dbg !80169
  %116 = extractvalue { [0 x %"std::ffi::OsString"]*, i64 } %114, 1, !dbg !80169
  br label %120, !dbg !80169

117:                                              ; preds = %102
  br label %118, !dbg !80170

118:                                              ; preds = %130, %117
  %119 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !80171
  br label %168, !dbg !80171

120:                                              ; preds = %109
  call void @_RNvNtCsijNWNMdNqoj_7pyembed18interpreter_config8set_argv(%"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<(), error::NewInterpreterError>") dereferenceable(32) %26, %"pyo3::ffi::PyConfig"* noalias align 8 dereferenceable(392) %34, [0 x %"std::ffi::OsString"]* noalias nonnull readonly align 8 %115, i64 %116), !dbg !80168
  br label %121, !dbg !80168

121:                                              ; preds = %120
  invoke void @_RNvXsz_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultuNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops9try_trait3Try6branchBP_(%"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* noalias nocapture sret(%"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>") dereferenceable(32) %27, %"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture dereferenceable(32) %26)
          to label %122 unwind label %54, !dbg !80168

122:                                              ; preds = %121
  %123 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %26 to i8*, !dbg !80168
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %123), !dbg !80168
  %124 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %27 to i64*, !dbg !80172
  %125 = load i64, i64* %124, align 8, !dbg !80172, !range !29490
  %126 = sub i64 %125, 2, !dbg !80172
  %127 = icmp eq i64 %126, 0, !dbg !80172
  %128 = select i1 %127, i64 0, i64 1, !dbg !80172
  switch i64 %128, label %129 [
    i64 0, label %130
    i64 1, label %137
  ], !dbg !80172

129:                                              ; preds = %122
  unreachable, !dbg !80168

130:                                              ; preds = %122
  %131 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %27 to i64*, !dbg !80173
  %132 = load i64, i64* %131, align 8, !dbg !80173, !range !29490
  %133 = sub i64 %132, 2, !dbg !80173
  %134 = icmp eq i64 %133, 0, !dbg !80173
  %135 = select i1 %134, i64 0, i64 1, !dbg !80173
  %136 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %27 to i8*, !dbg !80173
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %136), !dbg !80173
  br label %118, !dbg !80170

137:                                              ; preds = %122
  %138 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %25 to i8*, !dbg !80172
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %138), !dbg !80172
  %139 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %27 to %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>::Break"*, !dbg !80172
  %140 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>::Break"* %139 to %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"*, !dbg !80172
  %141 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %25 to i8*, !dbg !80172
  %142 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %140 to i8*, !dbg !80172
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %141, i8* align 8 %142, i64 32, i1 false), !dbg !80172
  %143 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %24 to i8*, !dbg !80144
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %143), !dbg !80144
  %144 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %24 to i8*, !dbg !80144
  %145 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %25 to i8*, !dbg !80144
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %144, i8* align 8 %145, i64 32, i1 false), !dbg !80144
  invoke void @_RNvXsA_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorEINtNtNtB7_3ops9try_trait12FromResidualIBy_NtNtB7_7convert10InfallibleB1I_EE13from_residualB1M_(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %0, %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* noalias nocapture dereferenceable(32) %24)
          to label %146 unwind label %161, !dbg !80174

146:                                              ; preds = %137
  %147 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %24 to i8*, !dbg !80174
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %147), !dbg !80174
  %148 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %25 to i8*, !dbg !80172
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %148), !dbg !80172
  %149 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %27 to i64*, !dbg !80173
  %150 = load i64, i64* %149, align 8, !dbg !80173, !range !29490
  %151 = sub i64 %150, 2, !dbg !80173
  %152 = icmp eq i64 %151, 0, !dbg !80173
  %153 = select i1 %152, i64 0, i64 1, !dbg !80173
  %154 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %27 to i8*, !dbg !80173
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %154), !dbg !80173
  br label %100, !dbg !80175

155:                                              ; preds = %161
  %156 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %27 to i64*, !dbg !80173
  %157 = load i64, i64* %156, align 8, !dbg !80173, !range !29490
  %158 = sub i64 %157, 2, !dbg !80173
  %159 = icmp eq i64 %158, 0, !dbg !80173
  %160 = select i1 %159, i64 0, i64 1, !dbg !80173
  br label %46, !dbg !80173

161:                                              ; preds = %137
  %162 = landingpad { i8*, i32 }
          cleanup
  %163 = bitcast { i8*, i32 }* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %163)
  %164 = extractvalue { i8*, i32 } %162, 0
  %165 = extractvalue { i8*, i32 } %162, 1
  %166 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 0
  store i8* %164, i8** %166, align 8
  %167 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1
  store i32 %165, i32* %167, align 8
  br label %155

168:                                              ; preds = %118
  %169 = bitcast %"config::OxidizedPythonInterpreterConfig"* %119 to %"std::option::Option<std::path::PathBuf>"*, !dbg !80171
  %170 = call zeroext i1 @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE7is_noneCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %169), !dbg !80171
  br label %171, !dbg !80171

171:                                              ; preds = %168
  br i1 %170, label %174, label %172, !dbg !80177

172:                                              ; preds = %171
  %173 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !80178
  br label %187, !dbg !80178

174:                                              ; preds = %171
  %175 = bitcast %"error::NewInterpreterError"* %23 to i8*, !dbg !80179
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %175), !dbg !80179
  %176 = bitcast %"error::NewInterpreterError"* %23 to %"error::NewInterpreterError::Simple"*, !dbg !80179
  %177 = getelementptr inbounds %"error::NewInterpreterError::Simple", %"error::NewInterpreterError::Simple"* %176, i32 0, i32 1, !dbg !80179
  %178 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %177, i32 0, i32 0, !dbg !80179
  store [0 x i8]* bitcast (<{ [57 x i8] }>* @296 to [0 x i8]*), [0 x i8]** %178, align 8, !dbg !80179
  %179 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %177, i32 0, i32 1, !dbg !80179
  store i64 57, i64* %179, align 8, !dbg !80179
  %180 = bitcast %"error::NewInterpreterError"* %23 to i64*, !dbg !80179
  store i64 0, i64* %180, align 8, !dbg !80179
  %181 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err"*, !dbg !80180
  %182 = getelementptr inbounds %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err", %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err"* %181, i32 0, i32 1, !dbg !80180
  %183 = bitcast %"error::NewInterpreterError"* %182 to i8*, !dbg !80180
  %184 = bitcast %"error::NewInterpreterError"* %23 to i8*, !dbg !80180
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %183, i8* align 8 %184, i64 32, i1 false), !dbg !80180
  %185 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to i64*, !dbg !80180
  store i64 1, i64* %185, align 8, !dbg !80180
  %186 = bitcast %"error::NewInterpreterError"* %23 to i8*, !dbg !80181
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %186), !dbg !80181
  br label %100, !dbg !80175

187:                                              ; preds = %172
  %188 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %173, i32 0, i32 3, !dbg !80178
  %189 = call zeroext i1 @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE7is_noneCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %188), !dbg !80178
  br label %190, !dbg !80178

190:                                              ; preds = %187
  br i1 %189, label %194, label %191, !dbg !80182

191:                                              ; preds = %190
  %192 = bitcast %"std::path::PathBuf"** %21 to i8*, !dbg !80183
  call void @llvm.lifetime.start.p0i8(i64 8, i8* %192), !dbg !80183
  %193 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !80184
  br label %207, !dbg !80184

194:                                              ; preds = %190
  %195 = bitcast %"error::NewInterpreterError"* %22 to i8*, !dbg !80185
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %195), !dbg !80185
  %196 = bitcast %"error::NewInterpreterError"* %22 to %"error::NewInterpreterError::Simple"*, !dbg !80185
  %197 = getelementptr inbounds %"error::NewInterpreterError::Simple", %"error::NewInterpreterError::Simple"* %196, i32 0, i32 1, !dbg !80185
  %198 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %197, i32 0, i32 0, !dbg !80185
  store [0 x i8]* bitcast (<{ [45 x i8] }>* @297 to [0 x i8]*), [0 x i8]** %198, align 8, !dbg !80185
  %199 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %197, i32 0, i32 1, !dbg !80185
  store i64 45, i64* %199, align 8, !dbg !80185
  %200 = bitcast %"error::NewInterpreterError"* %22 to i64*, !dbg !80185
  store i64 0, i64* %200, align 8, !dbg !80185
  %201 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err"*, !dbg !80186
  %202 = getelementptr inbounds %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err", %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Err"* %201, i32 0, i32 1, !dbg !80186
  %203 = bitcast %"error::NewInterpreterError"* %202 to i8*, !dbg !80186
  %204 = bitcast %"error::NewInterpreterError"* %22 to i8*, !dbg !80186
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %203, i8* align 8 %204, i64 32, i1 false), !dbg !80186
  %205 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to i64*, !dbg !80186
  store i64 1, i64* %205, align 8, !dbg !80186
  %206 = bitcast %"error::NewInterpreterError"* %22 to i8*, !dbg !80187
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %206), !dbg !80187
  br label %100, !dbg !80175

207:                                              ; preds = %191
  %208 = bitcast %"config::OxidizedPythonInterpreterConfig"* %193 to %"std::option::Option<std::path::PathBuf>"*, !dbg !80184
  %209 = call align 8 dereferenceable_or_null(24) i64* @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE6as_refCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %208), !dbg !80184
  br label %210, !dbg !80184

210:                                              ; preds = %207
  %211 = call align 8 dereferenceable(24) %"std::path::PathBuf"* @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionRNtNtCsahqMUB7vfgm_3std4path7PathBufE6unwrapCsijNWNMdNqoj_7pyembed(i64* noalias readonly align 8 dereferenceable_or_null(24) %209, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @299 to %"std::panic::Location"*)), !dbg !80184
  store %"std::path::PathBuf"* %211, %"std::path::PathBuf"** %21, align 8, !dbg !80184
  br label %212, !dbg !80184

212:                                              ; preds = %210
  %213 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !80188
  br label %214, !dbg !80188

214:                                              ; preds = %212
  %215 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %213, i32 0, i32 3, !dbg !80188
  %216 = call align 8 dereferenceable_or_null(24) i64* @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE6as_refCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %215), !dbg !80188
  br label %217, !dbg !80188

217:                                              ; preds = %214
  %218 = call align 8 dereferenceable(24) %"std::path::PathBuf"* @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionRNtNtCsahqMUB7vfgm_3std4path7PathBufE6unwrapCsijNWNMdNqoj_7pyembed(i64* noalias readonly align 8 dereferenceable_or_null(24) %216, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @300 to %"std::panic::Location"*)), !dbg !80188
  store %"std::path::PathBuf"* %218, %"std::path::PathBuf"** %3, align 8, !dbg !80188
  call void @llvm.dbg.declare(metadata %"std::path::PathBuf"** %3, metadata !80130, metadata !DIExpression()), !dbg !80189
  br label %219, !dbg !80188

219:                                              ; preds = %217
  %220 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !80190
  br label %221, !dbg !80190

221:                                              ; preds = %219
  %222 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %220, i32 0, i32 31, !dbg !80190
  %223 = load i8, i8* %222, align 2, !dbg !80190, !range !6865
  %224 = trunc i8 %223 to i1, !dbg !80190
  br i1 %224, label %226, label %225, !dbg !80191

225:                                              ; preds = %221
  br label %357, !dbg !80191

226:                                              ; preds = %221
  %227 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !80192
  br label %228, !dbg !80192

228:                                              ; preds = %226
  %229 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %227, i32 0, i32 5, !dbg !80192
  %230 = getelementptr inbounds %"python_packaging::interpreter::PythonInterpreterConfig", %"python_packaging::interpreter::PythonInterpreterConfig"* %229, i32 0, i32 25, !dbg !80192
  %231 = call zeroext i1 @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE7is_noneCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %230), !dbg !80192
  br label %232, !dbg !80192

232:                                              ; preds = %228
  br i1 %231, label %234, label %233, !dbg !80193

233:                                              ; preds = %232
  br label %292, !dbg !80193

234:                                              ; preds = %232
  %235 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %18 to i8*, !dbg !80194
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %235), !dbg !80194
  %236 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %17 to i8*, !dbg !80194
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %236), !dbg !80194
  %237 = getelementptr inbounds %"pyo3::ffi::PyConfig", %"pyo3::ffi::PyConfig"* %34, i32 0, i32 39, !dbg !80195
  %238 = load %"std::path::PathBuf"*, %"std::path::PathBuf"** %21, align 8, !dbg !80196, !nonnull !4
  %239 = call { %"std::path::Path"*, i64 } @"_ZN62_$LT$std..path..PathBuf$u20$as$u20$core..ops..deref..Deref$GT$5deref17he7f3f920ba1b2649E"(%"std::path::PathBuf"* noalias readonly align 8 dereferenceable(24) %238), !dbg !80196
  %240 = extractvalue { %"std::path::Path"*, i64 } %239, 0, !dbg !80196
  %241 = extractvalue { %"std::path::Path"*, i64 } %239, 1, !dbg !80196
  br label %242, !dbg !80196

242:                                              ; preds = %234
  call void @_RNvNtCsijNWNMdNqoj_7pyembed18interpreter_config27set_config_string_from_path(%"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<(), error::NewInterpreterError>") dereferenceable(32) %17, %"pyo3::ffi::PyConfig"* noalias readonly align 8 dereferenceable(392) %34, i32** noalias readonly align 8 dereferenceable(8) %237, %"std::path::Path"* noalias nonnull readonly align 1 %240, i64 %241, [0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [20 x i8] }>* @301 to [0 x i8]*), i64 20), !dbg !80194
  br label %243, !dbg !80194

243:                                              ; preds = %242
  invoke void @_RNvXsz_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultuNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops9try_trait3Try6branchBP_(%"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* noalias nocapture sret(%"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>") dereferenceable(32) %18, %"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture dereferenceable(32) %17)
          to label %244 unwind label %54, !dbg !80194

244:                                              ; preds = %243
  %245 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %17 to i8*, !dbg !80194
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %245), !dbg !80194
  %246 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %18 to i64*, !dbg !80197
  %247 = load i64, i64* %246, align 8, !dbg !80197, !range !29490
  %248 = sub i64 %247, 2, !dbg !80197
  %249 = icmp eq i64 %248, 0, !dbg !80197
  %250 = select i1 %249, i64 0, i64 1, !dbg !80197
  switch i64 %250, label %251 [
    i64 0, label %252
    i64 1, label %259
  ], !dbg !80197

251:                                              ; preds = %244
  unreachable, !dbg !80194

252:                                              ; preds = %244
  %253 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %18 to i64*, !dbg !80198
  %254 = load i64, i64* %253, align 8, !dbg !80198, !range !29490
  %255 = sub i64 %254, 2, !dbg !80198
  %256 = icmp eq i64 %255, 0, !dbg !80198
  %257 = select i1 %256, i64 0, i64 1, !dbg !80198
  %258 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %18 to i8*, !dbg !80198
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %258), !dbg !80198
  br label %292, !dbg !80193

259:                                              ; preds = %244
  %260 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %16 to i8*, !dbg !80197
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %260), !dbg !80197
  %261 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %18 to %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>::Break"*, !dbg !80197
  %262 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>::Break"* %261 to %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"*, !dbg !80197
  %263 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %16 to i8*, !dbg !80197
  %264 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %262 to i8*, !dbg !80197
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %263, i8* align 8 %264, i64 32, i1 false), !dbg !80197
  %265 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %15 to i8*, !dbg !80147
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %265), !dbg !80147
  %266 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %15 to i8*, !dbg !80147
  %267 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %16 to i8*, !dbg !80147
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %266, i8* align 8 %267, i64 32, i1 false), !dbg !80147
  invoke void @_RNvXsA_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorEINtNtNtB7_3ops9try_trait12FromResidualIBy_NtNtB7_7convert10InfallibleB1I_EE13from_residualB1M_(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %0, %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* noalias nocapture dereferenceable(32) %15)
          to label %268 unwind label %283, !dbg !80199

268:                                              ; preds = %259
  %269 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %15 to i8*, !dbg !80199
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %269), !dbg !80199
  %270 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %16 to i8*, !dbg !80197
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %270), !dbg !80197
  %271 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %18 to i64*, !dbg !80198
  %272 = load i64, i64* %271, align 8, !dbg !80198, !range !29490
  %273 = sub i64 %272, 2, !dbg !80198
  %274 = icmp eq i64 %273, 0, !dbg !80198
  %275 = select i1 %274, i64 0, i64 1, !dbg !80198
  %276 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %18 to i8*, !dbg !80198
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %276), !dbg !80198
  br label %290, !dbg !80200

277:                                              ; preds = %283
  %278 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %18 to i64*, !dbg !80198
  %279 = load i64, i64* %278, align 8, !dbg !80198, !range !29490
  %280 = sub i64 %279, 2, !dbg !80198
  %281 = icmp eq i64 %280, 0, !dbg !80198
  %282 = select i1 %281, i64 0, i64 1, !dbg !80198
  br label %46, !dbg !80198

283:                                              ; preds = %259
  %284 = landingpad { i8*, i32 }
          cleanup
  %285 = bitcast { i8*, i32 }* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %285)
  %286 = extractvalue { i8*, i32 } %284, 0
  %287 = extractvalue { i8*, i32 } %284, 1
  %288 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 0
  store i8* %286, i8** %288, align 8
  %289 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1
  store i32 %287, i32* %289, align 8
  br label %277

290:                                              ; preds = %333, %268
  %291 = bitcast %"std::path::PathBuf"** %21 to i8*, !dbg !80202
  call void @llvm.lifetime.end.p0i8(i64 8, i8* %291), !dbg !80202
  br label %100, !dbg !80175

292:                                              ; preds = %233, %252
  %293 = call align 8 dereferenceable(776) %"config::OxidizedPythonInterpreterConfig"* @_RNvXs1_NtCsijNWNMdNqoj_7pyembed6configNtB5_39ResolvedOxidizedPythonInterpreterConfigNtNtNtCskrsM4FCwAVA_4core3ops5deref5Deref5deref(%"config::ResolvedOxidizedPythonInterpreterConfig"* noalias readonly align 8 dereferenceable(776) %1), !dbg !80203
  br label %294, !dbg !80203

294:                                              ; preds = %292
  %295 = getelementptr inbounds %"config::OxidizedPythonInterpreterConfig", %"config::OxidizedPythonInterpreterConfig"* %293, i32 0, i32 5, !dbg !80203
  %296 = getelementptr inbounds %"python_packaging::interpreter::PythonInterpreterConfig", %"python_packaging::interpreter::PythonInterpreterConfig"* %295, i32 0, i32 19, !dbg !80203
  %297 = call zeroext i1 @_RNvMNtCskrsM4FCwAVA_4core6optionINtB2_6OptionNtNtCsahqMUB7vfgm_3std4path7PathBufE7is_noneCsijNWNMdNqoj_7pyembed(%"std::option::Option<std::path::PathBuf>"* noalias readonly align 8 dereferenceable(24) %296), !dbg !80203
  br label %298, !dbg !80203

298:                                              ; preds = %294
  br i1 %297, label %300, label %299, !dbg !80204

299:                                              ; preds = %298
  br label %356, !dbg !80204

300:                                              ; preds = %298
  %301 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %14 to i8*, !dbg !80205
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %301), !dbg !80205
  %302 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %13 to i8*, !dbg !80205
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %302), !dbg !80205
  %303 = getelementptr inbounds %"pyo3::ffi::PyConfig", %"pyo3::ffi::PyConfig"* %34, i32 0, i32 79, !dbg !80206
  %304 = call { %"std::path::Path"*, i64 } @"_ZN62_$LT$std..path..PathBuf$u20$as$u20$core..ops..deref..Deref$GT$5deref17he7f3f920ba1b2649E"(%"std::path::PathBuf"* noalias readonly align 8 dereferenceable(24) %218), !dbg !80207
  %305 = extractvalue { %"std::path::Path"*, i64 } %304, 0, !dbg !80207
  %306 = extractvalue { %"std::path::Path"*, i64 } %304, 1, !dbg !80207
  br label %307, !dbg !80207

307:                                              ; preds = %300
  call void @_RNvNtCsijNWNMdNqoj_7pyembed18interpreter_config27set_config_string_from_path(%"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<(), error::NewInterpreterError>") dereferenceable(32) %13, %"pyo3::ffi::PyConfig"* noalias readonly align 8 dereferenceable(392) %34, i32** noalias readonly align 8 dereferenceable(8) %303, %"std::path::Path"* noalias nonnull readonly align 1 %305, i64 %306, [0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [12 x i8] }>* @302 to [0 x i8]*), i64 12), !dbg !80205
  br label %308, !dbg !80205

308:                                              ; preds = %307
  invoke void @_RNvXsz_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultuNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorENtNtNtB7_3ops9try_trait3Try6branchBP_(%"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* noalias nocapture sret(%"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>") dereferenceable(32) %14, %"std::result::Result<(), error::NewInterpreterError>"* noalias nocapture dereferenceable(32) %13)
          to label %309 unwind label %54, !dbg !80205

309:                                              ; preds = %308
  %310 = bitcast %"std::result::Result<(), error::NewInterpreterError>"* %13 to i8*, !dbg !80205
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %310), !dbg !80205
  %311 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %14 to i64*, !dbg !80208
  %312 = load i64, i64* %311, align 8, !dbg !80208, !range !29490
  %313 = sub i64 %312, 2, !dbg !80208
  %314 = icmp eq i64 %313, 0, !dbg !80208
  %315 = select i1 %314, i64 0, i64 1, !dbg !80208
  switch i64 %315, label %316 [
    i64 0, label %317
    i64 1, label %324
  ], !dbg !80208

316:                                              ; preds = %309
  unreachable, !dbg !80205

317:                                              ; preds = %309
  %318 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %14 to i64*, !dbg !80209
  %319 = load i64, i64* %318, align 8, !dbg !80209, !range !29490
  %320 = sub i64 %319, 2, !dbg !80209
  %321 = icmp eq i64 %320, 0, !dbg !80209
  %322 = select i1 %321, i64 0, i64 1, !dbg !80209
  %323 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %14 to i8*, !dbg !80209
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %323), !dbg !80209
  br label %356, !dbg !80204

324:                                              ; preds = %309
  %325 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %12 to i8*, !dbg !80208
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %325), !dbg !80208
  %326 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %14 to %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>::Break"*, !dbg !80208
  %327 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>::Break"* %326 to %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"*, !dbg !80208
  %328 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %12 to i8*, !dbg !80208
  %329 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %327 to i8*, !dbg !80208
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %328, i8* align 8 %329, i64 32, i1 false), !dbg !80208
  %330 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %11 to i8*, !dbg !80149
  call void @llvm.lifetime.start.p0i8(i64 32, i8* %330), !dbg !80149
  %331 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %11 to i8*, !dbg !80149
  %332 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %12 to i8*, !dbg !80149
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %331, i8* align 8 %332, i64 32, i1 false), !dbg !80149
  invoke void @_RNvXsA_NtCskrsM4FCwAVA_4core6resultINtB5_6ResultNtNtNtNtCs9hgvHLNfJdN_4pyo33ffi7cpython10initconfig8PyConfigNtNtCsijNWNMdNqoj_7pyembed5error19NewInterpreterErrorEINtNtNtB7_3ops9try_trait12FromResidualIBy_NtNtB7_7convert10InfallibleB1I_EE13from_residualB1M_(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* noalias nocapture sret(%"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>") dereferenceable(400) %0, %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* noalias nocapture dereferenceable(32) %11)
          to label %333 unwind label %348, !dbg !80210

333:                                              ; preds = %324
  %334 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %11 to i8*, !dbg !80210
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %334), !dbg !80210
  %335 = bitcast %"std::result::Result<std::convert::Infallible, error::NewInterpreterError>::Err"* %12 to i8*, !dbg !80208
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %335), !dbg !80208
  %336 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %14 to i64*, !dbg !80209
  %337 = load i64, i64* %336, align 8, !dbg !80209, !range !29490
  %338 = sub i64 %337, 2, !dbg !80209
  %339 = icmp eq i64 %338, 0, !dbg !80209
  %340 = select i1 %339, i64 0, i64 1, !dbg !80209
  %341 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %14 to i8*, !dbg !80209
  call void @llvm.lifetime.end.p0i8(i64 32, i8* %341), !dbg !80209
  br label %290, !dbg !80200

342:                                              ; preds = %348
  %343 = bitcast %"std::ops::ControlFlow<std::result::Result<std::convert::Infallible, error::NewInterpreterError>>"* %14 to i64*, !dbg !80209
  %344 = load i64, i64* %343, align 8, !dbg !80209, !range !29490
  %345 = sub i64 %344, 2, !dbg !80209
  %346 = icmp eq i64 %345, 0, !dbg !80209
  %347 = select i1 %346, i64 0, i64 1, !dbg !80209
  br label %46, !dbg !80209

348:                                              ; preds = %324
  %349 = landingpad { i8*, i32 }
          cleanup
  %350 = bitcast { i8*, i32 }* %5 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* %350)
  %351 = extractvalue { i8*, i32 } %349, 0
  %352 = extractvalue { i8*, i32 } %349, 1
  %353 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 0
  store i8* %351, i8** %353, align 8
  %354 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %5, i32 0, i32 1
  store i32 %352, i32* %354, align 8
  br label %342

355:                                              ; preds = %357, %100
  ret void, !dbg !80163

356:                                              ; preds = %299, %317
  br label %357, !dbg !80191

357:                                              ; preds = %225, %356
  %358 = bitcast %"pyo3::ffi::PyConfig"* %10 to i8*, !dbg !80211
  call void @llvm.lifetime.start.p0i8(i64 392, i8* %358), !dbg !80211
  %359 = bitcast %"pyo3::ffi::PyConfig"* %10 to i8*, !dbg !80211
  %360 = bitcast %"pyo3::ffi::PyConfig"* %34 to i8*, !dbg !80211
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %359, i8* align 8 %360, i64 392, i1 false), !dbg !80211
  %361 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Ok"*, !dbg !80212
  %362 = getelementptr inbounds %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Ok", %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>::Ok"* %361, i32 0, i32 1, !dbg !80212
  %363 = bitcast %"pyo3::ffi::PyConfig"* %362 to i8*, !dbg !80212
  %364 = bitcast %"pyo3::ffi::PyConfig"* %10 to i8*, !dbg !80212
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %363, i8* align 8 %364, i64 392, i1 false), !dbg !80212
  %365 = bitcast %"std::result::Result<pyo3::ffi::PyConfig, error::NewInterpreterError>"* %0 to i64*, !dbg !80212
  store i64 0, i64* %365, align 8, !dbg !80212
  %366 = bitcast %"pyo3::ffi::PyConfig"* %10 to i8*, !dbg !80213
  call void @llvm.lifetime.end.p0i8(i64 392, i8* %366), !dbg !80213
  %367 = bitcast %"std::path::PathBuf"** %21 to i8*, !dbg !80202
  call void @llvm.lifetime.end.p0i8(i64 8, i8* %367), !dbg !80202
  %368 = bitcast %"pyo3::ffi::PyConfig"* %34 to i8*, !dbg !80162
  call void @llvm.lifetime.end.p0i8(i64 392, i8* %368), !dbg !80162
  br label %355, !dbg !80163
}
