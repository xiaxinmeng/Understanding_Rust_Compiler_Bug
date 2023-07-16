javascript
query = (function() {
    switch(raw) {
        case '!': return "Never";
        case '?': return "try!";
        case '+': return "Add";
        case '-': return "Sub";
        case '%': return "Rem";
        case '[':
        case ']': return "Index";
        // and so forth
        default: return raw;
    }
})();
