 xml
# ------------------------------------------------------------------------------
# | Web fonts access |
# ------------------------------------------------------------------------------

# Allow access to web fonts from all domains.

<IfModule mod_headers.c>
    <FilesMatch "\.(eot|otf|tt[cf]|woff)$">
        Header set Access-Control-Allow-Origin "*"
    </FilesMatch>
</IfModule>
