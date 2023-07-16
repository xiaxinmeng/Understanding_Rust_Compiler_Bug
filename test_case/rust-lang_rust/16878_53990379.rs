
for a in range(0i, 5) {
    let mut file = BufferedWriter::new(File::create(&Path::new("matches_rust.txt")));
    for auction in auctions.iter() {
        let mut i = -1;
        for ts in terms.iter() {
            i += 1;
            for t in ts.iter() {
                if auction.as_slice().contains(t.as_slice()) { // uses find_str
                    file.write_uint(i);
                    file.write(b" ");
                    break;
                }
            }
        }
        file.write(b"\n");
    }
}
