
x.a = <expr-a>;
...
x.z = ...z;
x = Foo { a: x.a, ..., z: x.z} // or remove as it is a noop
