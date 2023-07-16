
fn main() -> io::Result<()> {
fn main() -> Result<(), Box<dyn error::Error>> {
fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
fn main() -> Result<(), String> {
