
let constant_buffer_desc = D3D11_BUFFER_DESC {
    ByteWidth: mem::size_of::<cgmath::Matrix4<f32>> as _,
    Usage: D3D11_USAGE_DYNAMIC,
    BindFlags: D3D11_BIND_CONSTANT_BUFFER,
    CPUAccessFlags: D3D11_CPU_ACCESS_WRITE,
    ..Default::default()
};
