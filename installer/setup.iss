; Inno Setup Installer Script for Just Music
#define MyAppName "Just Music"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Just Music Team"
#define MyAppURL "https://github.com/Amlaach"
#define MyAppExeName "AetherPlayer.exe"

[Setup]
AppId={{8A14B823-9F93-4B83-A92B-6C23F923B411}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\Just Music
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=..\LICENSE
OutputDir=..\target\installer
OutputBaseFilename=JustMusic_Setup_v1.0.0
SetupIconFile=..\assets\icon.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "fileassoc"; Description: "Register Just Music as default player for supported audio formats (.mp3, .flac, .wav, .aac, .ogg, .m4a)"; GroupDescription: "File Associations:"

[Files]
Source: "..\target\release\just-music.exe"; DestDir: "{app}"; DestName: "{#MyAppExeName}"; Flags: ignoreversion
Source: "..\assets\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icon.ico"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icon.ico"; Tasks: desktopicon

[Registry]
Root: HKCU; Subkey: "Software\Classes\.mp3"; ValueType: string; ValueValue: "JustMusic.AudioFile"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.flac"; ValueType: string; ValueValue: "JustMusic.AudioFile"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.wav"; ValueType: string; ValueValue: "JustMusic.AudioFile"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.aac"; ValueType: string; ValueValue: "JustMusic.AudioFile"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.ogg"; ValueType: string; ValueValue: "JustMusic.AudioFile"; Flags: uninsdeletevalue; Tasks: fileassoc
Root: HKCU; Subkey: "Software\Classes\.m4a"; ValueType: string; ValueValue: "JustMusic.AudioFile"; Flags: uninsdeletevalue; Tasks: fileassoc

Root: HKCU; Subkey: "Software\Classes\JustMusic.AudioFile"; ValueType: string; ValueValue: "Just Music Audio File"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\JustMusic.AudioFile\DefaultIcon"; ValueType: string; ValueValue: "{app}\{#MyAppExeName},0"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\JustMusic.AudioFile\shell\open\command"; ValueType: string; ValueValue: """{app}\{#MyAppExeName}"" ""%1"""; Flags: uninsdeletekey

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent
