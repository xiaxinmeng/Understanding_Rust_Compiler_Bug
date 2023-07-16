javascript
const path = new URL(window.location.href).pathname
const firstComponent = path.split('/')[0]
const fixedVersions = ["stable", "beta", "nightly"]
const includeFirst = fixedVersions.includes(firstComponent) || /^[0-9]/.test(firstComponent)

// inside the search submit:

document.location.href = (includeFirst ? "/" + firstComponent : "") + "/std/index.html?search=" ...
