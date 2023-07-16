
uint32_t *array;
size_t array_length;
for(size_t i = 0; i < array_length; i++)
{
    uint8_t *bytes = &array[i];
    array[i] = (bytes[0]<<24) | (bytes[1]<<16) | (bytes[2]<<8) | (bytes[3])
}
