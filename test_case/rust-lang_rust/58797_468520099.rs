
[dependencies]
 
regex = "1"
serde_derive ="*"
serde_json="*"
serde="*"

reqwest = "0.9.5"
zip = "0.5.0"
tempfile = "3"
failure = "0.1.5"
log = "0.4.0"
env_logger = "0.6.0"


actix-web = "0.7.*"
futures = "*"
bytes = "*"
lazy_static = "1.2.0"
chrono = { version = "0.4", features = ["serde"] }
subparse = "*"
# postgres = "0.15"
uuid="0.6"
diesel = { version = "1.3.3", features = ["postgres","chrono","uuid","serde_json"] }
dotenv = "0.13.0"
tera = "0.11"
glob = "0.2"
image = "*"

walkdir = "2"
crossbeam-channel = "0.3"
#needed for postgres by musl
openssl = "*"

rusoto_core = "0.36.0"
rusoto_s3 = "0.36.0"
rusoto_ec2 =  "0.36.0"
rusoto_ecs =  "0.36.0"

google-youtube3 = "*"
# This project intentionally uses an old version of Hyper. See
# https://github.com/Byron/google-apis-rs/issues/173 for more
# information.
hyper = "^0.10"
hyper-rustls = "^0.6"
yup-oauth2 = "^1.0"

raster = "0.2.0"
rusttype="*"

rand = "0.6.*"
