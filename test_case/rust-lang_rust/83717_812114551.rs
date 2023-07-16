javascript
function removeEmptyStringsFromArray_A(x) {
    for (var i = 0, len = x.length; i < len; ++i) {
        if (x[i] === "") {
            x.splice(i, 1);
            i -= 1;
        }
    }
}
function removeEmptyStringsFromArray_B(x) {
    for (var i = x.length - 1, j = x.length - 1; j >= 0; j -= 1) {
        if (x[j] !== "") {
            x[i] = x[j];
            i -= 1;
        }
    }
    x.splice(0, i + 1);
}
