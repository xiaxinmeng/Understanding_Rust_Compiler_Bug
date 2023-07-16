rust
> static Y = // ...
> 
> let x = &Y;
> yield;
> use(x);
> // and
> let x = &Y;
> yield;
> use(&Y);
> 