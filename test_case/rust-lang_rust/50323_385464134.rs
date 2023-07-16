rust
fn main() {
    use ::std::collections::HashSet;
    let set: HashSet<_> = {
        (0..86).into_iter()
            .chain(87..89)
            .chain(92..93)
            .chain(94..105)
//            .chain(107..108)
            .chain(116..118)
            .chain(122..124)
            .chain(128..131)
            .chain(133..134)
//            .chain(141..143)
            .chain(151..152)
            .chain(153..154)
//            .chain(158..168)
//            .chain(169..170)
//            .chain(179..180)
//            .chain(181..182)
//            .chain(185..186)
            .chain(188..189)
            .chain(201..202)
//            .chain(203..204)
            .collect()
    };
    assert!(set.contains(&153));
}
