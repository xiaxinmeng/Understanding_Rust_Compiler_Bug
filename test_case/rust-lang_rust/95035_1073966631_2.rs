
this new std::sync::Mutex       avg 784.189µs    min 663.889µs    max 1.156756ms  
current std::sync::Mutex (gnu)  avg 2.802436ms   min 2.483913ms   max 3.781072ms  
current std::sync::Mutex (musl) avg 2.900143ms   min 2.795631ms   max 4.051815ms
parking_lot::Mutex              avg 695.039µs    min 573.318µs    max 921.173µs   
spin::Mutex                     avg 604.818µs    min 501.494µs    max 841.242µs   
AmdSpinlock                     avg 839.315µs    min 710.166µs    max 1.198424ms  
