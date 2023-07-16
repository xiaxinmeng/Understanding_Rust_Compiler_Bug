rust
pub struct SizeMgr<'a>(&'a dyn ThemeSize);

pub struct ConfigMgr<'a> {
    sh: &'a dyn ThemeSize,
    ds: &'a mut dyn DrawShared,
    pub(crate) ev: &'a mut EventState,
}

pub struct EventMgr<'a> {
    state: &'a mut EventState,
    shell: &'a mut dyn ShellWindow,
    messages: Vec<Message>,
    scroll: Scroll,
    action: TkAction,
}

pub struct DrawMgr<'a> {
    h: &'a mut dyn ThemeDraw,
    id: WidgetId,
}
