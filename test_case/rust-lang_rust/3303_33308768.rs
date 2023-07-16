 python
#!/usr/bin/env python

import sys, fileinput, subprocess, re, requests, json

def report_error_name_no(name, no, s):
    global err
    print("%s:%d: %s" % (name, no, s))  
    err=1

def report_err(s):
    report_error_name_no(fileinput.filename(), fileinput.filelineno(), s)

def report_warn(s):
    print("%s:%d: %s" % (fileinput.filename(), fileinput.filelineno(), s))

file_names = [s for s in sys.argv[1:] if (not s.endswith("_gen.rs")) 
    and (not ".#" in s)]

try:
    for line in fileinput.input(file_names, inplace=1):
        if "// FIXME" in line:
            comment = line.partition(':')[2]
            url = 'https://api.github.com/repos/mozilla/rust/issues'
            payload = {
                'title' : comment,
                'body' : ''
            }
            headers = { 'content-type' : 'application/json' }
            r = requests.post(url, data=json.dumps(payload), headers=headers)

            if r.status_code == 201:
                sys.stdout.write(line.replace("FIXME", "FIXME(#" + r['number'] + ")")
            else:
                sys.stdout.write(line)
        else:
             sys.stdout.write(line)

except UnicodeDecodeError, e:
    report_err("UTF-8 decoding error " + str(e))


sys.exit(err)
