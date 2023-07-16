python
 with bootstrap.output('Makefile') as f: 
     template = os.path.join(rust_dir, 'src', 'bootstrap', 'mk', 'Makefile.in') 
     contents = "BOOTSTRAP_ARGS := " + " ".join(sys.argv[1:]) + "\n"
     contents += open(template).read() 
