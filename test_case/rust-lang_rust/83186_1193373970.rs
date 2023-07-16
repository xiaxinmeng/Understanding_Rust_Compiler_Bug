
As opposed to the Path::exists method, this one doesn’t silently ignore errors
unrelated to the path not existing. (E.g. it will return Err(_) in case of permission
denied on some of the parent directories.) 
