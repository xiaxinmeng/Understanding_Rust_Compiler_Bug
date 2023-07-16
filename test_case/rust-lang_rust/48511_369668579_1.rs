js
["dark-EXAMPLE","main-EXAMPLE"].forEach(function(item) {
    var but = document.createElement('button');
    but.innerHTML = item;
    but.onclick = function(el) {
        switchTheme(currentTheme, mainTheme, item);
    };
    themes.appendChild(but);
});
