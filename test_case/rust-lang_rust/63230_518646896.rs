
2019-08-06T10:01:20.2097589Z error[E0381]: assign to part of possibly uninitialized variable: `partially_initialized`
2019-08-06T10:01:20.2098466Z   --> /checkout/src/doc/reference/src/destructors.md:69:5
2019-08-06T10:01:20.2099254Z    |
2019-08-06T10:01:20.2100069Z 40 |     partially_initialized.0 = ShowOnDrop("Partial tuple first");
2019-08-06T10:01:20.2100423Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ use of possibly uninitialized `partially_initialized`
2019-08-06T10:01:20.2100603Z 
2019-08-06T10:01:20.2100778Z error: aborting due to previous error
