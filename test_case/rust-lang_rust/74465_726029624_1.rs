rust
fn global_state() -> &'static GlobalState {
  static INSTANCE: SyncOnceCell<GlobalState> = SyncOnceCell::new();
  INSTANCE.get_or_init(GlobalState::default)
}
