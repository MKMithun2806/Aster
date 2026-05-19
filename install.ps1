# Aster Browser installer for Windows
# Clones the repository, builds a release binary, and installs it under Local AppData by default.

$ErrorActionPreference = "Stop"

function Assert-Command {
    param([Parameter(Mandatory = $true)][string]$Name)
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Required command '$Name' was not found on PATH."
    }
}

Assert-Command git
Assert-Command cargo

$repoUrl = "https://github.com/ahyanistheEmty/Aster.git"
$tempDir = Join-Path $env:TEMP ("AsterInstall_{0}" -f ([guid]::NewGuid().ToString("N").Substring(0, 8)))
$originalLocation = Get-Location

try {
    if (Test-Path $tempDir) {
        Remove-Item -Recurse -Force $tempDir
    }

    Write-Host "Cloning Aster from GitHub..." -ForegroundColor Cyan
    git clone --depth 1 $repoUrl $tempDir | Out-Null

    Set-Location $tempDir

    Write-Host "Building Aster in release mode..." -ForegroundColor Cyan
    cargo build --release

    $defaultInstallDir = Join-Path ([System.Environment]::GetFolderPath("LocalApplicationData")) "Programs\Aster"
    Write-Host ""
    Write-Host "Aster Browser Installation" -ForegroundColor Cyan
    Write-Host "Default install path: $defaultInstallDir"
    $userInput = Read-Host "Enter installation path or press Enter to accept the default"
    $installDir = if ([string]::IsNullOrWhiteSpace($userInput)) { $defaultInstallDir } else { $userInput }

    if (-not (Test-Path $installDir)) {
        New-Item -ItemType Directory -Force -Path $installDir | Out-Null
    }

    $exeSource = Join-Path $tempDir "target\release\Aster.exe"
    $exeDest = Join-Path $installDir "Aster.exe"
    Copy-Item -Force $exeSource $exeDest

    $desktopPath = [System.Environment]::GetFolderPath("Desktop")
    $startMenuPath = [System.Environment]::GetFolderPath("Programs")
    $wshShell = New-Object -ComObject WScript.Shell

    foreach ($shortcutPath in @(
        (Join-Path $desktopPath "Aster.lnk"),
        (Join-Path $startMenuPath "Aster.lnk")
    )) {
        if ([string]::IsNullOrWhiteSpace($shortcutPath)) {
            continue
        }
        $shortcutDir = Split-Path -Parent $shortcutPath
        if (-not (Test-Path $shortcutDir)) {
            continue
        }
        $shortcut = $wshShell.CreateShortcut($shortcutPath)
        $shortcut.TargetPath = $exeDest
        $shortcut.WorkingDirectory = $installDir
        $shortcut.Description = "Aster Browser"
        $shortcut.Save()
    }

    Write-Host ""
    Write-Host "Installation complete." -ForegroundColor Green
    Write-Host "Binary: $exeDest"
    Write-Host "State:  %APPDATA%\Aster"
}
finally {
    Set-Location $originalLocation
    if (Test-Path $tempDir) {
        Remove-Item -Recurse -Force $tempDir
    }
}
