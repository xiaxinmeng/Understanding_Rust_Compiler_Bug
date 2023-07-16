js
for (i = from; i <= to; ++i) {
    let element = document.getElementById(i);
    if (!element) { break; }
    addClass(element, "line-highlighted");
}
