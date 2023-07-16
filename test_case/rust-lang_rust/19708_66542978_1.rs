
extern crate test;

const TEXT: &'static str = "Hello World!";
const LONG_TEXT: &'static str = "
There are not many persons who know what wonders are opened to them in the
stories and visions of their youth; for when as children we listen and dream,
we think but half-formed thoughts, and when as men we try to remember, we are
dulled and prosaic with the poison of life. But some of us awake in the night
with strange phantasms of enchanted hills and gardens, of fountains that sing
in the sun, of golden cliffs overhanging murmuring seas, of plains that stretch
down to sleeping cities of bronze and stone, and of shadowy companies of heroes
that ride caparisoned white horses along the edges of thick forests; and then
we know that we have looked back through the ivory gates into that world of
wonder which was ours before we were wise and unhappy.";

fn bench_direct(b: &mut test::Bencher, string: &str) {
    b.iter(|| {
        test::black_box(string.into_string());
    });
    b.bytes = string.as_bytes().len().to_u64().unwrap();
}

fn bench_via_format(b: &mut test::Bencher, string: &str) {
    b.iter(|| {
        test::black_box(string.to_string());
    });
    b.bytes = string.as_bytes().len().to_u64().unwrap();
}

fn very_long_string() -> String {
    let mut s = String::with_capacity(LONG_TEXT.len() * 1000);
    for _ in range(0u32, 1000) {
        s.push_str(LONG_TEXT);
    }
    s
}

#[bench] fn bench_short_direct(b: &mut test::Bencher) { bench_direct(b, TEXT); }
#[bench] fn bench_short_via_format(b: &mut test::Bencher) { bench_via_format(b, TEXT); }

#[bench] fn bench_medium_direct(b: &mut test::Bencher) { bench_direct(b, LONG_TEXT); }
#[bench] fn bench_medium_via_format(b: &mut test::Bencher) { bench_via_format(b, LONG_TEXT); }

#[bench] fn bench_long_direct(b: &mut test::Bencher) { bench_direct(b, very_long_string().as_slice()); }
#[bench] fn bench_long_via_format(b: &mut test::Bencher) { bench_via_format(b, very_long_string().as_slice()); }
