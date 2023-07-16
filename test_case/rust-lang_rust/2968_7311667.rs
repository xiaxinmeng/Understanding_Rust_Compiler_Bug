
fn install_package(c: cargo, src: ~str, wd: ~str, pkg: package) {
    let url = copy pkg.url;
    let method = alt pkg.method {
        ~"git" { ~"git" }
        ~"file" { ~"file" }
        _ { ~"curl" }
    };

    info(#fmt["installing %s/%s via %s...", src, pkg.name, method]);

    alt method {
        ~"git" { install_git(c, wd, url, copy pkg.ref); }
        ~"file" { install_file(c, wd, url); }
        ~"curl" { install_curl(c, wd, copy url); }
        _ {}
    }
}
