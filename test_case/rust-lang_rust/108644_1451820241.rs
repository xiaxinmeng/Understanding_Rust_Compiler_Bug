python
"{" + ", ".join(map(lambda a: "{} = {}".format(to_toml(a[0]), to_toml(a[1])), value.items())) + "}"
