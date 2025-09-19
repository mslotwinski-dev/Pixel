[Setup]
AppName=Pixel
AppVersion=1.0.0
DefaultDirName={pf}\Pixel
DefaultGroupName=Pixel
UninstallDisplayIcon={app}\pixel.exe
OutputBaseFilename=PixelInstaller

[Files]
Source: "target\release\pixel.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Pixel"; Filename: "{app}\pixel.exe"
Name: "{commondesktop}\Pixel"; Filename: "{app}\pixel.exe"

[Registry]
; Definicja aplikacji PixelApp
Root: HKCR; Subkey: "PixelApp"; ValueType: string; ValueName: ""; ValueData: "Pixel"
Root: HKCR; Subkey: "PixelApp"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "Pixel"
Root: HKCR; Subkey: "PixelApp\DefaultIcon"; ValueType: string; ValueData: """{app}\Pixel.exe"",0"
Root: HKCR; Subkey: "PixelApp\shell\open\command"; ValueType: string; ValueData: """{app}\Pixel.exe"" ""%1"""

Root: HKCR; Subkey: ".jpg\OpenWithProgids"; ValueType: string; ValueName: "PixelApp"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".jpeg\OpenWithProgids"; ValueType: string; ValueName: "PixelApp"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".png\OpenWithProgids"; ValueType: string; ValueName: "PixelApp"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".bmp\OpenWithProgids"; ValueType: string; ValueName: "PixelApp"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".gif\OpenWithProgids"; ValueType: string; ValueName: "PixelApp"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".tiff\OpenWithProgids"; ValueType: string; ValueName: "PixelApp"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".webp\OpenWithProgids"; ValueType: string; ValueName: "PixelApp"; ValueData: ""; Flags: uninsdeletevalue

Root: HKCU; Subkey: "Environment"; \
   ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; \
   Flags: preservestringtype uninsdeletevalue