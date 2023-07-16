
echo '_ZN33_$LT$alloc..arc..Arc$LT$T$GT$$GT$9drop_slow17h9ed4cba808ccbc62E' | c++filt
<alloc::arc::Arc<T>>::drop_slow

echo '_ZN33_$LT$alloc..arc..Arc$LT$T$GT$$GT$9drop_slow17h9ed4cba808ccbc62E' | cppfilt 
_$LT$alloc..arc..Arc$LT$T$GT$$GT$::drop_slow::h9ed4cba808ccbc62
