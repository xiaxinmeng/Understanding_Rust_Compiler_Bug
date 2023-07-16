
%agg.tmp = alloca %struct.foo, align 8
call void @make_foo(%struct.foo* sret %agg.tmp)
call void @take_foo(%struct.foo* byval align 8 %agg.tmp)
