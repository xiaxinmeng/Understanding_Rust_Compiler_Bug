
macro_rules! translate_mob(
  ($world:expr, $mob:expr, $v:expr) => (
    App::translate_mob(
      &$world.gl,
      &mut $world.physics,
      $world.mob_buffers.get_mut_ref(),
      ...
      $mob,
      $v
    );
  );
)
