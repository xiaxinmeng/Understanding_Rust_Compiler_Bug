
// before
_3 = _1.0;
_4 = _3;
yield;
_6 = _3;

// after
_3 = _1.0;  // this local will not be stored in the generator at all
_4 = _3;
yield;
_5 = _1.0; // create a new local that will reload the value of the upvar directly from the generator fields
_6 = _5;
