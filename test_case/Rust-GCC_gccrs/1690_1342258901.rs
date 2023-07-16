
{ dg-error regexp [comment [{ target/xfail selector } [line] ]] }

    This DejaGnu directive appears on a source line that is expected to get an error message, or else specifies the source line 
associated with the message. If there is no message for that line or if the text of that message is not matched by regexp then the 
check fails and comment is included in the FAIL message. The check does not look for the string ‘error’ unless it is part of regexp.

