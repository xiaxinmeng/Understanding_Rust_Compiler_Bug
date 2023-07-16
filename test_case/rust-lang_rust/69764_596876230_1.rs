
fn main() {
    let a = 100 - 10 / 2;
    let host = "[3e80:a178:b18f:c5dd:250:56ff:fe9d:a351]";
    let host_v6 = host.replace('[', "").replace(']', "");
    println!("{}, {}, {}", a, host, host_v6);
}
