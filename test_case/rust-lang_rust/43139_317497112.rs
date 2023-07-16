
<marmistrz> I'm wondering if there's a slightest possibility of this affecting any user space processes
<Saturn> it is not. kernel tasks do not have pids at all
<Saturn> the wiki page you linked is talking strictly about what ps is showing, which is no longer true anyway
<Saturn> most importantly, http://pubs.opengroup.org/onlinepubs/9699919799/ defines process IDs as positive
<Saturn> er, http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap03.html#tag_03_300 specifically
