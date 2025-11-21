; LinkWithMentor Windows Installer Script
; Uses NSIS (Nullsoft Scriptable Install System)

!define APP_NAME "LinkWithMentor"
!define APP_VERSION "1.0.3"
!define APP_PUBLISHER "LinkWithMentor Team"
!define APP_URL "https://github.com/mg3994/lwm-gtk4-rust"
!define APP_EXECUTABLE "linkwithmentor.exe"

; Installer properties
Name "${APP_NAME}"
OutFile "LinkWithMentor-Setup.exe"
InstallDir "$PROGRAMFILES64\${APP_NAME}"
InstallDirRegKey HKLM "Software\${APP_NAME}" "InstallDir"
RequestExecutionLevel admin

; Modern UI
!include "MUI2.nsh"
!define MUI_ABORTWARNING
; Use default icons if custom ones don't exist
!ifdef CUSTOM_ICON
!define MUI_ICON "resources\icon.ico"
!define MUI_UNICON "resources\icon.ico"
!endif

; Pages
!insertmacro MUI_PAGE_WELCOME
; Only show license page if LICENSE file exists
!ifdef LICENSE_FILE
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!endif
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

; Languages
!insertmacro MUI_LANGUAGE "English"

; Version Information
VIProductVersion "${APP_VERSION}.0"
VIAddVersionKey "ProductName" "${APP_NAME}"
VIAddVersionKey "ProductVersion" "${APP_VERSION}"
VIAddVersionKey "CompanyName" "${APP_PUBLISHER}"
VIAddVersionKey "FileDescription" "${APP_NAME} Installer"
VIAddVersionKey "FileVersion" "${APP_VERSION}"

Section "Main Application" SecMain
  SectionIn RO
  
  ; Set output path to the installation directory
  SetOutPath $INSTDIR
  
  ; Install main executable
  File "release-bundle\${APP_EXECUTABLE}"
  
  ; Install all DLL dependencies
  File "release-bundle\*.dll"
  
  ; Install resources
  SetOutPath "$INSTDIR\resources"
  File /r "release-bundle\resources\*"
  
  ; Install GTK schemas if they exist
  IfFileExists "release-bundle\share\*" 0 +3
  SetOutPath "$INSTDIR\share"
  File /r "release-bundle\share\*"
  
  ; Install documentation
  SetOutPath $INSTDIR
  File "release-bundle\README.md"
  File "release-bundle\INSTALLATION.md"
  IfFileExists "release-bundle\LICENSE" 0 +2
  File "release-bundle\LICENSE"
  
  ; Create launcher script
  FileOpen $0 "$INSTDIR\${APP_NAME}.bat" w
  FileWrite $0 "@echo off$\r$\n"
  FileWrite $0 "cd /d $\"$INSTDIR$\"$\r$\n"
  FileWrite $0 "set GTK_THEME=win32$\r$\n"
  FileWrite $0 "set GSETTINGS_SCHEMA_DIR=$\"$INSTDIR\share\glib-2.0\schemas$\"$\r$\n"
  FileWrite $0 "$\"$INSTDIR\${APP_EXECUTABLE}$\" %*$\r$\n"
  FileClose $0
  
  ; Write registry keys
  WriteRegStr HKLM "Software\${APP_NAME}" "InstallDir" $INSTDIR
  WriteRegStr HKLM "Software\${APP_NAME}" "Version" "${APP_VERSION}"
  
  ; Create uninstaller
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  
  ; Add to Add/Remove Programs
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayName" "${APP_NAME}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "UninstallString" "$INSTDIR\Uninstall.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayIcon" "$INSTDIR\${APP_EXECUTABLE}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "Publisher" "${APP_PUBLISHER}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "URLInfoAbout" "${APP_URL}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayVersion" "${APP_VERSION}"
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "NoModify" 1
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "NoRepair" 1
  
SectionEnd

Section "Desktop Shortcut" SecDesktop
  CreateShortcut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${APP_NAME}.bat" "" "$INSTDIR\${APP_EXECUTABLE}" 0
SectionEnd

Section "Start Menu Shortcuts" SecStartMenu
  CreateDirectory "$SMPROGRAMS\${APP_NAME}"
  CreateShortcut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${APP_NAME}.bat" "" "$INSTDIR\${APP_EXECUTABLE}" 0
  CreateShortcut "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
SectionEnd

; Section descriptions
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecMain} "Main application files (required)"
  !insertmacro MUI_DESCRIPTION_TEXT ${SecDesktop} "Create a desktop shortcut"
  !insertmacro MUI_DESCRIPTION_TEXT ${SecStartMenu} "Create Start Menu shortcuts"
!insertmacro MUI_FUNCTION_DESCRIPTION_END

Section "Uninstall"
  ; Remove files
  Delete "$INSTDIR\${APP_EXECUTABLE}"
  Delete "$INSTDIR\${APP_NAME}.bat"
  Delete "$INSTDIR\*.dll"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\INSTALLATION.md"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\Uninstall.exe"
  
  ; Remove directories
  RMDir /r "$INSTDIR\resources"
  RMDir /r "$INSTDIR\share"
  RMDir "$INSTDIR"
  
  ; Remove shortcuts
  Delete "$DESKTOP\${APP_NAME}.lnk"
  Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
  Delete "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk"
  RMDir "$SMPROGRAMS\${APP_NAME}"
  
  ; Remove registry keys
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}"
  DeleteRegKey HKLM "Software\${APP_NAME}"
  
SectionEnd