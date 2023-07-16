powershell
function x {
    $env:Path = ($env:Path -split ";" |
        % { $_.TrimEnd("\/").Trim() } |
        ? { $_ } |
        ? { $_ -NotLike "$env:ProgramFiles\WindowsApps\Microsoft.WindowsTerminal_*" } |
        select -Unique
    ) -join ";"

    $xPy = ,(gi .) + (gi . | Get-ParentDirectories) | % { gcm "$_\x.py" -EA 0 } | select -First 1
    if ($xPy) {
        pushd ($xPy.Source | Split-Path)
        try {
            python.exe $xPy.Source @Args
        } finally {
            popd
        }
    } else {
        Write-HostError "x.py not found. Please run inside of a checkout of `https://github.com/rust-lang/rust`."
    }
}
