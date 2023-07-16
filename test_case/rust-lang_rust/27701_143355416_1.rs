
#[cfg(my_feature)]
extern crate core as std;
#[cfg(not(my_feature))]
extern crate std;

// The rest of the source code uses name std
// ...
