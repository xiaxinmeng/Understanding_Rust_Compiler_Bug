js
function foo() {
    console.log("foo!");
    foo = function() {};
}
foo(); // will display "foo!"
foo(); // will display nothing
