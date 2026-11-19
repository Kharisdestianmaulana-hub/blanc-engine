#ifndef MyAppVersion
  #define MyAppVersion "0.1.1"
#endif

#ifndef StagingDir
  #define StagingDir "..\dist\staging"
#endif

#ifndef OutputDir
  #define OutputDir "..\dist"
#endif

#ifndef AppIconFile
  #define AppIconFile "..\assets\icons\blanc_engine.ico"
#endif

[Setup]
AppId={{8F6E6D22-5A4C-4F8B-9D1C-6B2F4A1E9C10}
AppName=Blanc Engine
AppVersion={#MyAppVersion}
AppVerName=Blanc Engine v{#MyAppVersion}
AppPublisher=Blanc Engine Contributors
DefaultDirName={autopf}\Blanc Engine
DefaultGroupName=Blanc Engine
AllowNoIcons=yes
SetupIconFile={#AppIconFile}
UninstallDisplayIcon={app}\BlancEngine.exe
OutputDir={#OutputDir}
OutputBaseFilename=BlancEngine-{#MyAppVersion}-Setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#StagingDir}\BlancEngine.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#StagingDir}\blanc_runtime.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#StagingDir}\blanc_installer.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Blanc Engine"; Filename: "{app}\BlancEngine.exe"; IconFilename: "{app}\BlancEngine.exe"
Name: "{group}\{cm:UninstallProgram,Blanc Engine}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\Blanc Engine"; Filename: "{app}\BlancEngine.exe"; IconFilename: "{app}\BlancEngine.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\BlancEngine.exe"; Description: "{cm:LaunchProgram,Blanc Engine}"; Flags: nowait postinstall skipifsilent
