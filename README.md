# LinkWithMentor - GTK4 Social Media Application

A modern, feature-rich social media application built with GTK4 and Rust.

## 🎯 Features

- **Home Feed** - View and interact with posts from mentors and peers
- **Real-time Chat** - Direct messaging with audio/video call buttons
- **Groups** - Join and participate in community discussions
- **Media Gallery** - Share and view images, videos, documents
- **Content Sharing** - Share files, links, locations, and polls
- **User Profiles** - Manage your profile and view activity

## 🎨 Design

- **Glassmorphism effects** with backdrop blur
- **Vibrant gradients** (purple/pink theme)
- **Smooth animations** and transitions
- **Modern typography** using Inter font
- **Responsive layout** that adapts to window size

## 📋 Prerequisites

### Windows

You need to install GTK4 on Windows. Here are the steps:

#### Option 1: Using MSYS2 (Recommended)

1. **Install MSYS2** from https://www.msys2.org/
2. **Open MSYS2 MINGW64** terminal
3. **Install GTK4 and dependencies:**
   ```bash
   pacman -S mingw-w64-x86_64-gtk4
   pacman -S mingw-w64-x86_64-pkg-config
   pacman -S mingw-w64-x86_64-gcc
   ```

4. **Add MSYS2 to your PATH:**
   - Add `C:\msys64\mingw64\bin` to your System PATH environment variable
   - Restart your terminal/IDE

#### Option 2: Using vcpkg

1. **Install vcpkg:**
   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git
   cd vcpkg
   .\bootstrap-vcpkg.bat
   ```

2. **Install GTK4:**
   ```powershell
   .\vcpkg install gtk:x64-windows
   ```

3. **Set environment variables:**
   ```powershell
   $env:PKG_CONFIG_PATH = "C:\path\to\vcpkg\installed\x64-windows\lib\pkgconfig"
   ```

### Verify Installation

After installing GTK4, verify it's working:

```powershell
pkg-config --modversion gtk4
```

This should output the GTK4 version (e.g., `4.10.0` or similar).

## 🚀 Building and Running

### 1. Install Rust

If you haven't already:
```powershell
# Download and run rustup-init.exe from https://rustup.rs/
```

### 2. Build the project

```powershell
cd c:\Users\manis\Desktop\gtk4
cargo build --release
```

### 3. Run the application

```powershell
cargo run --release
```

## 📁 Project Structure

```
gtk4/
├── Cargo.toml              # Project dependencies
├── src/
│   └── main.rs            # Main application code
├── resources/
│   ├── style.css          # Premium CSS styling
│   └── ui.glade           # UI definition (not currently used)
└── README.md              # This file
```

## 🎨 Customization

### Changing Colors

Edit `resources/style.css` to customize the color scheme:

```css
/* Sidebar gradient */
.sidebar {
    background: linear-gradient(180deg, 
        rgba(102, 126, 234, 0.95) 0%,   /* Change these colors */
        rgba(118, 75, 162, 0.95) 100%);
}

/* Main window background */
window {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}
```

### Adding New Features

The code is organized into view functions:
- `create_home_view()` - Feed page
- `create_chat_view()` - Messaging
- `create_groups_view()` - Groups
- `create_media_view()` - Media gallery
- `create_share_view()` - Sharing options
- `create_profile_view()` - User profile

Each view is self-contained and can be modified independently.

## 🔧 Troubleshooting

### "pkg-config not found"
- Make sure MSYS2/vcpkg bin directory is in your PATH
- Restart your terminal after adding to PATH

### "gtk-4 not found"
- Verify GTK4 is installed: `pkg-config --list-all | grep gtk`
- Check PKG_CONFIG_PATH environment variable

### Build errors with file locks
- Close any running instances of the app
- Run `cargo clean` and try again
- Restart your IDE/terminal

### CSS not loading
- Make sure `resources/style.css` exists
- Check the path in `load_css()` function
- CSS errors are logged to console

## 📝 Code Overview

### Main Components

**Application Setup:**
```rust
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.linkwithmentor")
        .build();
    
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run()
}
```

**Navigation:**
- Uses `Stack` widget for page switching
- Custom sidebar with navigation buttons
- Smooth slide transitions between pages

**Styling:**
- CSS loaded via `CssProvider`
- Applied globally to all widgets
- CSS classes for component styling

## 🌟 Future Enhancements

- [ ] Backend API integration
- [ ] Real WebRTC video/audio calls
- [ ] Database for persistent storage
- [ ] User authentication
- [ ] Real-time notifications
- [ ] File upload/download
- [ ] Emoji picker
- [ ] Dark/Light theme toggle
- [ ] Search functionality
- [ ] Settings page

## 📄 License

MIT License - feel free to use and modify!

## 🤝 Contributing

This is a demonstration project. Feel free to fork and extend it!

---

**Built with ❤️ using GTK4 and Rust**
