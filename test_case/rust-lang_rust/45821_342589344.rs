llvm-ir
define i8 @no_op(i8) {

    %temp = alloca i8
    store i8 %0, i8* %temp
    %value = load i8, i8* %temp, !range !{i8 0, i8 2}

    switch i8 %value, label %one [
        i8 0, label %zero
    ]

zero:
    br label %end

one:
    br label %end

end:
    %ret = phi i8 [0, %zero], [1, %one]
    ret i8 %ret
}
