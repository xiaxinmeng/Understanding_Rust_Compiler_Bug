
const gpio: *const RegisterBlock = unsafe { lpc176x5x::GPIO::ptr() };

unsafe fn x() {
  (*gpio).pin2.write( |w| w.bits(0xDEADBEEF))
}
