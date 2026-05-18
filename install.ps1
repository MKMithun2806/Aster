# Aster Browser Installer
# Installs Aster natively to %APPDATA%\Aster and creates shortcuts.

$ErrorActionPreference = "Stop"

Write-Host "🚀 Building Aster in Release mode..." -ForegroundColor Cyan
cargo build --release

$appDataPath = [System.Environment]::GetFolderPath('ApplicationData')
$installDir = Join-Path $appDataPath "Aster"

if (!(Test-Path $installDir)) {
    Write-Host "📁 Creating installation directory at $installDir..."
    New-Item -ItemType Directory -Force -Path $installDir | Out-Null
}

$exeSource = "target\release\Aster.exe"
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
$startMenuPath = [System.Environment]::GetFolderPath('Programs')
$startMenuShortcut = $WshShell.CreateShortcut((Join-Path $startMenuPath "Aster.lnk"))
$startMenuShortcut.TargetPath = $exeDest
$startMenuShortcut.WorkingDirectory = $installDir
$startMenuShortcut.Description = "Aster Browser"
$startMenuShortcut.Save()

Write-Host "✅ Installation Complete! You can now launch Aster from your Desktop or Start Menu." -ForegroundColor Green
