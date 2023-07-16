
ldc2 -relocation-model=pic --shared lib.d -od=shared_lib_objects & ldc2 --lib lib.d -od=static_lib_objects
