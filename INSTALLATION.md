# LinkWithMentor Installation Guide

## Windows Installation

### Option 1: Windows Installer (Recommended)

1. Download `LinkWithMentor-Setup.exe` from the [Releases](https://github.com/mg3994/lwm-gtk4-rust/releases) page
2. Run the installer as Administrator
3. Follow the installation wizard
4. Launch from Start Menu or Desktop shortcut

The installer automatically:
- Installs all GTK4 dependencies
- Creates shortcuts
- Adds to Windows Add/Remove Programs
- Sets up proper environment variables

### Option 2: Portable ZIP Bundle

1. Download the latest `LinkWithMentor-Windows-x64.zip` from the [Releases](https://github.com/mg3994/lwm-gtk4-rust/releases) page
2. Extract the ZIP file to a folder of your choice (e.g., `C:\Program Files\LinkWithMentor\`)
3. Run `LinkWithMentor.bat` to start the application

The release bundle includes all necessary GTK4 libraries and dependencies.

### Option 2: Direct Executable

If you want to run `linkwithmentor.exe` directly, you'll need to install GTK4 on your system:

1. Install MSYS2 from https://www.msys2.org/
2. Open MSYS2 terminal and run:
   ```bash
   pacman -S mingw-w64-x86_64-gtk4
   ```
3. Add `C:\msys64\mingw64\bin` to your system PATH
4. Run `linkwithmentor.exe`

## Troubleshooting

### "DLL not found" errors

If you see errors like "libgio-2.0-0.dll was not found":

1. Make sure you're using the complete release bundle
2. Run `LinkWithMentor.bat` instead of the `.exe` directly
3. If the issue persists, install GTK4 system-wide using Option 2 above

### Application won't start

1. Check that all files from the ZIP were extracted
2. Try running from Command Prompt to see error messages:
   ```cmd
   cd "path\to\extracted\folder"
   LinkWithMentor.bat
   ```

### Performance Issues

- The application works best on Windows 10/11
- Ensure your graphics drivers are up to date
- Try running with different GTK themes by modifying the `.bat` file

## Building from Source

See [README.md](README.md) for development setup instructions.