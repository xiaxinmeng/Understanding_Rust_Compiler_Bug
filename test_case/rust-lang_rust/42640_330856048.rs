rust
> for &mut (tok, ref mut bot) in &mut self.bots {
>   let actions = bot.tick(&self.game_state);
>    debug!("Bot {} executed {:?}", tok, actions);
> }
> 