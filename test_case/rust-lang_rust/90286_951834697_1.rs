sh
#!/bin/sh

cargo check 2>&1 | grep " ^^^ move occurs because"
