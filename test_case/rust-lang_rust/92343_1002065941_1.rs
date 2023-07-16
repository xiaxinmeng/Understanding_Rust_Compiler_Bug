
{"app_name":"rustc","timestamp":"2021-12-28 00:25:54.00 +0200","app_version":"","slice_uuid":"5b5b970c-fb3b-32ee-9f3a-ff56e63570ea","build_version":"","platform":1,"share_with_app_devs":0,"is_first_party":1,"bug_type":"309","os_version":"macOS 12.1 (21C52)","incident_id":"8AA09F10-C7D2-482E-9154-2B94B2E872EC","name":"rustc"}
{
  "uptime" : 270000,
  "procLaunch" : "2021-12-28 00:25:52.2870 +0200",
  "procRole" : "Unspecified",
  "version" : 2,
  "userID" : 501,
  "deployVersion" : 210,
  "modelCode" : "Macmini8,1",
  "procStartAbsTime" : 271376805640792,
  "coalitionID" : 1428,
  "osVersion" : {
    "train" : "macOS 12.1",
    "build" : "21C52",
    "releaseType" : "User"
  },
  "captureTime" : "2021-12-28 00:25:54.3468 +0200",
  "incident" : "8AA09F10-C7D2-482E-9154-2B94B2E872EC",
  "bug_type" : "309",
  "pid" : 14668,
  "procExitAbsTime" : 271378864029788,
  "cpuType" : "X86-64",
  "procName" : "rustc",
  "procPath" : "\/Volumes\/VOLUME\/*\/rustc",
  "parentProc" : "Exited process",
  "parentPid" : 14664,
  "coalitionName" : "com.googlecode.iterm2",
  "crashReporterKey" : "C628BF39-24D9-BCC4-4BE5-1FE4CA9E81E1",
  "responsiblePid" : 40005,
  "responsibleProc" : "iTerm2",
  "bridgeVersion" : {"build":"19P647","train":"6.1"},
  "sip" : "enabled",
  "isCorpse" : 1,
  "exception" : {"codes":"0x0000000000000000, 0x0000000000000000","rawCodes":[0,0],"type":"EXC_CRASH","signal":"SIGABRT"},
  "asi" : {"libsystem_c.dylib":["abort() called"],"libsystem_malloc.dylib":["rustc(14668,0x70000730e000) malloc: *** error for object 0x600007360e00: pointer being realloc'd was not allocated"],"dyld":["dyld4 config: DYLD_LIBRARY_PATH=\/Volumes\/Development\/Arduino\/rust-avr-nightly-builder\/build\/rust\/build\/x86_64-apple-darwin\/stage1\/lib"]},
  "extMods" : {"caller":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"system":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"targeted":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"warnings":0},
  "faultingThread" : 1,
  "threads" : [{"id":2287682,"queue":"com.apple.main-thread","frames":[{"imageOffset":8714,"symbol":"__ulock_wait","symbolLocation":10,"imageIndex":0},{"imageOffset":31853,"symbol":"_pthread_join","symbolLocation":362,"imageIndex":1},{"imageOffset":226448,"symbol":"std::sys::unix::thread::Thread::join::hfafbd51722f7d476","symbolLocation":16,"imageIndex":2},{"imageOffset":1209690,"symbol":"_RNvMs7_NtCsh2iSufJsRi7_3std6threadINtB5_10JoinHandleINtNtCsasXF9FuNFFy_4core6result6ResultuNtCsdjh14Z7klAl_12rustc_errors13ErrorReportedEE4joinCskBROs5LPUyG_12rustc_driver","symbolLocation":42,"imageIndex":3},{"imageOffset":781401,"symbol":"_RINvNtCslf2ReciObD5_15rustc_interface4util51setup_callbacks_and_run_in_thread_pool_with_globalsNCINvNtB4_9interface12run_compilerINtNtCsasXF9FuNFFy_4core6result6ResultuNtCsdjh14Z7klAl_12rustc_errors13ErrorReportedENCNvCskBROs5LPUyG_12rustc_driver12run_compilers_0E0B23_EB3u_","symbolLocation":409,"imageIndex":3},{"imageOffset":720859,"symbol":"_RNvMs_CskBROs5LPUyG_12rustc_driverNtB4_11RunCompiler3run","symbolLocation":7211,"imageIndex":3},{"imageOffset":846062,"symbol":"_RNvXsn_NtNtCsasXF9FuNFFy_4core5panic11unwind_safeINtB5_16AssertUnwindSafeNCNvCskBROs5LPUyG_12rustc_driver4main0EINtNtNtB9_3ops8function6FnOnceuE9call_onceB1d_","symbolLocation":174,"imageIndex":3},{"imageOffset":752043,"symbol":"_RNvCskBROs5LPUyG_12rustc_driver4main","symbolLocation":203,"imageIndex":3},{"imageOffset":15790,"symbol":"_RNvCshzTDRxvkZ7X_10rustc_main4main","symbolLocation":14,"imageIndex":4},{"imageOffset":15846,"symbol":"_RINvNtNtCsh2iSufJsRi7_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECshzTDRxvkZ7X_10rustc_main","symbolLocation":6,"imageIndex":4},{"imageOffset":15884,"symbol":"_RNCINvNtCsh2iSufJsRi7_3std2rt10lang_startuE0CshzTDRxvkZ7X_10rustc_main","symbolLocation":12,"imageIndex":4},{"imageOffset":91189,"symbol":"std::rt::lang_start_internal::h5d782eef0744dacf","symbolLocation":37,"imageIndex":2},{"imageOffset":15833,"symbol":"main","symbolLocation":41,"imageIndex":4},{"imageOffset":15764,"symbol":"start","symbolLocation":52,"imageIndex":4}]},{"triggered":true,"id":2287686,"name":"rustc","threadState":{"r13":{"value":80},"rax":{"value":0},"rflags":{"value":582},"cpu":{"value":0},"r14":{"value":6},"rsi":{"value":6},"r8":{"value":0},"cr2":{"value":1407047392
37896},"rdx":{"value":0},"r10":{"value":0},"r9":{"value":0},"r15":{"value":22},"rbx":{"value":123145422954496},"trap":{"value":133},"err":{"value":33554760},"r11":{"value":582},"rip":{"value":140703639822610,"matchesCrashFrame":1},"rbp":{"value":123145422893856},"rsp":{"value":123145422893816},"r12":{"value":4355},"rcx":{"value":123145422893816},"flavor":"x86_THREAD_STATE","rdi":{"value":4355}},"frames":[{"imageOffset":28946,"symbol":"__pthread_kill","symbolLocation":10,"imageIndex":0},{"imageOffset":25108,"symbol":"pthread_kill","symbolLocation":263,"imageIndex":1},{"imageOffset":531728,"symbol":"abort","symbolLocation":123,"imageIndex":5},{"imageOffset":62434,"symbol":"malloc_vreport","symbolLocation":548,"imageIndex":6},{"imageOffset":75245,"symbol":"malloc_report","symbolLocation":151,"imageIndex":6},{"imageOffset":13634,"symbol":"realloc","symbolLocation":328,"imageIndex":6},{"imageOffset":59467364,"symbol":"_RINvNtCs9mNDKRZk0IV_5alloc7raw_vec11finish_growNtNtB4_5alloc6GlobalECsetNeWcxqv12_12rustc_expand","symbolLocation":52,"imageIndex":3},{"imageOffset":59468829,"symbol":"_RNvMs_NtCs9mNDKRZk0IV_5alloc7raw_vecINtB4_6RawVecNtNtCsetNeWcxqv12_12rustc_expand3mbe9TokenTreeE16reserve_for_pushBP_","symbolLocation":125,"imageIndex":3},{"imageOffset":58907730,"symbol":"_RNvNtNtCsetNeWcxqv12_12rustc_expand3mbe6quoted5parse","symbolLocation":4002,"imageIndex":3},{"imageOffset":58904110,"symbol":"_RNvNtNtCsetNeWcxqv12_12rustc_expand3mbe6quoted5parse","symbolLocation":382,"imageIndex":3},{"imageOffset":59089488,"symbol":"_RINvXs0_NtNtNtCsasXF9FuNFFy_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtNtCsetNeWcxqv12_12rustc_expand3mbe12macro_parser10NamedMatchENCNvNtB1r_11macro_rules25compile_declarative_macros_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3i_8for_each4callNtB1r_9TokenTreeNCNvXs_NtNtCs9mNDKRZk0IV_5alloc3vec11spec_extendINtB4K_3VecB4l_EINtB4I_10SpecExtendB4l_BN_E11spec_extend0E0EB1t_","symbolLocation":336,"imageIndex":3},{"imageOffset":59314247,"symbol":"_RNvXNtNtCs9mNDKRZk0IV_5alloc3vec14spec_from_iterINtB4_3VecNtNtCsetNeWcxqv12_12rustc_expand3mbe9TokenTreeEINtB2_12SpecFromIterBU_INtNtNtNtCsasXF9FuNFFy_4core4iter8adapters3map3MapINtNtNtB2b_5slice4iter4IterNtNtBW_12macro_parser10NamedMatchENCNvNtBW_11macro_rules25compile_declarative_macros_0EE9from_iterBY_","symbolLocation":247,"imageIndex":3},{"imageOffset":59702867,"symbol":"_RNvNtNtCsetNeWcxqv12_12rustc_expand3mbe11macro_rules25compile_declarative_macro","symbolLocation":2451,"imageIndex":3},{"imageOffset":41452116,"symbol":"_RNvMs_NtCsfI4xnekMpqO_13rustc_resolve6macrosNtB6_8Resolver13compile_macro","symbolLocation":68,"imageIndex":3},{"imageOffset":41170919,"symbol":"_RNvMs3_NtCsfI4xnekMpqO_13rustc_resolve19build_reduced_graphNtB5_24BuildReducedGraphVisitor12define_macro","symbolLocation":295,"imageIndex":3},{"imageOffset":41175176,"symbol":"_RNvXs4_NtCsfI4xnekMpqO_13rustc_resolve19build_reduced_graphNtB5_24BuildReducedGraphVisitorNtNtCs9cCHMvxMcfg_9rustc_ast5visit7Visitor10visit_item","symbolLocation":56,"imageIndex":3},{"imageOffset":41654172,"symbol":"_RINvNtCs9cCHMvxMcfg_9rustc_ast5visit9walk_itemNtNtCsfI4xnekMpqO_13rustc_resolve19build_reduced_graph24BuildReducedGraphVisitorEBM_","symbolLocation":572,"imageIndex":3},{"imageOffset":41183932,"symbol":"_RNvXs4_NtCsfI4xnekMpqO_13rustc_resolve19build_reduced_graphNtB5_24BuildReducedGraphVisitorNtNtCs9cCHMvxMcfg_9rustc_ast5visit7Visitor10visit_item","symbolLocation":8812,"imageIndex":3},{"imageOffset":41654172,"symbol":"_RINvNtCs9cCHMvxMcfg_9rustc_ast5visit9walk_itemNtNtCsfI4xnekMpqO_13rustc_resolve19build_reduced_graph24BuildReducedGraphVisitorEBM_","symbolLocation":572,"imageIndex":3},{"imageOffset":41183932,"symbol
":"_RNvXs4_NtCsfI4xnekMpqO_13rustc_resolve19build_reduced_graphNtB5_24BuildReducedGraphVisitorNtNtCs9cCHMvxMcfg_9rustc_ast5visit7Visitor10visit_item","symbolLocation":8812,"imageIndex":3},{"imageOffset":41982701,"symbol":"_RINvMs6_NtCsetNeWcxqv12_12rustc_expand6expandNtB6_11AstFragment10visit_withNtNtCsfI4xnekMpqO_13rustc_resolve19build_reduced_graph24BuildReducedGraphVisitorEB1f_","symbolLocation":685,"imageIndex":3},{"imageOffset":41407943,"symbol":"_RNvXNtCsfI4xnekMpqO_13rustc_resolve6macrosNtB4_8ResolverNtNtCsetNeWcxqv12_12rustc_expand4base14ResolverExpand36visit_ast_fragment_with_placeholders","symbolLocation":487,"imageIndex":3},{"imageOffset":58815683,"symbol":"_RNvMs1_NtCsetNeWcxqv12_12rustc_expand6expandNtB5_13MacroExpander19collect_invocations","symbolLocation":707,"imageIndex":3},{"imageOffset":58795456,"symbol":"_RNvMs1_NtCsetNeWcxqv12_12rustc_expand6expandNtB5_13MacroExpander21fully_expand_fragment","symbolLocation":256,"imageIndex":3},{"imageOffset":58794722,"symbol":"_RNvMs1_NtCsetNeWcxqv12_12rustc_expand6expandNtB5_13MacroExpander12expand_crate","symbolLocation":1186,"imageIndex":3},{"imageOffset":2611788,"symbol":"_RINvMNtCs3NuVQBhYW7C_13rustc_session5utilsNtNtB5_7session7Session4timeINtNtCsasXF9FuNFFy_4core6result6ResultNtNtCs9cCHMvxMcfg_9rustc_ast3ast5CrateNtCsdjh14Z7klAl_12rustc_errors13ErrorReportedENCNvNtCslf2ReciObD5_15rustc_interface6passes20configure_and_expands_0EB3a_","symbolLocation":1116,"imageIndex":3},{"imageOffset":2037712,"symbol":"_RNvNtCslf2ReciObD5_15rustc_interface6passes20configure_and_expand","symbolLocation":528,"imageIndex":3},{"imageOffset":2061780,"symbol":"_RNvMs0_NtCslf2ReciObD5_15rustc_interface7queriesNtB5_7Queries9expansion","symbolLocation":836,"imageIndex":3},{"imageOffset":849599,"symbol":"_RINvMs2_NtCslf2ReciObD5_15rustc_interface7queriesNtNtB8_9interface8Compiler5enterNCNCNvCskBROs5LPUyG_12rustc_driver12run_compilers_0s0_0INtNtCsasXF9FuNFFy_4core6result6ResultINtNtB2f_6option6OptionNtB6_6LinkerENtCsdjh14Z7klAl_12rustc_errors13ErrorReportedEEB1n_","symbolLocation":1327,"imageIndex":3},{"imageOffset":805000,"symbol":"_RINvCskTEkA9M7v6o_10rustc_span15with_source_mapINtNtCsasXF9FuNFFy_4core6result6ResultuNtCsdjh14Z7klAl_12rustc_errors13ErrorReportedENCINvNtCslf2ReciObD5_15rustc_interface9interface23create_compiler_and_runBJ_NCNvCskBROs5LPUyG_12rustc_driver12run_compilers_0Es_0EB3o_","symbolLocation":376,"imageIndex":3},{"imageOffset":854095,"symbol":"_RINvMs_Cs7KVLOe8Wn3E_10scoped_tlsINtB5_9ScopedKeyNtCskTEkA9M7v6o_10rustc_span14SessionGlobalsE3setNCNCINvNtCslf2ReciObD5_15rustc_interface4util51setup_callbacks_and_run_in_thread_pool_with_globalsNCINvNtB1H_9interface12run_compilerINtNtCsasXF9FuNFFy_4core6result6ResultuNtCsdjh14Z7klAl_12rustc_errors13ErrorReportedENCNvCskBROs5LPUyG_12rustc_driver12run_compilers_0E0B3H_E00B3H_EB58_","symbolLocation":1311,"imageIndex":3},{"imageOffset":840658,"symbol":"_RINvNtNtCsh2iSufJsRi7_3std10sys_common9backtrace28___rust_begin_short_backtraceNCINvNtCslf2ReciObD5_15rustc_interface4util51setup_callbacks_and_run_in_thread_pool_with_globalsNCINvNtB1m_9interface12run_compilerINtNtCsasXF9FuNFFy_4core6result6ResultuNtCsdjh14Z7klAl_12rustc_errors13ErrorReportedENCNvCskBROs5LPUyG_12rustc_driver12run_compilers_0E0B3m_E0B3m_EB4N_","symbolLocation":146,"imageIndex":3},{"imageOffset":1208533,"symbol":"_RNSNvYNCINvMNtCsh2iSufJsRi7_3std6threadNtBa_7Builder15spawn_uncheckedNCINvNtCslf2ReciObD5_15rustc_interface4util51setup_callbacks_and_run_in_thread_pool_with_globalsNCINvNtB1c_9interface12run_compilerINtNtCsasXF9FuNFFy_4core6result6ResultuNtCsdjh14Z7klAl_12rustc_errors13ErrorReportedENCNvCskBROs5LPUyG_12rustc_driver12run_compilers_0E0B3c_E0B3c_Es_0INtNtNtB3h_3ops8function6FnOn
ceuE9call_once6vtableB4D_","symbolLocation":165,"imageIndex":3},{"imageOffset":226247,"symbol":"std::sys::unix::thread::Thread::new::thread_start::h78d8518f6f52ffd3","symbolLocation":39,"imageIndex":2},{"imageOffset":25844,"symbol":"_pthread_start","symbolLocation":125,"imageIndex":1},{"imageOffset":8207,"symbol":"thread_start","symbolLocation":15,"imageIndex":1}]}],
  "usedImages" : [
  {
    "source" : "P",
    "arch" : "x86_64",
    "base" : 140703639793664,
    "size" : 225280,
    "uuid" : "5aa1e5be-b5b8-3a02-9885-a8c99e0ca378",
    "path" : "\/usr\/lib\/system\/libsystem_kernel.dylib",
    "name" : "libsystem_kernel.dylib"
  },
  {
    "source" : "P",
    "arch" : "x86_64",
    "base" : 140703640018944,
    "size" : 49152,
    "uuid" : "6c7561b4-4b92-3f45-921e-abe669299844",
    "path" : "\/usr\/lib\/system\/libsystem_pthread.dylib",
    "name" : "libsystem_pthread.dylib"
  },
  {
    "source" : "P",
    "arch" : "x86_64",
    "base" : 4346793984,
    "size" : 1114112,
    "uuid" : "a2488d1b-6b9b-3b55-9a2e-67d19d2b04ff",
    "path" : "\/Volumes\/VOLUME\/*\/libstd-de515994d461a049.dylib",
    "name" : "libstd-de515994d461a049.dylib"
  },
  {
    "source" : "P",
    "arch" : "x86_64",
    "base" : 4525465600,
    "size" : 91684864,
    "uuid" : "80271f80-5ed6-3a63-b128-9ea2934c17d9",
    "path" : "\/Volumes\/VOLUME\/*\/librustc_driver-9d1dd63cf081cf4c.dylib",
    "name" : "librustc_driver-9d1dd63cf081cf4c.dylib"
  },
  {
    "source" : "P",
    "arch" : "x86_64",
    "base" : 4340334592,
    "size" : 16384,
    "uuid" : "5b5b970c-fb3b-32ee-9f3a-ff56e63570ea",
    "path" : "\/Volumes\/VOLUME\/*\/rustc",
    "name" : "rustc"
  },
  {
    "source" : "P",
    "arch" : "x86_64",
    "base" : 140703638777856,
    "size" : 561152,
    "uuid" : "e58814cc-dcb7-35a5-badc-e367ed3ac207",
    "path" : "\/usr\/lib\/system\/libsystem_c.dylib",
    "name" : "libsystem_c.dylib"
  },
  {
    "source" : "P",
    "arch" : "x86_64",
    "base" : 140703638056960,
    "size" : 180224,
    "uuid" : "ca14841a-43f1-3f98-ad36-4a7c5c2447da",
    "path" : "\/usr\/lib\/system\/libsystem_malloc.dylib",
    "name" : "libsystem_malloc.dylib"
  }
],
  "sharedCache" : {
  "base" : 140703636791296,
  "size" : 15216738304,
  "uuid" : "40432a03-88d3-305f-9c0c-e7549e71d927"
},
  "vmSummary" : "ReadOnly portion of Libraries: Total=257.8M resident=0K(0%) swapped_out_or_unallocated=257.8M(100%)\nWritable regions: Total=1.1G written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=1.1G(100%)\n\n                                VIRTUAL   REGION \nREGION TYPE                        SIZE    COUNT (non-coalesced) \n===========                     =======  ======= \nKernel Alloc Once                    8K        1 \nMALLOC                           753.6M      189 \nMALLOC guard page                   16K        4 \nMALLOC_NANO (reserved)           384.0M        1         reserved VM address space (unallocated)\nSTACK GUARD                          4K        1 \nStack                             16.0M        3 \nStack Guard                       56.0M        1 \nVM_ALLOCATE                         24K        6 \nVM_ALLOCATE (reserved)             256K        2         reserved VM address space (unallocated)\n__DATA                            7865K       45 \n__DATA_CONST                       296K       37 \n__DATA_DIRTY                        58K       22 \n__LINKEDIT                       163.5M        7 \n__OBJC_RO                         81.8M        1 \n__OBJC_RW                         3136K        2 \n__TEXT                            94.3M       49 \ndyld private memory               1024K        1 \nshared memory                        8K        1 \n===========                     =======  ======= \nTOTAL                              1.5G      373 \nTOTAL, minus reserved VM space     1.1G      373 \n",
  "legacyInfo" : {
  "threadTriggered" : {
    "name" : "rustc"
  }
},
  "trialInfo" : {
  "rollouts" : [
    {
      "rolloutId" : "60da5e84ab0ca017dace9abf",
      "factorPackIds" : {

      },
      "deploymentId" : 240000008
    },
    {
      "rolloutId" : "607844aa04477260f58a8077",
      "factorPackIds" : {
        "SIRI_MORPHUN_ASSETS" : "6103050cbfe6dc472e1c982a"
      },
      "deploymentId" : 240000066
    },
    {
      "rolloutId" : "602ad4dac86151000cf27e46",
      "factorPackIds" : {
        "SIRI_DICTATION_ASSETS" : "6193d03f2171a2330e561dfc"
      },
      "deploymentId" : 240000290
    },
    {
      "rolloutId" : "5ffde50ce2aacd000d47a95f",
      "factorPackIds" : {

      },
      "deploymentId" : 240000090
    },
    {
      "rolloutId" : "601d9415f79519000ccd4b69",
      "factorPackIds" : {
        "SIRI_TEXT_TO_SPEECH" : "61c11dcd2cb6041dc630dc63"
      },
      "deploymentId" : 240000356
    },
    {
      "rolloutId" : "5fc94383418129005b4e9ae0",
      "factorPackIds" : {

      },
      "deploymentId" : 240000196
    }
  ],
  "experiments" : [

  ]
}
}
