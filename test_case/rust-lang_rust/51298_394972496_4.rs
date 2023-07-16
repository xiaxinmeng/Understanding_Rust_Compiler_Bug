\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs","byte_start":531,"byte_end":617,"line_start":16,"line_end":18,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn can_parse_zero_as_f32() -> Result<f32, ParseIntError> { //~ ERROR","highlight_start":1,"highlight_end":69},{"text":"    \"0\".parse()","highlight_start":1,"highlight_end":16},{"text":"}","highlight_start":1,"highlight_end":2}],"label":"`main` can only return types that implement `std::process::Termination`","suggested_replacement":

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00366298
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
