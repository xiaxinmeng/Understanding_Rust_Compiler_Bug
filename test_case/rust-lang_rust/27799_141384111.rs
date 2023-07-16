 rust
   #[derive(Copy, Clone)]
   pub struct Timer {
       start_t: SteadyTime,
   }
   
   impl Timer {
       pub fn start() -> Timer {
           Timer {
               start_t: SteadyTime::now(),
           }
       }
   
       pub fn stop(&self) -> Duration {
           &SteadyTime::now() - &self.start_t
       }
   
       pub fn span<F>(f: F) -> Duration where F: FnOnce() {
           let timer = Timer::start();
           f();
           timer.stop()
       }
   }
   