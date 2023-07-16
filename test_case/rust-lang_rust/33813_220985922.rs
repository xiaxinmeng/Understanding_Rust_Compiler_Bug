
<nox> Why is .offset unsafe?
<eddyb> nox: GEPi semantics
<benh> it goes faster if its unsafe
<nox> benh: What?
<nox> eddyb: I don't understand.
<eddyb> I' trying to look for the LLVM docs
<eddyb> http://llvm.org/docs/GetElementPtr.html#what-happens-if-a-gep-computation-overflows
<eddyb> it looks like you can trigger UB in at least one way
<eddyb> (address computation overflow)
<nox> eddyb: Thanks.
<ubsan> if it's not one past the end or inside the object, basically
