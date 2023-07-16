 rust
fn showDialog(message: ~str,
              parent: Option<Widget>,
              title: Option<~str>,
              type: Option<DialogType>,
              icon: Option<Icon>) { ... }

// Display an info box in the middle of the screen.
// Set the title to "information", translated in the current locale
// Set the icon to an "info" icon
showDialog(~"hello, world!");

// Display a warning box in the middle of the screen.
// Set the title to "warning", translated in the current locale
// Set the icon to a "warning" icon
showDialog(~"sick, sad world!", None, None, WarningDialog);

// Display a warning box in the middle of the screen.
// Set the title to "warning", translated in the current locale
// Set the icon to a custom icon
showDialog(~"sick, sad world!", None, None, WarningDialog, bugIcon);
