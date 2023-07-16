 shell
$ egrep -v '^[ ]*#' confusables.txt | egrep -v '^[ ]+$|^$' | egrep '00(2[1-9A-F]|3[A-F]|40|5[BCDF]|7[BCD])' | egrep ';[^0-9A-F]00.. ;' | egrep -v '^00' > confusables-ascii-punc.txt
