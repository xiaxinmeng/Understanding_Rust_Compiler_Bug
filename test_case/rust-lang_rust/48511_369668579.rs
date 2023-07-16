js
function switchTheme(styleElem, mainStyleElem, newTheme) {
    styleElem.href = mainStyleElem.href.replace("rustdoc-EXAMPLE.css", newTheme + ".css");
    updateLocalStorage('theme', newTheme);
}
