
"pre-link-args": {
    "lld-link": [
      "-subsystem:windows",
      "-fixed",
      "-base:0x00010000",
      "-stack:65536",
      "-merge:.edata=.edataxb"
    ]
  },
