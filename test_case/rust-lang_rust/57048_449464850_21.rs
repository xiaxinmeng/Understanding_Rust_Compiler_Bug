\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-single-nominal-type.rs","byte_start":1125,"byte_end":1162,"line_start":29,"line_end":29,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    fn x86_mm_max_epi16(x: B, y: B) -> B;","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0441]: unrecognized platform-specific intrinsic function: `x86_mm_max_epi16`\n  --> /checkout/src/test/ui/simd-intrinsic/simd-intrinsic-single-nominal-type.rs:29:5\n   |\nLL |     fn x86_mm_max_epi16(x: B, y: B) -> B;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^travis_time:end:24da720d:start=1545413696276156963,finish=1545417238208879371,duration=3541932722408
travis_time:start:04953d83
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 21 18:33:58 UTC 2018
Fri, 21 Dec 2018 18:33:58 GMT
