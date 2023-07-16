
datatype foo = FOO of foo;

val x: foo = Unsafe.cast 0;
case x of _ => print "hello darkness my old friend";

Error: Compiler bug: PPObj: switch: none of the datacons matched
