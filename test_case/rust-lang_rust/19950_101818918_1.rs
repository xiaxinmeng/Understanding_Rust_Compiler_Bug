
iss_19950.rs:7:1: 7:22 error: the trait `Copy` may not be implemented for this type; variant `MyVariant` does not implement `Copy` [E0205]
iss_19950.rs:7 impl Copy for Test {}
               ^~~~~~~~~~~~~~~~~~~~~
iss_19950.rs:7:1: 7:22 error: the trait `core::clone::Clone` is not implemented for the type `Test` [E0277]
iss_19950.rs:7 impl Copy for Test {}
               ^~~~~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
