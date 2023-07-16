rust
let now = Instant::now();
let elapsed = if self.current_timer >= now {
   Duration::new(0,0)
} else {
   self.current_timer.elapsed()
};

self.current_timer = now;
