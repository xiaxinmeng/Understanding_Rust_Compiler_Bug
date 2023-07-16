
i.map(|x| (x, f(x))).max_with(comparing(|(_, h)| h)).map(|(v, _)| v)
