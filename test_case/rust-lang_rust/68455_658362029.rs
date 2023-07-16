C++
internal::atomic_ptr<const implementation> active_implementation{&detect_best_supported_implementation_on_first_use_singleton};


const implementation *detect_best_supported_implementation_on_first_use::set_best() const noexcept {
  return active_implementation = available_implementations.detect_best_supported();
}
