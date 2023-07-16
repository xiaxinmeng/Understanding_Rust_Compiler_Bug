
enum maybe_pointy {
    no_pointy,
    yes_pointy(@pointy)
}

type pointy = {
    mutable x : maybe_pointy
};

fn main()
{
    const max : uint = 10u;

    let v : [mutable @pointy] = [mutable];
    uint::range(0u, max) {|_i|
        v += [mutable @{ mutable x : no_pointy }];
    }

    v[0].x = yes_pointy(v[1]);
    v[1].x = yes_pointy(v[0]);

    uint::range(0u, max) {|i|
        v[i] = @{ mutable x : no_pointy };
    }
}
