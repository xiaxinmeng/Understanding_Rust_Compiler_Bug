
// before
_3 = _1.0; // here upvar is stored into a local, the local will be needlessly included in the generator
_4 = _3; // here the local is used

// after
_4 = _1.0; // we got rid of the local altogether
