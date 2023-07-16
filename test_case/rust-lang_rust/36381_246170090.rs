
>        // We don't want to normalize associated types that occur inside of region
>        // binders, because they may contain bound regions, and we can't cope with that.
>        //
>        // Example:
>        //
>        //     for<'a> fn(<T as Foo<&'a>>::A)
>        //
>        // Instead of normalizing `<T as Foo<&'a>>::A` here, we'll
>        // normalize it when we instantiate those bound regions (which
>        // should occur eventually).
> 