js
for (var i = 0, len = x.length; i < len; ++i) {
    if (x[i] === "") {
        x.splice(i, 1);
        i -= 1;
    }
}
