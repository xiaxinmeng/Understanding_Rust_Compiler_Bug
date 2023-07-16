
#[bench]
fn bench_cowstring(b: &mut test::Bencher) {
    b.iter(|| {
        let fizz = Into::<Cow<_>>::into("Fizz").into_owned();
        let buzz = Into::<Cow<_>>::into("Buzz");
        Into::<Cow<_>>::into(fizz + &*buzz)
    });
}

#[bench]
fn bench_cowstring_ms2ger(b: &mut test::Bencher) {
    b.iter( || {  
            let mut fizz = Into::<Cow<_>>::into("Fizz").into_owned();
            let buzz = Into::<Cow<_>>::into("Buzz");
            fizz.push_str(&*buzz);
            Into::<Cow<_>>::into(fizz);
        }
    );
}
