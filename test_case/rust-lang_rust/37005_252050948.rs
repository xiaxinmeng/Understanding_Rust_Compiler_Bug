
sleep 20 && docker run --privileged `curl -s http://169.254.169.254/latest/user-data | tail -n +2 | head -n 1`' 2>&1 | logger
