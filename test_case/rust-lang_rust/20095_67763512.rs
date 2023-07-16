 js
$('.search-input').on('keyup', function() {
    clearTimeout(keyUpTimeout);
    keyUpTimeout = setTimeout(search, 100);
});
