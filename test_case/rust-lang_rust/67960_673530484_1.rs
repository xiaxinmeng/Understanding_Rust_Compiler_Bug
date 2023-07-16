none
./ffmpeg/ffmpeg -h encoder=librav1e
ffmpeg version git-2020-08-12-bb59bdb Copyright (c) 2000-2020 the FFmpeg developers
  built with gcc 10.2.0 (Rev1, Built by MSYS2 project)
  configuration: --enable-librav1e --disable-shared --enable-static --disable-ffprobe --pkg-config-flags=--static
  libavutil      56. 58.100 / 56. 58.100
  libavcodec     58.100.100 / 58.100.100
  libavformat    58. 50.100 / 58. 50.100
  libavdevice    58. 11.101 / 58. 11.101
  libavfilter     7. 87.100 /  7. 87.100
  libswscale      5.  8.100 /  5.  8.100
  libswresample   3.  8.100 /  3.  8.100
Encoder librav1e [librav1e AV1]:
    General capabilities: delay threads
    Threading capabilities: auto
    Supported pixel formats: yuv420p yuvj420p yuv420p10le yuv420p12le yuv422p yuvj422p yuv422p10l420p yuv420p10le yuv420p12le yuv422p yuvje yuv444p12le422p yuv422p10le yuv422p12le yuv444p yuvj444p yuv444p10le yuv444p12le             ... use constant quantizer mode (from -1 to 255) (defaul
librav1e AVOptions:
  -qp                <int>        E..V...... what speed preset to use (from -1 to 10) (default -1... use constant quantizer mode (from -1
to 255) (default -1)                     ... number of tiles encode with (from -1 to I64_MAX) (de
  -speed             <int>        E..V...... what speed preset to use (from -1 to 10) (default -1)
  -tiles             <int>        E..V...... number of tiles encode with (from -1 to I64_MAX) (default 0)
  -tile-rows         <int>        E..V...... number of tiles rows to encode with (from -1 to I64_MAX) (default 0)
  -tile-columns      <int>        E..V...... number of tiles columns to encode with (from -1 to I64_MAX) (default 0)
  -rav1e-params      <dictionary> E..V...... set the rav1e configuration using a :-separated list of key=value parameters
