javascript
const list = document.querySelectorAll('input[type=checkbox]');
Array.prototype.forEach.call(list, function (checkbox) {
  checkbox.checked = true;
});
