
fn sleep(s: uint) {
    let iotask = std::uv::global_loop::get();
    std::timer::sleep(iotask, s * 1000);
}
