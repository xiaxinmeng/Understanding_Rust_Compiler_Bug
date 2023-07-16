
Rust Vector and Array performance comparison
Temp: 94697384247824, 94697384247824
buffer_len_immut: 128, buffer_len_mut: 128
buffer_len_borrowed_immut: 128, buffer_len_borrowed_mut: 128

Vector of borrowed mutable binding size
        iir_intoiterator_shared                 20.190 ns       1032x realtime
        iir_intoiterator_uniq_1                 19.311 ns       1079x realtime
        iir_intoiterator_enforced_len           19.614 ns       1062x realtime
        iir_c_style_for_loop                    22.398 ns       930x realtime
        iir_unchecked_c_style_for_loop          36.094 ns       577x realtime

Vector of borrowed immutable binding size
        iir_intoiterator_shared                 19.385 ns       1075x realtime
        iir_intoiterator_uniq_2                 19.288 ns       1080x realtime
        iir_intoiterator_enforced_len           19.376 ns       1075x realtime
        iir_c_style_for_loop                    22.410 ns       930x realtime
        iir_unchecked_c_style_for_loop          36.082 ns       577x realtime

Vector of mutable binding size
        iir_intoiterator_shared                 19.271 ns       1081x realtime
        iir_intoiterator_uniq_3                 19.289 ns       1080x realtime
        iir_intoiterator_enforced_len           19.371 ns       1076x realtime
        iir_c_style_for_loop                    22.656 ns       920x realtime
        iir_unchecked_c_style_for_loop          36.069 ns       578x realtime

Vector of immutable binding size
        iir_intoiterator_shared                 19.436 ns       1072x realtime
        iir_intoiterator_uniq_4                 19.429 ns       1072x realtime
        iir_intoiterator_enforced_len           19.360 ns       1076x realtime
        iir_c_style_for_loop                    22.386 ns       931x realtime
        iir_unchecked_c_style_for_loop          36.078 ns       577x realtime

Vector of const size
        iir_intoiterator_shared                 19.288 ns       1080x realtime
        iir_intoiterator_uniq_5                 19.323 ns       1078x realtime
        iir_intoiterator_enforced_len           19.463 ns       1070x realtime
        iir_c_style_for_loop                    22.392 ns       930x realtime
        iir_unchecked_c_style_for_loop          36.049 ns       578x realtime

Slice of mutable size from Array (4096)
        iir_intoiterator_shared                 19.336 ns       1077x realtime
        iir_intoiterator_uniq_6                 19.983 ns       1043x realtime
        iir_intoiterator_enforced_len           19.431 ns       1072x realtime
        iir_c_style_for_loop                    22.363 ns       932x realtime
        iir_unchecked_c_style_for_loop          36.094 ns       577x realtime

Slice of immutable size from Array (4096)
        iir_intoiterator_shared                 19.424 ns       1073x realtime
        iir_intoiterator_uniq_7                 19.298 ns       1080x realtime
        iir_intoiterator_enforced_len           19.324 ns       1078x realtime
        iir_c_style_for_loop                    22.436 ns       929x realtime
        iir_unchecked_c_style_for_loop          36.095 ns       577x realtime

Slice of const size from Array (4096)
        iir_intoiterator_shared                 19.441 ns       1072x realtime
        iir_intoiterator_uniq_8                 19.398 ns       1074x realtime
        iir_intoiterator_enforced_len           19.359 ns       1076x realtime
        iir_c_style_for_loop                    22.422 ns       929x realtime
        iir_unchecked_c_style_for_loop          36.174 ns       576x realtime

Array (128)
        iir_intoiterator_shared                 19.218 ns       1084x realtime
        iir_intoiterator_uniq_9                 19.126 ns       1089x realtime
        iir_intoiterator_enforced_len           19.258 ns       1082x realtime
        iir_c_style_for_loop                    22.205 ns       938x realtime
        iir_unchecked_c_style_for_loop_shared   35.893 ns       580x realtime
        iir_array_c_style_for_loop              21.976 ns       948x realtime
