javascript
let wasmInstance = "";
let addRust = "";

fetchAndInstantiate('small-add.wasm', {}).then(instance => {
    wasmInstance = instance;
    console.log("wasmInstance instantiated in line 11");

    sayHello = wasmInstance.exports.say_hello;
    console.log("function sayHello instantiated in line 17");
    console.log(sayHello());
});
