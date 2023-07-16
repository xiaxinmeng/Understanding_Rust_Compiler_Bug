 c++
size_t countlines(void)
  {
    int fd = 0;
    ssize_t bytes_read;
    char buf[BUFFER_SIZE + 1];
    size_t lines = 0;
  
    while ((bytes_read = read (fd, buf, BUFFER_SIZE)) > 0)
      {
        char *p = buf;
        char *end = buf + bytes_read;          
        while (p != end)
        {
          lines += *p++ == '\n';
        }
      }
 return lines;
}

