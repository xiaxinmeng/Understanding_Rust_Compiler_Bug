rust

impl<T, E> Termination for Result<T, E> where
E: Termination {
// Do match pattern here and grab report but also display the error as is standard.
}

impl<T, E> Termination for Result<T, E> where
E: !Termination {
// This would be the old impl making it valid for all E types who do not have Termination implemented.
}
