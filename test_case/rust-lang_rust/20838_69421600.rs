
[bucket]
# Must be specified:
name   = "static-rust-lang-org"
region = "us-west-1"
access_key = "" # Can alternatively be defined through AWS_ACCESS_KEY_ID environment
                # variable
secret_key = "" # Can alternatively be defined through AWS_SECRET_ACCESS_KEY
                # environment variable
prefix = "dist/"       # Only files with this prefix will be listed;
path_separator = "/"   # Specify, if anything other than / is used to denote path hierarchy.
                       # \ is common on windows for example;
base_url = "https://static.rust-lang.org" # Base URL for links. Example values
              # * `https://bucketname.s3.amazonaws.com`
              # * `https://static.example.com`.
              # Defaults to empty string and links will be relative rather
              # than absolute.
              # *DO NOT* add a trailing slash.

[output]
extra_head = ""                   # Extra HTML to be included into <head>.
list_zero_sized = false           # List files with 0 Size. Generally useless, since it also lists
                                  # directories… i think?
