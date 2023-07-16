

#!/bin/bash

# now check to see if the profile.dev stuff is in root cargo.toml and if not, add it
grep profile.dev /home/nixomose/git/project/Cargo.toml
if [ "$?" != 0 ]; then
  echo "profile.dev not found in cargo.toml, adding it."
  echo "# avoid any optimization, so that the linker might not destroy the binary and make it undebuggable. anger. hate." >> /home/nixomose/git/project/Cargo.toml
  echo '[profile.dev]' >> /home/nixomose/git/project/Cargo.toml
  echo "incremental = false" >> /home/nixomose/git/project/Cargo.toml
  echo "codegen-units = 1" >> /home/nixomose/git/project/Cargo.toml
  echo 'lto = "off"' >> /home/nixomose/git/project/Cargo.toml
fi

