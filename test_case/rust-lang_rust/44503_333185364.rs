css
/* =============== Rust Header Menu CSS ================= */

span#rust-top-links {
    display: block !important;
    margin: 5px auto 0 auto;
    max-width: 1110px;
}
div#rust-navbar {
    z-index: 1040;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
}
body[class*="archetype-"] #rust-navbar {
    position: absolute;
}
.rust-nav-link-container li {
    display: inline-block;
    float: left;
}
.rust-nav-link-container a {
    text-decoration: none;
    color: #428bca;
    text-align: center;
    line-height: 48px;
    font-size: 14px;
    padding: 1em;
}
.rust-nav-link-container a.active {
    font-weight: bold;
}
.rust-nav-link-container a:hover,
.rust-nav-link-container a:active,
.rust-nav-link-container a:focus {
    text-decoration: underline;
    color: #2a6496;
}
.rust-nav-link-container {
    display: block;
    overflow: hidden;
    margin-left: 60px;
    position: absolute;
}
@media screen and (max-width: 800px) {
    .js div#rust-navbar {
        position: static;
        margin-top: -1em;
        margin-bottom: 1em;
    }
    .rust-nav-link-container {
        position: static;
        margin-left: 0;
    }
}
