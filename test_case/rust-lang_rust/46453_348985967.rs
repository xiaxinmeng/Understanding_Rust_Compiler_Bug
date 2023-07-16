
  if (cu->language == language_rust && mangled != NULL
      && strchr (mangled, '{') != NULL)
    mangled = NULL;
