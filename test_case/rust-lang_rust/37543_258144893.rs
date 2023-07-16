
extern crate qt_widgets;

use qt_widgets::application::Application;
use qt_widgets::push_button::PushButton;
use qt_widgets::cpp_utils::*;
use qt_widgets::qt_core::string::String;
use qt_widgets::libc::{c_char, c_int};

fn to_qt_string<S: AsRef<str>>(s: S) -> String {
  let slice = s.as_ref().as_bytes();
  String::from_utf8((slice.as_ptr() as *const c_char, slice.len() as c_int, AsStruct))
}

fn from_qt_string(string: &String) -> std::string::String {
  let buf = string.to_utf8(AsStruct);
  unsafe {
    let bytes = std::slice::from_raw_parts(buf.const_data() as *const u8, buf.count(()) as usize);
    std::str::from_utf8_unchecked(bytes).to_string()
  }
}

fn main() {
  let _app = Application::new((&mut 0i32, &mut (&mut 0i8 as *mut i8) as *mut *mut i8, AsBox));
  let mut btn = PushButton::new((&to_qt_string("first_button"), AsBox));
  let mut btn2 = PushButton::new((&to_qt_string("second_button"), AsBox));
  let text = from_qt_string(&btn.text(AsStruct));
  let text2 = from_qt_string(&btn2.text(AsStruct));
  btn.show();
  btn2.show();
  let ret = Application::exec();
}
