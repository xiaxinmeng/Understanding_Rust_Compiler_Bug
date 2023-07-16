 rust
fn main() {
    let v: ~[&str] = "Mary had a little lamb".split(' ').collect();
    assert_eq!(v, ~["Mary", "had", "a", "little", "lamb"]);

    let v: ~[&str] = "abc1def2ghi".split(|c: char| c.is_digit()).collect();
    assert_eq!(v, ~["abc", "def", "ghi"]);

    let v: ~[&str] = "lionXXtigerXleopard".split('X').collect();
    assert_eq!(v, ~["lion", "", "tiger", "leopard"]);


    let v: ~[&str] = "Mary had a little lambda".splitn(' ', 2).collect();
    assert_eq!(v, ~["Mary", "had", "a little lambda"]);

    let v: ~[&str] = "abc1def2ghi".splitn(|c: char| c.is_digit(), 1).collect();
    assert_eq!(v, ~["abc", "def2ghi"]);

    let v: ~[&str] = "lionXXtigerXleopard".splitn('X', 2).collect();
    assert_eq!(v, ~["lion", "", "tigerXleopard"]);


    let v: ~[&str] = "A.B.".split_terminator('.').collect();
    assert_eq!(v, ~["A", "B"]);

    let v: ~[&str] = "A..B..".split_terminator('.').collect();
    assert_eq!(v, ~["A", "", "B", ""]);

    let v: ~[&str] = "Mary had a little lamb".rsplit(' ').collect();
    assert_eq!(v, ~["lamb", "little", "a", "had", "Mary"]);

    let v: ~[&str] = "abc1def2ghi".rsplit(|c: char| c.is_digit()).collect();
    assert_eq!(v, ~["ghi", "def", "abc"]);
    let v: ~[&str] = "lionXXtigerXleopard".rsplit('X').collect();
    assert_eq!(v, ~["leopard", "tiger", "", "lion"]);

    let v: ~[&str] = "abcXXXabcYYYabc".split_str("abc").collect();
    assert_eq!(v, ~["", "XXX", "YYY", ""]);

    let v: ~[&str] = "1abcabc2".split_str("abc").collect();
    assert_eq!(v, ~["1", "", "2"]);

    let v: ~[(uint, uint)] = "abcXXXabcYYYabc".match_indices("abc").collect();
    assert_eq!(v, ~[(0, 3), (6, 9), (12, 15)]);
    let v: ~[(uint, uint)] = "1abcabc2".match_indices("abc").collect();
    assert_eq!(v, ~[(1,4), (4,7)]);


    // composed forms of `ö` and `é`
    let c = "Löwe 虎 léopard"; // German, Simplified Chinese, French
    assert_eq!(c.char_len(), 14);
    assert_eq!(c.len(), 18);

    // decomposed forms of `ö` and `é`
    let d = "Lo\u0308we 虎 le\u0301opard";
    println(d);
    assert_eq!(d.char_len(), 16);
    assert_eq!(d.len(), 20);

    let four_lines = "foo\nbar\n\nbaz\n";
    let v: ~[&str] = four_lines.lines().collect();
    assert_eq!(v, ~["foo", "bar", "", "baz"]);

    let four_lines = "foo\r\nbar\n\r\nbaz\n";
    let v: ~[&str] = four_lines.lines_any().collect();
    assert_eq!(v, ~["foo", "bar", "", "baz"]);
    let some_words = " Mary   had\ta little  \n\t lamb";
    let v: ~[&str] = some_words.words().collect();
    assert_eq!(v, ~["Mary", "had", "a", "little", "lamb"]);

    assert!("Löwe虎léopard123".is_alphanumeric());
    assert!( !" &*~".is_alphanumeric());
    let s = "Löwe 虎 léopard";
    assert_eq!(s.slice(0, 1), "L");

    assert_eq!(s.slice(1, 9), "öwe 虎")

        let s = ~"Do you know the muffin man,
The muffin man, the muffin man, ...";

    assert_eq!(s.replace("muffin man", "little lamb"),
               ~"Do you know the little lamb,
The little lamb, the little lamb, ...");

    // not found, so no change.
    assert_eq!(s.replace("cookie monster", "little lamb"), s);

    let s = "Löwe 虎 Léopard";

    assert_eq!(s.find('L'), Some(0));
    assert_eq!(s.find('é'), Some(11));

    // the first space
    assert_eq!(s.find(|c: char| c.is_whitespace()), Some(5));

    // neither are found
    assert_eq!(s.find(&['1', '2']), None);

    let s = "Löwe 虎 Léopard";

    assert_eq!(s.rfind('L'), Some(10));
    assert_eq!(s.rfind('é'), Some(11));

    // the second space
    assert_eq!(s.rfind(|c: char| c.is_whitespace()), Some(9));

    // neither are found
    assert_eq!(s.rfind(&['1', '2']), None);

    let s = "Löwe 虎 Léopard";

    assert_eq!(s.find_str("虎 L"), Some(6));
    assert_eq!(s.find_str("muffin man"), None);

}
