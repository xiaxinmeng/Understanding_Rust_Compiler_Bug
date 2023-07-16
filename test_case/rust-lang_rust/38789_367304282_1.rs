rust
fn baz_host(...) { bar_host(...); baz(); }
#[target_feature(enabled = "sm40")]
extern "ptx" fn baz_nvptx(...) { bar_nvptx(...); baz();  }
extern "spriv" fn baz_spirv(...) { bar_spirv(...); baz();  }

fn foo_host(...) { bar_host(...); baz(); }
#[target_feature(enabled = "sm40")]
extern "ptx-kernel" fn foo_nvptx(...) { bar_nvptx(...); baz();  }
extern "spirv-kernell" fn foo_spirv(...) { bar_device(...); baz();  }
