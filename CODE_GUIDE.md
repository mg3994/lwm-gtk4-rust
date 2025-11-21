# Code Structure Guide

## Overview

The LinkWithMentor application is built with a modular architecture where each view (page) is a self-contained function that returns a GTK widget.

## File Structure

```
src/main.rs         - Main application code (~830 lines)
resources/style.css - All styling and theming
resources/ui.glade  - XML UI definition (currently unused)
```

## Main Application Flow

```rust
main()
  ↓
Application::builder()
  ↓
connect_startup() → load_css()
  ↓
connect_activate() → build_ui()
  ↓
window.present()
```

## Core Functions

### Application Setup

| Function | Purpose | Returns |
|----------|---------|---------|
| `main()` | Entry point, creates GTK Application | `glib::ExitCode` |
| `load_css()` | Loads and applies CSS styling | `()` |
| `build_ui(app)` | Creates main window and all views | `()` |

### Layout Components

| Function | Purpose | Returns |
|----------|---------|---------|
| `create_sidebar(stack)` | Navigation sidebar with buttons | `GtkBox` |
| `create_section_header(title, subtitle)` | Reusable page header | `GtkBox` |

### View Functions

Each view function creates a complete page:

| Function | View | Returns | Features |
|----------|------|---------|----------|
| `create_home_view()` | Feed page | `ScrolledWindow` | Posts, create button |
| `create_chat_view()` | Messaging | `GtkBox` | Chat list, conversation, input |
| `create_groups_view()` | Groups | `ScrolledWindow` | Group cards, join buttons |
| `create_media_view()` | Media gallery | `ScrolledWindow` | Grid of media items |
| `create_share_view()` | Sharing | `GtkBox` | Share options, recent shares |
| `create_profile_view()` | User profile | `ScrolledWindow` | Stats, bio, activity |

### Card/Item Components

Reusable components for displaying data:

| Function | Purpose | Returns |
|----------|---------|---------|
| `create_post_card(user, content, time, emoji)` | Social media post | `Frame` |
| `create_chat_item(name, message, time, unread)` | Chat list item | `GtkBox` |
| `create_message_bubble(sender, text, is_own)` | Chat message | `GtkBox` |
| `create_group_card(name, members, emoji, desc)` | Group card | `Frame` |
| `create_media_card(icon, title, time)` | Media item | `Frame` |
| `create_share_option(icon, label)` | Share option button | `Frame` |
| `create_share_item(file, shared_with, time, icon)` | Share history item | `Frame` |
| `create_activity_item(activity, time, icon)` | Profile activity | `Frame` |

## Widget Hierarchy

```
ApplicationWindow
└── GtkBox (horizontal)
    ├── Sidebar (GtkBox vertical)
    │   ├── Logo
    │   ├── Navigation Buttons
    │   └── Settings Button
    └── Stack (page container)
        ├── Home View (ScrolledWindow)
        │   └── Feed Box
        │       ├── Header
        │       ├── Create Post Button
        │       └── Post Cards
        ├── Chat View (GtkBox)
        │   ├── Chat List Sidebar
        │   └── Conversation Area
        │       ├── Chat Header
        │       ├── Messages
        │       └── Input Box
        ├── Groups View
        ├── Media View
        ├── Share View
        └── Profile View
```

## CSS Classes

### Layout Classes

| Class | Applied To | Purpose |
|-------|-----------|---------|
| `.sidebar` | Sidebar box | Purple gradient background |
| `.feed-container` | Feed wrapper | Transparent background |
| `.conversation-area` | Chat area | White background with blur |
| `.chat-sidebar` | Chat list | White background |

### Component Classes

| Class | Applied To | Purpose |
|-------|-----------|---------|
| `.nav-button` | Navigation buttons | Glassmorphism effect |
| `.post-card` | Post frames | White card with shadow |
| `.group-card` | Group frames | White card with shadow |
| `.media-card` | Media items | White card with shadow |
| `.chat-item` | Chat list items | Hover effects |
| `.message-bubble-own` | Your messages | Purple gradient |
| `.message-bubble-other` | Their messages | Gray background |

### Text Classes

| Class | Applied To | Purpose |
|-------|-----------|---------|
| `.section-title` | Page titles | Large, bold, white |
| `.section-subtitle` | Page subtitles | Medium, white, 80% opacity |
| `.post-user` | Post author | Bold, dark |
| `.post-content` | Post text | Regular, dark |
| `.post-time` | Timestamps | Small, gray |

### Button Classes

| Class | Applied To | Purpose |
|-------|-----------|---------|
| `.create-post-button` | Create buttons | Pink gradient |
| `.action-button` | Like/Comment/Share | Light purple background |
| `.join-button` | Group join | Purple gradient |
| `.send-button` | Send message | Purple gradient |
| `.call-button` | Audio/Video | Transparent white |

## Adding a New View

1. **Create the view function:**
```rust
fn create_my_view() -> ScrolledWindow {
    let scroll = ScrolledWindow::new();
    let main_box = GtkBox::new(Orientation::Vertical, 20);
    
    // Add your content here
    
    scroll.set_child(Some(&main_box));
    scroll
}
```

2. **Add to stack in `build_ui()`:**
```rust
let my_view = create_my_view();
stack.add_titled(&my_view, Some("myview"), "🎯 My View");
```

3. **Add navigation button in `create_sidebar()`:**
```rust
("myview", "🎯", "My View", "myview-page"),
```

4. **Add CSS styling in `style.css`:**
```css
.myview-specific-class {
    /* Your styles */
}
```

## Adding a New Card Component

```rust
fn create_my_card(title: &str, description: &str) -> Frame {
    let frame = Frame::new(None);
    frame.add_css_class("my-card");
    
    let box_content = GtkBox::new(Orientation::Vertical, 12);
    box_content.set_margin_start(20);
    box_content.set_margin_end(20);
    box_content.set_margin_top(16);
    box_content.set_margin_bottom(16);
    
    let title_label = Label::new(Some(title));
    title_label.add_css_class("card-title");
    
    let desc_label = Label::new(Some(description));
    desc_label.add_css_class("card-description");
    
    box_content.append(&title_label);
    box_content.append(&desc_label);
    
    frame.set_child(Some(&box_content));
    frame
}
```

## Event Handling

### Button Clicks

```rust
button.connect_clicked(|_| {
    println!("Button clicked!");
});
```

### With State (using clone)

```rust
use gtk4::glib::clone;

let label = Label::new(Some("0"));
let button = Button::with_label("Increment");

button.connect_clicked(clone!(@weak label => move |_| {
    // Access label here
    label.set_text("Clicked!");
}));
```

### Navigation

```rust
let stack_clone = stack.clone();
button.connect_clicked(move |_| {
    stack_clone.set_visible_child_name("page_name");
});
```

## Common Patterns

### Creating a Box with Margins

```rust
let box_widget = GtkBox::new(Orientation::Vertical, 12);
box_widget.set_margin_start(20);
box_widget.set_margin_end(20);
box_widget.set_margin_top(16);
box_widget.set_margin_bottom(16);
```

### Creating a Scrollable Area

```rust
let scroll = ScrolledWindow::builder()
    .hscrollbar_policy(gtk4::PolicyType::Never)
    .vexpand(true)
    .build();

let content = GtkBox::new(Orientation::Vertical, 20);
scroll.set_child(Some(&content));
```

### Adding CSS Classes

```rust
widget.add_css_class("my-class");
widget.add_css_class("another-class");
```

### Setting Alignment

```rust
label.set_halign(gtk4::Align::Start);  // Left
label.set_halign(gtk4::Align::Center); // Center
label.set_halign(gtk4::Align::End);    // Right
```

## Debugging Tips

### Print Widget Tree

```rust
println!("{:?}", widget);
```

### Check CSS Classes

```rust
let classes = widget.css_classes();
println!("CSS classes: {:?}", classes);
```

### Monitor Events

```rust
button.connect_clicked(|btn| {
    println!("Button clicked: {:?}", btn);
});
```

## Performance Tips

1. **Use `clone()` sparingly** - Only clone what you need in closures
2. **Reuse widgets** - Don't create new widgets unnecessarily
3. **Lazy loading** - Create content only when needed
4. **CSS over code** - Use CSS for styling instead of programmatic changes

## Next Steps

- Add database integration for persistent data
- Implement real networking for chat
- Add file upload/download functionality
- Create settings page
- Add user authentication
- Implement search functionality

---

**Happy coding! 🎨**
