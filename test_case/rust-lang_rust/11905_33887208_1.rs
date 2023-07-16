
do_thing_that_needs_to_be_reversed();
{
    finally!(reverse_thing());
    do_other_stuff();
}
