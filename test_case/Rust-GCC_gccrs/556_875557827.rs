
> gccrs block_expr1.rs      -O1     -lm  -o ./block_expr1.exe  -fno-pie -fno-PIE -static
> readelf -h block_expr1.exe |grep Type:
  Type:                              EXEC (Executable file)
> gccrs block_expr1.rs     -O1     -lm  -o ./block_expr1.exe  -fno-pie -fno-PIE
> readelf -h block_expr1.exe |grep Type:
  Type:                              DYN (Shared object file)
> gccrs block_expr1.rs     -O1     -lm  -o ./block_expr1.exe
> readelf -h block_expr1.exe |grep Type:
  Type:                              DYN (Shared object file)
