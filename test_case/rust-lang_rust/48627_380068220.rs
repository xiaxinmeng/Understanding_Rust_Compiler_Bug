llvm
@_ZN4test5FUNCS17h4349eb3b1be595e8E = local_unnamed_addr constant { [0 x i8], {}*, [0 x i8], {}*, [0 x i8], {}*, [0 x i8] } { [0 x i8] undef, {}* null, [0 x i8] undef, {}* null, [0 x i8] undef, {}* null, [0 x i8] undef }, align 8

; ...

  %0 = load i8*, i8** bitcast ({ [0 x i8], {}*, [0 x i8], {}*, [0 x i8], {}*, [0 x i8] }* @_ZN4test5FUNCS17h4349eb3b1be595e8E to i8**), align 8
  %1 = icmp eq i8* %0, null
