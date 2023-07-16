rust
macro_rules! foo {
    ($x:ident, $e:expr) => ([(); {
        let $x = 0;
        let x = 1;
        $e
    }])
}

macro_rules! bar {
    ($x:ident, $e:expr) => ([(); {
        let $x = 0;
        let $x = 1;
        $e
    }])
}

const _: foo!(x, x + 1) = [()];
const _: bar!(x, x + 1) = [(), ()];
