js
document.addEventListener("DOMContentLoaded", function() {
  let fragment = getPageId();  // getPageId is defined in main.js
  if (fragment.startsWith("tymethod.")) {
    document.location.hash = "#" + fragment.slice(2);
  }
}
