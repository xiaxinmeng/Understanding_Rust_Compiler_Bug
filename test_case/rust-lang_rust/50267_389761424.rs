rust
// Option<T>
fn deref(&self) -> Option<&T::Target>;

// Result<T, E>
fn deref_ok(&self) -> Result<&T::Target, &E>;
fn deref_err(&self) -> Result<&T, &E::Target>;
fn deref(&self) -> Result<&T::Target, &E::Target>;
