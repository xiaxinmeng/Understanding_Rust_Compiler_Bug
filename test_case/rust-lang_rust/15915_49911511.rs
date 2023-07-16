
#[bench]
fn bench_serializer_mem_writer(b: &mut Bencher) {
    let log = Log::new();
    let json = json::to_vec(&log);
    b.bytes = json.len() as u64;

    b.iter(|| {
        //let _json = json::to_str(&log).unwrap();
        let mut wr = MemWriter0::with_capacity(1024);
        {
            let mut serializer = json::Serializer::new(wr.by_ref());
            log.serialize(&mut serializer).unwrap();
        }
        let _json = wr.unwrap();
    });
}
