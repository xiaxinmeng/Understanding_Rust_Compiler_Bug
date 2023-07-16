scala
> lift {
>   val a = unlift(callServiceA())
>   val b = unlift(callServiceB(a))
>   val c = unlift(callServiceC(b))
>   (a, c)
> }
> 