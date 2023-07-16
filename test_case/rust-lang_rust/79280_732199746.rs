
% cat ~/bin/force-arm64-cc
#!/usr/bin/env bash

exec arch -arm64e cc "$@"
