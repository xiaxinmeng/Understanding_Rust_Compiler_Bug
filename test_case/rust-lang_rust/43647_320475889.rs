sh
#!/bin/sh                                                                                                                                                                                                                                              
exec "${REALGCC:-gcc}" "$@" -specs "$HOME/no-pie-compile.specs" -specs "$HOME/no-pie-link.specs"
