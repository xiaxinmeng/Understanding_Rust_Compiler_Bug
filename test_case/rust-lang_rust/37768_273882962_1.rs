rust
   pub fn draw(&self, jobs: &mut Vec<RenderJob>) {
        // just imagine we clear self.starfield & fill it with rects to be drawn
        jobs.push(RenderJob::DrawMany(self.tx_star_fg, self.starfield.clone()));
    }
