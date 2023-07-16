 C#
try {
   ... complex array code for, say, decoding a packet...
} catch( IndexOutOfRangeException )
{
    return ... "invalid data format"...;
}
