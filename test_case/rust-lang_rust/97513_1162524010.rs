
if use_vendoring:
  if .cargo/config exists:
    print a warning and continue
  else:
    create .cargo/config
else:
  remove .cargo/config
