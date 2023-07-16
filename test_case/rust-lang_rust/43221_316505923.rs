rust
    // When targeting MSVC, emit C++ style type names for compatability with
    // .natvis visualizers (and perhaps other existing native debuggers?)
    let cpp_like_names = cx.sess().target.target.options.is_like_msvc;
