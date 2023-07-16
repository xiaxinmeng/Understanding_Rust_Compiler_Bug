
for (id, mob) in self.mobs.mut_iter() {
    translate_mob(mob, &self.physics, &self.gl_context, &self.mob_gl_buffers, ..., Vec3::new(1.0, 2.0, 3.0));
}
