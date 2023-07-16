 diff
8825    +           /*
8826    +            * If the previous line ends on '[' we are probably in an
8827    +            * array constant:
8828    +            * something = [
8829    +            *     234,  <- extra indent
8830    +            */
8831    +           if (cin_ends_in(l, (char_u *)"[", NULL))
8832    +           {
8833    +               amount = get_indent() + ind_continuation;
8834    +               break;
8835    +           }
