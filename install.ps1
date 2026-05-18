# Aster Browser Automated Installer
# Automatically clones, builds, and installs Aster to %LOCALAPPDATA%\Programs\Aster
# Data is saved to %APPDATA%\Aster

$ErrorActionPreference = "Stop"

$tempDir = Join-Path $env:TEMP "AsterInstall_$([guid]::NewGuid().ToString().Substring(0,8))"
if (Test-Path $tempDir) { Remove-Item -Recurse -Force $tempDir }

Write-Host "📥 Cloning Aster from GitHub..." -ForegroundColor Cyan
git clone "https://github.com/ahyanistheEmty/Aster" $tempDir

# Save original location
$originalLocation = Get-Location

# Move into the cloned repo
Set-Location $tempDir

Write-Host "🚀 Building Aster in Release mode..." -ForegroundColor Cyan
cargo build --release

# Determine installation directory (Local AppData Programs folder)
$localAppData = [System.Environment]::GetFolderPath('LocalApplicationData')
$installDir = Join-Path $localAppData "Programs\Aster"

Write-Host ""
Write-Host "=========================================="
Write-Host " Aster Browser Installation"
Write-Host "=========================================="
$userInput = Read-Host "Enter installation path (Press Enter to use default: $installDir)"

if (![string]::IsNullOrWhiteSpace($userInput)) {
    $installDir = $userInput
}

if (!(Test-Path $installDir)) {
    Write-Host "📁 Creating installation directory at $installDir..."
    New-Item -ItemType Directory -Force -Path $installDir | Out-Null
}

$exeSource = Join-Path $tempDir "target\release\Aster.exe"
$exeDest = Join-Path $installDir "Aster.exe"

Write-Host "📦 Copying executable to $installDir..."
Copy-Item -Path $exeSource -Destination $exeDest -Force

Write-Host "🔗 Creating shortcuts..."
$WshShell = New-Object -comObject WScript.Shell

# Desktop Shortcut
$desktopPath = [System.Environment]::GetFolderPath('Desktop')
$desktopShortcut = $WshShell.CreateShortcut((Join-Path $desktopPath "Aster.lnk"))
$desktopShortcut.TargetPath = $exeDest
$desktopShortcut.WorkingDirectory = $installDir
$desktopShortcut.Description = "Aster Browser"
$desktopShortcut.Save()

# Start Menu Shortcut
$startMenuPrograms = [System.Environment]::GetFolderPath('Programs')
$startMenuShortcut = $WshShell.CreateShortcut((Join-Path $startMenuPrograms "Aster.lnk"))
$startMenuShortcut.TargetPath = $exeDest
$startMenuShortcut.WorkingDirectory = $installDir
$startMenuShortcut.Description = "Aster Browser"
$startMenuShortcut.Save()

Write-Host "🧹 Cleaning up temporary files..."
Set-Location $originalLocation
Remove-Item -Recurse -Force $tempDir

Write-Host ""
Write-Host "✅ Installation Complete!" -ForegroundColor Green
Write-Host "Aster has been installed to: $installDir"
Write-Host "Your browsing state & profiles will automatically save to your roaming profile (%APPDATA%\Aster)."
Write-Host "You can now launch Aster from your Desktop or Start Menu!" -ForegroundColor Green
