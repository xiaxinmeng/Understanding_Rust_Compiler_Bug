
for n in line.find_str(modsearchstr+s).iter() {
    let end = find_end(line, n+modsearchstr.len());
    let l = line.slice(n + modsearchstr.len(), end);
    (*outputfn)(l, i, path, line);
}
