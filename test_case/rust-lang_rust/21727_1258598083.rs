
$RustupToolchains = Resolve-Path "~/.rustup/toolchains"
$ToolchainPrefix = Join-Path $RustupToolchains "prebuilt"
New-Item -ItemType Directory -Path $ToolchainPrefix | Out-Null
$DownloadPath = "/tmp/rust-download"
New-Item -ItemType Directory -Path $DownloadPath | Out-Null
Push-Location
Set-Location $DownloadPath
@(
  "rustc-1.64.0-x86_64-apple-darwin.tar.xz",
  "rustc-dev-1.64.0-x86_64-apple-darwin.tar.xz",
  "rust-dev-1.64.0-x86_64-apple-darwin.tar.xz",
  "rust-std-1.64.0-x86_64-apple-darwin.tar.xz"
) | % {
  Write-Host "Downloading $_"
  wget -q https://github.com/awakecoding/llvm-prebuilt/releases/download/v2022.2.0/$_
}
@(
  "rust-std-1.64.0-aarch64-apple-darwin.tar.xz",
  "rust-std-1.64.0-x86_64-apple-ios.tar.xz",
  "rust-std-1.64.0-aarch64-apple-ios.tar.xz"
) | % {
  Write-Host "Downloading $_"
  wget -q https://static.rust-lang.org/dist/2022-09-22/$_
}
Get-Item *.tar.xz | % {
  $FileName = $_.Name
  $DirName = $FileName -Replace ".tar.xz",""
  Write-Host "Extracting $FileName"
  tar -xf $_
  & "./${DirName}/install.sh" "--prefix=$ToolchainPrefix"
}
Pop-Location
rustup default prebuilt
