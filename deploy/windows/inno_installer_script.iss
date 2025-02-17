[Setup]
AppName=Casdesk
AppVersion=1.1.0
DefaultDirName={autopf}\Casdesk
DefaultGroupName=Casdesk
OutputDir=.
OutputBaseFilename=Casdesk_1.1.0_Installer
Compression=lzma
SolidCompression=yes

[Files]
Source: "C:\Users\relay\proj\casdesk\target\dx\casdesk\release\windows\app\casdesk.exe"; DestDir: "{app}"
Source: "C:\Users\relay\proj\casdesk\target\dx\casdesk\release\windows\app\assets\*"; DestDir: "{app}\assets"; Flags: recursesubdirs

[Icons]
Name: "{group}\Casdesk"; Filename: "{app}\casdesk.exe"
Name: "{commondesktop}\Casdesk"; Filename: "{app}\casdesk.exe"

[Run]
Filename: "{app}\Casdesk.exe"; Description: "Launch Casdesk"; Flags: nowait postinstall skipifsilent
