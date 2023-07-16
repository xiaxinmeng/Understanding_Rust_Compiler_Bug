
% rustc /tmp/hw.rs -g
% rustc -g /tmp/hw.rs
error: no input filename given
% rustc -g=2 /tmp/hw.rs
error: debug info level needs to be between 0-2
% rustc -g 2 /tmp/hw.rs
% rustc -g2 /tmp/hw.rs
% 
