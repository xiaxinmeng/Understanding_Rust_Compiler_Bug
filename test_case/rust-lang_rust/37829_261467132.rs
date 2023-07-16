
$ llc bugpoint-reduced-simplified.ll 
Assertion failed: (width < BitWidth && "Invalid APInt Truncate request"), function trunc, file /Users/vadzim/llvm/lib/Support/APInt.cpp, line 917.
Stack dump:
0.  Program arguments: llc bugpoint-reduced-simplified.ll 
1.  Running pass 'Function Pass Manager' on module 'bugpoint-reduced-simplified.ll'.
2.  Running pass 'MSP430 DAG->DAG Pattern Instruction Selection' on function '@_ZN4core3num6bignum5tests6Big8x37div_rem17h4a91efdf5fc031b4E'
Abort trap: 6
