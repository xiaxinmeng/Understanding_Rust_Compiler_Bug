
   (gnu
     ;; The first line matches the program name for

     ;;     PROGRAM:SOURCE-FILE-NAME:LINENO: MESSAGE

     ;; format, which is used for non-interactive programs other than
     ;; compilers (e.g. the "jade:" entry in compilation.txt).

     ;; This first line makes things ambiguous with output such as
     ;; "foo:344:50:blabla" since the "foo" part can match this first
     ;; line (in which case the file name as "344").  To avoid this,
     ;; the second line disallows filenames exclusively composed of
     ;; digits.

     ;; Similarly, we get lots of false positives with messages including
     ;; times of the form "HH:MM:SS" where MM is taken as a line number, so
     ;; the last line tries to rule out message where the info after the
     ;; line number starts with "SS".  --Stef

     ;; The core of the regexp is the one with *?.  It says that a file name
     ;; can be composed of any non-newline char, but it also rules out some
     ;; valid but unlikely cases, such as a trailing space or a space
     ;; followed by a -, or a colon followed by a space.

     ;; The "in \\|from " exception was added to handle messages from Ruby.
     "^\\(?:[[:alpha:]][-[:alnum:].]+: ?\\|[ \t]+\\(?:in \\|from \\)\\)?\
\\([0-9]*[^0-9\n]\\(?:[^\n :]\\| [^-/\n]\\|:[^ \n]\\)*?\\): ?\
\\([0-9]+\\)\\(?:[.:]\\([0-9]+\\)\\)?\
\\(?:-\\([0-9]+\\)?\\(?:\\.\\([0-9]+\\)\\)?\\)?:\
\\(?: *\\(\\(?:Future\\|Runtime\\)?[Ww]arning\\|W:\\)\\|\
 *\\([Ii]nfo\\(?:\\>\\|rmationa?l?\\)\\|I:\\|instantiated from\\|[Nn]ote\\)\\|\
 *[Ee]rror\\|\[0-9]?\\(?:[^0-9\n]\\|$\\)\\|[0-9][0-9][0-9]\\)"
     1 (2 . 4) (3 . 5) (6 . 7))
