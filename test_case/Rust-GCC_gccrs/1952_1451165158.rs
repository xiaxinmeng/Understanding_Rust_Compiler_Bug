c++

const char *fmt_str
	    = "%s. see issue %ld "
	      "<https://github.com/rust-lang/rust/issues/%ld> for more "
	      "information. add `#![feature(%s)]` to the crate attributes to "
	      "enable.";
 rust_error_at (loc, fmt_str, error_msg.c_str (), issue, issue,
			 feature.as_string ().c_str ());
