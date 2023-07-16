
$ shellcheck myscriptshellcheck.net
 
Line 34:
    if [ $? -ne 0 ]
         ^-- SC2181: Check exit code directly with e.g. 'if mycmd;', not indirectly with $?.
 
Line 41:
    if command -v $1 >/dev/null 2>&1
                  ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 48:
    local t
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 49:
    local tlen
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 50:
    eval t=\$$1
             ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 51:
    eval tlen=\${#$1}
                ^-- SC1083: This { is literal. Check expression (missing ;/\n?) or quote it.
                  ^-- SC2086: Double quote to prevent globbing and word splitting.
                    ^-- SC1083: This } is literal. Check expression (missing ;/\n?) or quote it.
 
Line 52:
    if [ $tlen -gt 35 ]
         ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 54:
        printf "gen-installer: %-20s := %.35s ...\n" $1 "$t"
                                                     ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 56:
        printf "gen-installer: %-20s := %s %s\n" $1 "$t"
               ^-- SC2183: This format string has 3 variables, but is passed 2 arguments.
                                                 ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 63:
    local op=$1
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 64:
    local default=$2
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 67:
    local doc="$*"
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 68:
    if [ $HELP -eq 0 ]
         ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 70:
        local uop=$(echo $op | tr '[:lower:]' '[:upper:]' | tr '\-' '\_')
        ^-- SC2039: In POSIX sh, 'local' is undefined.
              ^-- SC2155: Declare and assign separately to avoid masking return values.
                         ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 71:
        local v="CFG_${uop}"
        ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 72:
        eval $v="$default"
             ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 77:
                local val=$(echo "$arg" | cut -f2 -d=)
                ^-- SC2039: In POSIX sh, 'local' is undefined.
                      ^-- SC2155: Declare and assign separately to avoid masking return values.
 
Line 78:
                eval $v=$val
                     ^-- SC2086: Double quote to prevent globbing and word splitting.
                        ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 81:
        putvar $v
               ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 95:
    local op=$1
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 96:
    local default=$2
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 99:
    local doc="$*"
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 100:
    local flag=""
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 102:
    if [ $default -eq 0 ]
         ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 110:
    if [ $HELP -eq 0 ]
         ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 116:
                op=$(echo $op | tr 'a-z-' 'A-Z_')
                          ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 117:
                flag=$(echo $flag | tr 'a-z' 'A-Z')
                                       ^-- SC2018: Use '[:lower:]' to support accents and foreign alphabets.
                                             ^-- SC2019: Use '[:upper:]' to support accents and foreign alphabets.
 
Line 118:
                local v="CFG_${flag}_${op}"
                ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 119:
                eval $v=1
                     ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 120:
                putvar $v
                       ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 135:
    local op=$1
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 137:
    local doc="$*"
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 139:
    if [ $HELP -eq 0 ]
         ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 145:
                op=$(echo $op | tr 'a-z-' 'A-Z_')
                          ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 146:
                local v="CFG_${op}"
                ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 147:
                eval $v=1
                     ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 148:
                putvar $v
                       ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 163:
        local is_arg_valid=0
        ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 166:
            if test --disable-$option = $arg
                              ^-- SC2086: Double quote to prevent globbing and word splitting.
                                        ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 170:
            if test --enable-$option = $arg
                             ^-- SC2086: Double quote to prevent globbing and word splitting.
                                       ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 174:
            if test --$option = $arg
                      ^-- SC2086: Double quote to prevent globbing and word splitting.
                                ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 203:
    local path="$1"
    ^-- SC2039: In POSIX sh, 'local' is undefined.
 
Line 221:
CFG_ARGS="$@"
         ^-- SC2124: Assigning an array to a string! Assign as array, or use * instead of @ to concatenate.
 
Line 237:
OPTIONS=""
^-- SC2034: OPTIONS appears unused. Verify it or export it.
 
Line 262:
src_dir="$(abs_path $(dirname "$0"))"
                    ^-- SC2046: Quote this to prevent word splitting.
 
Line 264:
rust_installer_version=`cat "$src_dir/rust-installer-version"`
                       ^-- SC2006: Use $(..) instead of legacy `..`.
 
Line 274:
rm -Rf "$CFG_WORK_DIR/$CFG_PACKAGE_NAME"
       ^-- SC2115: Use "${var:?}" to ensure this never expands to / .
 
Line 284:
manifest=`(cd "$CFG_WORK_DIR/$CFG_PACKAGE_NAME/$CFG_COMPONENT_NAME" && find . -type f | sed 's/^\.\///') | sort`
         ^-- SC2006: Use $(..) instead of legacy `..`.
 
Line 287:
bulk_dirs=`echo "$CFG_BULK_DIRS" | tr "," " "`
          ^-- SC2006: Use $(..) instead of legacy `..`.
 
Line 289:
    bulk_dir=`echo "$bulk_dir" | sed s/\\\//\\\\\\\\\\\//g`
             ^-- SC2006: Use $(..) instead of legacy `..`.
 
Line 290:
    manifest=`echo "$manifest" | sed /^$bulk_dir/d`
             ^-- SC2006: Use $(..) instead of legacy `..`.
                                       ^-- SC2086: Double quote to prevent globbing and word splitting.
 
Line 294:
manifest=`echo "$manifest" | sed /^$/d | sed s/^/file:/`
         ^-- SC2006: Use $(..) instead of legacy `..`.
 
Line 298:
    manifest=`echo "$manifest" && echo "dir:$bulk_dir"`
             ^-- SC2006: Use $(..) instead of legacy `..`.
 
Line 303:
manifest=`echo "$manifest" | sed /^$/d`
         ^-- SC2006: Use $(..) instead of legacy `..`.
 
Line 320:
    overlay_files=`(cd "$CFG_NON_INSTALLED_OVERLAY" && find . -type f)`
                  ^-- SC2006: Use $(..) instead of legacy `..`.

$ 
