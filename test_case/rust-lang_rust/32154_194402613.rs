 sh
# Install the manifest for future updates if [ "$_manifest_to_stash" != "" ]; then
    ensure printf "%s" "$_manifest_to_stash" > "$_prefix/lib/rustlib/channel-manifest.toml"
fi
