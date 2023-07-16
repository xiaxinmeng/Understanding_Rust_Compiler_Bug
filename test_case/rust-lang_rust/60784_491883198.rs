`
trait Error {
   ...
   fn as_dyn_any(&self) -> &dyn Any where Self: 'static { self as _ }
   fn as_mut_dyn_any(&mut self) -> &mut dyn Any where Self: 'static { self as _ }
}
