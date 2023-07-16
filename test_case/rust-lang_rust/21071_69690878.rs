
document.addEventListener("DOMContentLoaded", function(event) {
 document.getElementById("toggle-nav").onclick = toggleNav;

 function toggleNav() {
     var toc = document.getElementById("toc");
     var pagewrapper = document.getElementById("page-wrapper");

     toggleClass(toc, "mobile-hidden");
     toggleClass(pagewrapper, "mobile-hidden";)
 };

 function toggleClass(el, className) {
    // Stolen from http://youmightnotneedjquery.com/
    if (el.classList) {
      el.classList.toggle(className);
    } else {
      var classes = el.className.split(' ');
      var existingIndex = classes.indexOf(className);

      if (existingIndex >= 0) {
        classes.splice(existingIndex, 1);
      } else {
        classes.push(className);
      }

      el.className = classes.join(' ');
    }
 }
});
