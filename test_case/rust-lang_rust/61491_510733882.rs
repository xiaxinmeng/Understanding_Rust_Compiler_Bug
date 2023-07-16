diff
  impl<T: 'static> LocalKey<T> {
      pub fn try_with<F, R>(&'static self, f: F) -> Result<R, AccessError>
      where
          F: FnOnce(&T) -> R;
  }

  pub struct AccessError { _private: () }

+ impl Clone for AccessError {...}
+ impl Copy for AccessError {...}
+ impl Eq for AccessError {...}
+ impl PartialEq for AccessError {...}
+ impl std::error::Error for AccessError {...}
