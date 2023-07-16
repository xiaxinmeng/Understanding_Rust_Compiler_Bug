
// FIXME Use a collection type that doesn't copy node and edge data and
// grow multiplicatively on reallocation. Without such a collection or
// solution having the same effect, there is a performance hazard here
// in both time and space, as growing these collections means copying a
// large amount of data and doubling already large buffer capacities. A
// solution for this will also mean that it's less important to get
// these estimates right.
