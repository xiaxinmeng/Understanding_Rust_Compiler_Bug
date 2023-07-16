\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/loop-break-value-no-repeat.rs","byte_start":771,"byte_end":779,"line_start":22,"line_end":22,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        break 22 //~ ERROR `break` with value from a `for` loop","highlight_start":9,"highlight_end":17}],"label":"can only break with a value inside `loop` or breakable block","suggested_replacement":null,"expansion":null}],"children":[{"message":"instead, use `break` on its own without a value inside this `for` loop","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/loop-break-value-no-repeat.rs","byte_start":771,"byte_end":779,"line_start":22,"line_end":22,"colu

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:045b4d03
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
