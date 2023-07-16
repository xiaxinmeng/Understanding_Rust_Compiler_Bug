
this new std::sync::Mutex       avg 1.189231ms   min 807.39µs     max 1.50386ms
current std::sync::Mutex (gnu)  avg 2.97886ms    min 2.079744ms   max 3.96414ms   
current std::sync::Mutex (musl) avg 2.794938ms   min 2.732422ms   max 3.21436ms   
parking_lot::Mutex              avg 940.333µs    min 643.591µs    max 1.14756ms
spin::Mutex                     avg 765.141µs    min 565.754µs    max 983.661µs   
AmdSpinlock                     avg 856.35µs     min 615.248µs    max 1.07866ms   
