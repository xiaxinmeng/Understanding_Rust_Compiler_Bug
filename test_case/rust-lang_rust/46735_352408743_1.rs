rust

#[bench]
fn find_char(b: &mut Bencher) {
    let x = test::black_box("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.");
    b.iter(|| test::black_box(x.find('/')));
}

#[bench]
fn find_char_memchr(b: &mut Bencher) {
    let x = test::black_box("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.");
    b.iter(|| test::black_box(memchr::memchr(b'/', x.as_bytes())));
}

#[bench]
fn find_multibyte_char_found(b: &mut Bencher) {
    let x = test::black_box("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, ก remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.");
    b.iter(|| test::black_box(x.find('ก')));
}

#[bench]
fn find_multibyte_char_notfound(b: &mut Bencher) {
    let x = test::black_box("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.");
    b.iter(|| test::black_box(x.find('ก')));
}

#[bench]
fn find_multibyte_string_multibyte_char(b: &mut Bencher) {
    let x = test::black_box("जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली");
    b.iter(|| test::black_box(x.find('ग'))); // not in the string
}

#[bench]
fn find_multibyte_string_pathological(b: &mut Bencher) {
    let x = test::black_box("जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली, जलद कोल्हा आळशी कुत्रा वरुन उडी मारली");
    b.iter(|| test::black_box(x.find('ä'))); // ä's last byte is found often in Devanagari text
}
