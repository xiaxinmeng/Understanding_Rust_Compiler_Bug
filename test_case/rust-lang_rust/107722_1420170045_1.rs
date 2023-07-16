python
with open(path, "wb") as outfile:
  run(["curl", option,
      "-L", # Follow redirect.
      "-y", "30", "-Y", "10",    # timeout if speed is < 10 bytes/sec for > 30 seconds
      "--connect-timeout", "30",  # timeout if cannot connect within 30 seconds
      "--retry", "3", "-Sf", url],  #modified line
      stdout=outfile,  #modified line
      verbose=verbose,
      exception=True, # Will raise RuntimeError on failure
  )
