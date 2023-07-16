
static linted_error parse_int(char const *str, int *resultp)
{
    linted_error err = 0;

    char start = str[0U];

    if (isspace(start))
        return EINVAL;
    if ('+' == start)
        return EINVAL;
    if ('-' == start)
        return ERANGE;

    errno = 0;
    long yy;
    char *endptr;
    {
        char *xx;
        yy = strtol(str, &xx, 10);
        endptr = xx;
    }
    err = errno;
    if (err != 0)
        return err;

    if (*endptr != '\0')
        return EINVAL;
    if (yy > INT_MAX)
        return ERANGE;

    *resultp = yy;
    return 0;
}
