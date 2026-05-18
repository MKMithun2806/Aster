; ==========================================
; Aster Web Browser - Inno Setup Installer Script
; ==========================================

[Setup]
AppName=Aster
AppVersion=0.1.0
AppPublisher=Google DeepMind Advanced Agentic Coding
DefaultDirName={autopf}\Aster
DefaultGroupName=Aster
UninstallDisplayIcon={app}\Aster.exe
Compression=lzma2/max
SolidCompression=yes
OutputDir=target\installer
OutputBaseFilename=AsterSetup
SetupIconFile=assets\aster.ico
PrivilegesRequired=admin
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64

[Files]
Source: "target\release\Aster.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Aster"; Filename: "{app}\Aster.exe"
Name: "{autodesktop}\Aster"; Filename: "{app}\Aster.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop shortcut"; GroupDescription: "Additional shortcuts:"; Flags: unchecked

[Run]
Filename: "{app}\Aster.exe"; Description: "Launch Aster Browser"; Flags: nowait postinstall skipifsilent
