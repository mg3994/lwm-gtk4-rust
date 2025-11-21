// LinkWithMentor - Advanced GTK4 Social Media Application
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, CssProvider, Entry, Frame, Label,
    ListBox, Orientation, ScrolledWindow, Stack,
};

const APP_ID: &str = "com.linkwithmentor";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        load_css();
    });

    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("resources/style.css");

    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("LinkWithMentor")
        .default_width(1400)
        .default_height(900)
        .build();

    // Main container
    let main_box = GtkBox::new(Orientation::Horizontal, 0);

    // Create navigation stack
    let stack = Stack::builder()
        .transition_type(gtk4::StackTransitionType::SlideLeftRight)
        .transition_duration(300)
        .build();

    // Build all views
    let home_view = create_home_view();
    let chat_view = create_chat_view();
    let groups_view = create_groups_view();
    let media_view = create_media_view();
    let share_view = create_share_view();
    let profile_view = create_profile_view();
    let notifications_view = create_notifications_view();
    let search_view = create_search_view();
    let settings_view = create_settings_view();
    let video_call_view = create_video_call_view();
    let create_post_view = create_create_post_view();
    let analytics_view = create_home_view();
    let events_view = create_groups_view();
    let jobs_view = create_media_view();
    let achievements_view = create_profile_view();
    let learning_view = create_search_view();
    let discover_view = create_home_view();

    stack.add_titled(&home_view, Some("home"), "🏠 Home");
    stack.add_titled(&chat_view, Some("chat"), "💬 Chat");
    stack.add_titled(&groups_view, Some("groups"), "👥 Groups");
    stack.add_titled(&media_view, Some("media"), "📸 Media");
    stack.add_titled(&share_view, Some("share"), "📤 Share");
    stack.add_titled(&profile_view, Some("profile"), "👤 Profile");
    stack.add_titled(
        &notifications_view,
        Some("notifications"),
        "🔔 Notifications",
    );
    stack.add_titled(&search_view, Some("search"), "🔍 Search");
    stack.add_titled(&settings_view, Some("settings"), "⚙️ Settings");
    stack.add_titled(&video_call_view, Some("videocall"), "📹 Video Call");
    stack.add_titled(&create_post_view, Some("createpost"), "✨ Create Post");
    stack.add_titled(&analytics_view, Some("analytics"), "📊 Analytics");
    stack.add_titled(&events_view, Some("events"), "🎮 Events");
    stack.add_titled(&jobs_view, Some("jobs"), "💼 Jobs");
    stack.add_titled(&achievements_view, Some("achievements"), "🏆 Achievements");
    stack.add_titled(&learning_view, Some("learning"), "📚 Learning");
    stack.add_titled(&discover_view, Some("discover"), "🌐 Discover");

    // Create custom sidebar
    let sidebar = create_sidebar(&stack);

    main_box.append(&sidebar);
    main_box.append(&stack);

    window.set_child(Some(&main_box));
    window.present();
}

fn create_sidebar(stack: &Stack) -> GtkBox {
    let sidebar_box = GtkBox::new(Orientation::Vertical, 16);
    sidebar_box.set_width_request(280);
    sidebar_box.set_margin_start(12);
    sidebar_box.set_margin_end(12);
    sidebar_box.set_margin_top(24);
    sidebar_box.set_margin_bottom(24);
    sidebar_box.add_css_class("sidebar");

    // Logo section
    let logo_box = GtkBox::new(Orientation::Vertical, 8);
    let logo = Label::new(Some("🎓"));
    logo.add_css_class("logo-icon");
    let app_name = Label::new(Some("LinkWithMentor"));
    app_name.add_css_class("app-title");
    logo_box.append(&logo);
    logo_box.append(&app_name);
    sidebar_box.append(&logo_box);

    // Navigation buttons
    let nav_items = vec![
        ("home", "🏠", "Home", "home-page"),
        ("discover", "🌐", "Discover", "discover-page"),
        ("search", "🔍", "Search", "search-page"),
        ("notifications", "🔔", "Notifications", "notifications-page"),
        ("chat", "💬", "Chat", "chat-page"),
        ("groups", "👥", "Groups", "groups-page"),
        ("events", "🎮", "Events", "events-page"),
        ("learning", "📚", "Learning", "learning-page"),
        ("jobs", "💼", "Jobs", "jobs-page"),
        ("achievements", "🏆", "Achievements", "achievements-page"),
        ("analytics", "📊", "Analytics", "analytics-page"),
        ("media", "📸", "Media", "media-page"),
        ("share", "📤", "Share", "share-page"),
        ("profile", "👤", "Profile", "profile-page"),
    ];

    for (page_name, icon, label_text, css_class) in nav_items {
        let button = Button::new();
        let button_box = GtkBox::new(Orientation::Horizontal, 12);

        let icon_label = Label::new(Some(icon));
        icon_label.add_css_class("nav-icon");
        let text_label = Label::new(Some(label_text));
        text_label.set_halign(gtk4::Align::Start);
        text_label.add_css_class("nav-text");

        button_box.append(&icon_label);
        button_box.append(&text_label);
        button.set_child(Some(&button_box));
        button.add_css_class("nav-button");
        button.add_css_class(css_class);

        let stack_clone = stack.clone();
        let page = page_name.to_string();
        button.connect_clicked(move |_| {
            stack_clone.set_visible_child_name(&page);
        });

        sidebar_box.append(&button);
    }

    // Spacer
    let spacer = GtkBox::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    sidebar_box.append(&spacer);

    // Settings button at bottom
    let settings_btn = Button::new();
    let settings_box = GtkBox::new(Orientation::Horizontal, 12);
    let settings_icon = Label::new(Some("⚙️"));
    settings_icon.add_css_class("nav-icon");
    let settings_text = Label::new(Some("Settings"));
    settings_text.add_css_class("nav-text");
    settings_box.append(&settings_icon);
    settings_box.append(&settings_text);

    let stack_clone = stack.clone();
    settings_btn.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("settings");
    });
    settings_btn.set_child(Some(&settings_box));
    settings_btn.add_css_class("nav-button");
    sidebar_box.append(&settings_btn);

    sidebar_box
}

fn create_home_view() -> ScrolledWindow {
    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vexpand(true)
        .build();

    let feed_box = GtkBox::new(Orientation::Vertical, 20);
    feed_box.set_margin_start(40);
    feed_box.set_margin_end(40);
    feed_box.set_margin_top(30);
    feed_box.set_margin_bottom(30);
    feed_box.add_css_class("feed-container");

    // Header
    let header = create_section_header("📰 Your Feed", "Stay updated with your mentors");
    feed_box.append(&header);

    // Create post button
    let create_post_btn = Button::with_label("✨ Create New Post");
    create_post_btn.add_css_class("create-post-button");
    feed_box.append(&create_post_btn);

    // Sample posts
    let posts = vec![
        (
            "Alice Johnson",
            "Just completed an amazing mentoring session on Rust! 🦀",
            "2 hours ago",
            "🎯",
        ),
        (
            "Bob Smith",
            "Looking for mentors in Machine Learning. Any recommendations?",
            "5 hours ago",
            "🤖",
        ),
        (
            "Carol Williams",
            "Sharing my latest project: A GTK4 social media app! Check it out 🚀",
            "1 day ago",
            "💻",
        ),
        (
            "David Brown",
            "Thanks to my mentor, I finally understood async/await! 🎉",
            "2 days ago",
            "⚡",
        ),
    ];

    for (user, content, time, emoji) in posts {
        let post = create_post_card(user, content, time, emoji);
        feed_box.append(&post);
    }

    scroll.set_child(Some(&feed_box));
    scroll
}

fn create_post_card(user: &str, content: &str, time: &str, emoji: &str) -> Frame {
    let frame = Frame::new(None);
    frame.add_css_class("post-card");

    let card_box = GtkBox::new(Orientation::Vertical, 12);
    card_box.set_margin_start(20);
    card_box.set_margin_end(20);
    card_box.set_margin_top(16);
    card_box.set_margin_bottom(16);

    // User header
    let user_box = GtkBox::new(Orientation::Horizontal, 12);
    let avatar = Label::new(Some(emoji));
    avatar.add_css_class("avatar");
    let user_info = GtkBox::new(Orientation::Vertical, 4);
    let user_label = Label::new(Some(user));
    user_label.set_halign(gtk4::Align::Start);
    user_label.add_css_class("post-user");
    let time_label = Label::new(Some(time));
    time_label.set_halign(gtk4::Align::Start);
    time_label.add_css_class("post-time");
    user_info.append(&user_label);
    user_info.append(&time_label);
    user_box.append(&avatar);
    user_box.append(&user_info);

    // Content
    let content_label = Label::new(Some(content));
    content_label.set_wrap(true);
    content_label.set_halign(gtk4::Align::Start);
    content_label.add_css_class("post-content");

    // Actions
    let actions_box = GtkBox::new(Orientation::Horizontal, 12);
    let like_btn = Button::with_label("👍 Like");
    like_btn.add_css_class("action-button");
    let comment_btn = Button::with_label("💬 Comment");
    comment_btn.add_css_class("action-button");
    let share_btn = Button::with_label("🔗 Share");
    share_btn.add_css_class("action-button");

    actions_box.append(&like_btn);
    actions_box.append(&comment_btn);
    actions_box.append(&share_btn);

    card_box.append(&user_box);
    card_box.append(&content_label);
    card_box.append(&actions_box);

    frame.set_child(Some(&card_box));
    frame
}

fn create_chat_view() -> GtkBox {
    let main_box = GtkBox::new(Orientation::Horizontal, 0);

    // Chat list sidebar
    let chat_list_box = GtkBox::new(Orientation::Vertical, 8);
    chat_list_box.set_width_request(320);
    chat_list_box.add_css_class("chat-sidebar");
    chat_list_box.set_margin_start(12);
    chat_list_box.set_margin_end(12);
    chat_list_box.set_margin_top(12);
    chat_list_box.set_margin_bottom(12);

    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some("🔍 Search conversations..."));
    search_entry.add_css_class("search-entry");
    chat_list_box.append(&search_entry);

    let chat_scroll = ScrolledWindow::new();
    let chat_list = ListBox::new();
    chat_list.add_css_class("chat-list");

    // Sample chats
    let chats = vec![
        (
            "Alice Johnson",
            "Great! Let's schedule for tomorrow",
            "2m",
            true,
        ),
        ("Bob Smith", "Thanks for the resources!", "1h", false),
        ("Carol Williams", "Can you review my code?", "3h", true),
        ("David Brown", "See you in the meeting!", "1d", false),
    ];

    for (name, message, time, unread) in chats {
        let chat_item = create_chat_item(name, message, time, unread);
        chat_list.append(&chat_item);
    }

    chat_scroll.set_child(Some(&chat_list));
    chat_scroll.set_vexpand(true);
    chat_list_box.append(&chat_scroll);

    // Chat conversation area
    let conversation_box = GtkBox::new(Orientation::Vertical, 0);
    conversation_box.set_hexpand(true);
    conversation_box.add_css_class("conversation-area");

    // Chat header
    let chat_header = GtkBox::new(Orientation::Horizontal, 12);
    chat_header.add_css_class("chat-header");
    chat_header.set_margin_start(20);
    chat_header.set_margin_end(20);
    chat_header.set_margin_top(12);
    chat_header.set_margin_bottom(12);

    let header_avatar = Label::new(Some("👤"));
    header_avatar.add_css_class("chat-avatar");
    let header_name = Label::new(Some("Alice Johnson"));
    header_name.add_css_class("chat-header-name");
    chat_header.append(&header_avatar);
    chat_header.append(&header_name);

    // Add call buttons
    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    chat_header.append(&spacer);

    let audio_btn = Button::with_label("🎤 Audio");
    audio_btn.add_css_class("call-button");
    let video_btn = Button::with_label("📹 Video");
    video_btn.add_css_class("call-button");
    chat_header.append(&audio_btn);
    chat_header.append(&video_btn);

    // Messages area
    let messages_scroll = ScrolledWindow::new();
    messages_scroll.set_vexpand(true);
    let messages_box = GtkBox::new(Orientation::Vertical, 12);
    messages_box.set_margin_start(20);
    messages_box.set_margin_end(20);
    messages_box.set_margin_top(20);
    messages_box.add_css_class("messages-container");

    // Sample messages
    let messages = vec![
        ("Alice", "Hey! How's your project going?", false),
        ("You", "It's going great! Just implemented the UI", true),
        ("Alice", "Awesome! Can I see a demo?", false),
        ("You", "Sure! Let me share my screen", true),
    ];

    for (sender, text, is_own) in messages {
        let msg = create_message_bubble(sender, text, is_own);
        messages_box.append(&msg);
    }

    messages_scroll.set_child(Some(&messages_box));

    // Input area
    let input_box = GtkBox::new(Orientation::Horizontal, 12);
    input_box.add_css_class("chat-input-area");
    input_box.set_margin_start(20);
    input_box.set_margin_end(20);
    input_box.set_margin_top(12);
    input_box.set_margin_bottom(12);

    let attach_btn = Button::with_label("📎");
    attach_btn.add_css_class("attach-button");
    let message_entry = Entry::new();
    message_entry.set_placeholder_text(Some("Type a message..."));
    message_entry.set_hexpand(true);
    message_entry.add_css_class("message-entry");
    let send_btn = Button::with_label("Send 🚀");
    send_btn.add_css_class("send-button");

    input_box.append(&attach_btn);
    input_box.append(&message_entry);
    input_box.append(&send_btn);

    conversation_box.append(&chat_header);
    conversation_box.append(&messages_scroll);
    conversation_box.append(&input_box);

    main_box.append(&chat_list_box);
    main_box.append(&conversation_box);
    main_box
}

fn create_chat_item(name: &str, message: &str, time: &str, unread: bool) -> GtkBox {
    let item_box = GtkBox::new(Orientation::Horizontal, 12);
    item_box.add_css_class("chat-item");
    if unread {
        item_box.add_css_class("unread");
    }
    item_box.set_margin_start(8);
    item_box.set_margin_end(8);
    item_box.set_margin_top(6);
    item_box.set_margin_bottom(6);

    let avatar = Label::new(Some("👤"));
    avatar.add_css_class("chat-item-avatar");

    let content_box = GtkBox::new(Orientation::Vertical, 4);
    content_box.set_hexpand(true);

    let name_label = Label::new(Some(name));
    name_label.set_halign(gtk4::Align::Start);
    name_label.add_css_class("chat-item-name");

    let message_label = Label::new(Some(message));
    message_label.set_halign(gtk4::Align::Start);
    message_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    message_label.add_css_class("chat-item-message");

    content_box.append(&name_label);
    content_box.append(&message_label);

    let time_label = Label::new(Some(time));
    time_label.add_css_class("chat-item-time");

    item_box.append(&avatar);
    item_box.append(&content_box);
    item_box.append(&time_label);

    item_box
}

fn create_message_bubble(sender: &str, text: &str, is_own: bool) -> GtkBox {
    let container = GtkBox::new(Orientation::Horizontal, 0);

    let bubble = GtkBox::new(Orientation::Vertical, 4);
    bubble.set_margin_start(12);
    bubble.set_margin_end(12);
    bubble.set_margin_top(8);
    bubble.set_margin_bottom(8);

    if is_own {
        bubble.add_css_class("message-bubble-own");
        container.set_halign(gtk4::Align::End);
    } else {
        bubble.add_css_class("message-bubble-other");
        container.set_halign(gtk4::Align::Start);
    }

    let sender_label = Label::new(Some(sender));
    sender_label.set_halign(gtk4::Align::Start);
    sender_label.add_css_class("message-sender");

    let text_label = Label::new(Some(text));
    text_label.set_wrap(true);
    text_label.set_halign(gtk4::Align::Start);
    text_label.add_css_class("message-text");

    bubble.append(&sender_label);
    bubble.append(&text_label);
    container.append(&bubble);

    container
}

fn create_groups_view() -> ScrolledWindow {
    let scroll = ScrolledWindow::new();
    let groups_box = GtkBox::new(Orientation::Vertical, 20);
    groups_box.set_margin_start(40);
    groups_box.set_margin_end(40);
    groups_box.set_margin_top(30);
    groups_box.set_margin_bottom(30);

    let header = create_section_header("👥 Groups", "Connect with communities");
    groups_box.append(&header);

    let create_group_btn = Button::with_label("➕ Create New Group");
    create_group_btn.add_css_class("create-post-button");
    groups_box.append(&create_group_btn);

    // Sample groups
    let groups = vec![
        (
            "Rust Developers",
            "2.5k members",
            "🦀",
            "Discuss Rust programming",
        ),
        (
            "UI/UX Design",
            "1.8k members",
            "🎨",
            "Share design resources",
        ),
        (
            "Career Mentorship",
            "3.2k members",
            "🎯",
            "Get career guidance",
        ),
        (
            "Open Source",
            "4.1k members",
            "💻",
            "Collaborate on projects",
        ),
    ];

    for (name, members, emoji, description) in groups {
        let group_card = create_group_card(name, members, emoji, description);
        groups_box.append(&group_card);
    }

    scroll.set_child(Some(&groups_box));
    scroll
}

fn create_group_card(name: &str, members: &str, emoji: &str, description: &str) -> Frame {
    let frame = Frame::new(None);
    frame.add_css_class("group-card");

    let card_box = GtkBox::new(Orientation::Horizontal, 16);
    card_box.set_margin_start(20);
    card_box.set_margin_end(20);
    card_box.set_margin_top(16);
    card_box.set_margin_bottom(16);

    let icon = Label::new(Some(emoji));
    icon.add_css_class("group-icon");

    let info_box = GtkBox::new(Orientation::Vertical, 6);
    info_box.set_hexpand(true);

    let name_label = Label::new(Some(name));
    name_label.set_halign(gtk4::Align::Start);
    name_label.add_css_class("group-name");

    let desc_label = Label::new(Some(description));
    desc_label.set_halign(gtk4::Align::Start);
    desc_label.add_css_class("group-description");

    let members_label = Label::new(Some(members));
    members_label.set_halign(gtk4::Align::Start);
    members_label.add_css_class("group-members");

    info_box.append(&name_label);
    info_box.append(&desc_label);
    info_box.append(&members_label);

    let join_btn = Button::with_label("Join");
    join_btn.add_css_class("join-button");
    join_btn.set_valign(gtk4::Align::Center);

    card_box.append(&icon);
    card_box.append(&info_box);
    card_box.append(&join_btn);

    frame.set_child(Some(&card_box));
    frame
}

fn create_media_view() -> ScrolledWindow {
    let scroll = ScrolledWindow::new();
    let media_box = GtkBox::new(Orientation::Vertical, 20);
    media_box.set_margin_start(40);
    media_box.set_margin_end(40);
    media_box.set_margin_top(30);
    media_box.set_margin_bottom(30);

    let header = create_section_header("📸 Media Gallery", "Your shared moments");
    media_box.append(&header);

    let upload_btn = Button::with_label("📤 Upload Media");
    upload_btn.add_css_class("create-post-button");
    media_box.append(&upload_btn);

    // Grid for media items
    let grid = gtk4::Grid::new();
    grid.set_row_spacing(16);
    grid.set_column_spacing(16);
    grid.add_css_class("media-grid");

    let media_items = vec![
        ("🖼️", "Project Screenshot", "2 days ago"),
        ("📹", "Tutorial Video", "5 days ago"),
        ("📄", "Resume.pdf", "1 week ago"),
        ("🎵", "Podcast Episode", "2 weeks ago"),
        ("🖼️", "Team Photo", "3 weeks ago"),
        ("📹", "Demo Recording", "1 month ago"),
    ];

    for (i, (icon, title, time)) in media_items.iter().enumerate() {
        let media_card = create_media_card(icon, title, time);
        grid.attach(&media_card, (i % 3) as i32, (i / 3) as i32, 1, 1);
    }

    media_box.append(&grid);
    scroll.set_child(Some(&media_box));
    scroll
}

fn create_media_card(icon: &str, title: &str, time: &str) -> Frame {
    let frame = Frame::new(None);
    frame.add_css_class("media-card");
    frame.set_width_request(250);
    frame.set_height_request(200);

    let card_box = GtkBox::new(Orientation::Vertical, 12);
    card_box.set_margin_start(16);
    card_box.set_margin_end(16);
    card_box.set_margin_top(16);
    card_box.set_margin_bottom(16);
    card_box.set_valign(gtk4::Align::Center);
    card_box.set_halign(gtk4::Align::Center);

    let icon_label = Label::new(Some(icon));
    icon_label.add_css_class("media-icon");

    let title_label = Label::new(Some(title));
    title_label.add_css_class("media-title");

    let time_label = Label::new(Some(time));
    time_label.add_css_class("media-time");

    card_box.append(&icon_label);
    card_box.append(&title_label);
    card_box.append(&time_label);

    frame.set_child(Some(&card_box));
    frame
}

fn create_share_view() -> GtkBox {
    let main_box = GtkBox::new(Orientation::Vertical, 20);
    main_box.set_margin_start(40);
    main_box.set_margin_end(40);
    main_box.set_margin_top(30);
    main_box.set_margin_bottom(30);

    let header = create_section_header("📤 Share Content", "Share files, links, and more");
    main_box.append(&header);

    // Share options grid
    let options_box = GtkBox::new(Orientation::Horizontal, 20);
    options_box.set_halign(gtk4::Align::Center);

    let share_options = vec![
        ("📄", "Documents"),
        ("🖼️", "Images"),
        ("📹", "Videos"),
        ("🔗", "Links"),
        ("📍", "Location"),
        ("📊", "Polls"),
    ];

    for (icon, label) in share_options {
        let option_card = create_share_option(icon, label);
        options_box.append(&option_card);
    }

    main_box.append(&options_box);

    // Recent shares
    let recent_label = Label::new(Some("Recent Shares"));
    recent_label.add_css_class("section-subtitle");
    recent_label.set_halign(gtk4::Align::Start);
    recent_label.set_margin_top(20);
    main_box.append(&recent_label);

    let shares = vec![
        (
            "Project Proposal.pdf",
            "Shared with Alice",
            "1 hour ago",
            "📄",
        ),
        (
            "Screenshot.png",
            "Shared in Rust Group",
            "3 hours ago",
            "🖼️",
        ),
        ("Tutorial.mp4", "Shared with Bob", "Yesterday", "📹"),
    ];

    for (file, shared_with, time, icon) in shares {
        let share_item = create_share_item(file, shared_with, time, icon);
        main_box.append(&share_item);
    }

    main_box
}

fn create_share_option(icon: &str, label: &str) -> Frame {
    let frame = Frame::new(None);
    frame.add_css_class("share-option");
    frame.set_width_request(120);
    frame.set_height_request(120);

    let box_content = GtkBox::new(Orientation::Vertical, 8);
    box_content.set_valign(gtk4::Align::Center);
    box_content.set_halign(gtk4::Align::Center);

    let icon_label = Label::new(Some(icon));
    icon_label.add_css_class("share-icon");

    let text_label = Label::new(Some(label));
    text_label.add_css_class("share-label");

    box_content.append(&icon_label);
    box_content.append(&text_label);

    frame.set_child(Some(&box_content));
    frame
}

fn create_share_item(file: &str, shared_with: &str, time: &str, icon: &str) -> Frame {
    let frame = Frame::new(None);
    frame.add_css_class("share-item");

    let item_box = GtkBox::new(Orientation::Horizontal, 16);
    item_box.set_margin_start(16);
    item_box.set_margin_end(16);
    item_box.set_margin_top(12);
    item_box.set_margin_bottom(12);

    let icon_label = Label::new(Some(icon));
    icon_label.add_css_class("share-item-icon");

    let info_box = GtkBox::new(Orientation::Vertical, 4);
    info_box.set_hexpand(true);

    let file_label = Label::new(Some(file));
    file_label.set_halign(gtk4::Align::Start);
    file_label.add_css_class("share-item-file");

    let shared_label = Label::new(Some(shared_with));
    shared_label.set_halign(gtk4::Align::Start);
    shared_label.add_css_class("share-item-shared");

    info_box.append(&file_label);
    info_box.append(&shared_label);

    let time_label = Label::new(Some(time));
    time_label.add_css_class("share-item-time");

    item_box.append(&icon_label);
    item_box.append(&info_box);
    item_box.append(&time_label);

    frame.set_child(Some(&item_box));
    frame
}

fn create_profile_view() -> ScrolledWindow {
    let scroll = ScrolledWindow::new();
    let profile_box = GtkBox::new(Orientation::Vertical, 24);
    profile_box.set_margin_start(40);
    profile_box.set_margin_end(40);
    profile_box.set_margin_top(30);
    profile_box.set_margin_bottom(30);

    // Profile header
    let header_box = GtkBox::new(Orientation::Vertical, 16);
    header_box.add_css_class("profile-header");
    header_box.set_margin_start(20);
    header_box.set_margin_end(20);
    header_box.set_margin_top(20);
    header_box.set_margin_bottom(20);

    let avatar = Label::new(Some("👤"));
    avatar.add_css_class("profile-avatar");

    let name = Label::new(Some("Your Name"));
    name.add_css_class("profile-name");

    let bio = Label::new(Some("Software Developer | Mentor | Open Source Enthusiast"));
    bio.add_css_class("profile-bio");
    bio.set_wrap(true);

    let stats_box = GtkBox::new(Orientation::Horizontal, 40);
    stats_box.set_halign(gtk4::Align::Center);
    stats_box.set_margin_top(16);

    let stats = vec![("Posts", "42"), ("Followers", "1.2k"), ("Following", "856")];

    for (label, value) in stats {
        let stat_box = GtkBox::new(Orientation::Vertical, 4);
        let value_label = Label::new(Some(value));
        value_label.add_css_class("stat-value");
        let label_label = Label::new(Some(label));
        label_label.add_css_class("stat-label");
        stat_box.append(&value_label);
        stat_box.append(&label_label);
        stats_box.append(&stat_box);
    }

    let edit_btn = Button::with_label("✏️ Edit Profile");
    edit_btn.add_css_class("edit-profile-button");
    edit_btn.set_halign(gtk4::Align::Center);
    edit_btn.set_margin_top(16);

    header_box.append(&avatar);
    header_box.append(&name);
    header_box.append(&bio);
    header_box.append(&stats_box);
    header_box.append(&edit_btn);

    profile_box.append(&header_box);

    // Activity section
    let activity_label = Label::new(Some("Recent Activity"));
    activity_label.add_css_class("section-subtitle");
    activity_label.set_halign(gtk4::Align::Start);
    activity_label.set_margin_top(20);
    profile_box.append(&activity_label);

    let activities = vec![
        ("Posted in Rust Developers", "2 hours ago", "📝"),
        ("Joined UI/UX Design group", "1 day ago", "👥"),
        ("Shared a video", "3 days ago", "📹"),
        ("Commented on Alice's post", "5 days ago", "💬"),
    ];

    for (activity, time, icon) in activities {
        let activity_item = create_activity_item(activity, time, icon);
        profile_box.append(&activity_item);
    }

    scroll.set_child(Some(&profile_box));
    scroll
}

fn create_activity_item(activity: &str, time: &str, icon: &str) -> Frame {
    let frame = Frame::new(None);
    frame.add_css_class("activity-item");

    let item_box = GtkBox::new(Orientation::Horizontal, 16);
    item_box.set_margin_start(16);
    item_box.set_margin_end(16);
    item_box.set_margin_top(12);
    item_box.set_margin_bottom(12);

    let icon_label = Label::new(Some(icon));
    icon_label.add_css_class("activity-icon");

    let activity_label = Label::new(Some(activity));
    activity_label.set_halign(gtk4::Align::Start);
    activity_label.set_hexpand(true);
    activity_label.add_css_class("activity-text");

    let time_label = Label::new(Some(time));
    time_label.add_css_class("activity-time");

    item_box.append(&icon_label);
    item_box.append(&activity_label);
    item_box.append(&time_label);

    frame.set_child(Some(&item_box));
    frame
}

fn create_section_header(title: &str, subtitle: &str) -> GtkBox {
    let header_box = GtkBox::new(Orientation::Vertical, 8);

    let title_label = Label::new(Some(title));
    title_label.set_halign(gtk4::Align::Start);
    title_label.add_css_class("section-title");

    let subtitle_label = Label::new(Some(subtitle));
    subtitle_label.set_halign(gtk4::Align::Start);
    subtitle_label.add_css_class("section-subtitle");

    header_box.append(&title_label);
    header_box.append(&subtitle_label);

    header_box
}

// ============================================================================
// NEW ADVANCED VIEWS
// ============================================================================

fn create_notifications_view() -> ScrolledWindow {
    let scroll = ScrolledWindow::new();
    let notif_box = GtkBox::new(Orientation::Vertical, 20);
    notif_box.set_margin_start(40);
    notif_box.set_margin_end(40);
    notif_box.set_margin_top(30);
    notif_box.set_margin_bottom(30);

    let header = create_section_header("🔔 Notifications", "Stay updated with your activity");
    notif_box.append(&header);

    // Filter buttons
    let filter_box = GtkBox::new(Orientation::Horizontal, 12);
    let filters = vec!["All", "Mentions", "Likes", "Comments", "Follows"];

    for filter in filters {
        let filter_btn = Button::with_label(filter);
        filter_btn.add_css_class("filter-button");
        if filter == "All" {
            filter_btn.add_css_class("filter-active");
        }
        filter_box.append(&filter_btn);
    }
    notif_box.append(&filter_box);

    // Notifications
    let notifications = vec![
        ("👍", "Alice Johnson", "liked your post", "2 min ago", true),
        (
            "💬",
            "Bob Smith",
            "commented: 'Great work!'",
            "15 min ago",
            true,
        ),
        (
            "👥",
            "Carol Williams",
            "joined your group 'Rust Developers'",
            "1 hour ago",
            false,
        ),
        (
            "🔗",
            "David Brown",
            "shared your post",
            "2 hours ago",
            false,
        ),
        (
            "👤",
            "Emma Davis",
            "started following you",
            "5 hours ago",
            false,
        ),
        (
            "💬",
            "Frank Miller",
            "mentioned you in a comment",
            "1 day ago",
            false,
        ),
        ("👍", "Grace Lee", "liked your comment", "2 days ago", false),
    ];

    for (icon, user, action, time, unread) in notifications {
        let notif_item = create_notification_item(icon, user, action, time, unread);
        notif_box.append(&notif_item);
    }

    scroll.set_child(Some(&notif_box));
    scroll
}

fn create_notification_item(
    icon: &str,
    user: &str,
    action: &str,
    time: &str,
    unread: bool,
) -> Frame {
    let frame = Frame::new(None);
    frame.add_css_class("notification-item");
    if unread {
        frame.add_css_class("notification-unread");
    }

    let item_box = GtkBox::new(Orientation::Horizontal, 16);
    item_box.set_margin_start(16);
    item_box.set_margin_end(16);
    item_box.set_margin_top(12);
    item_box.set_margin_bottom(12);

    let icon_label = Label::new(Some(icon));
    icon_label.add_css_class("notification-icon");

    let content_box = GtkBox::new(Orientation::Vertical, 4);
    content_box.set_hexpand(true);

    let user_label = Label::new(Some(user));
    user_label.set_halign(gtk4::Align::Start);
    user_label.add_css_class("notification-user");

    let action_label = Label::new(Some(action));
    action_label.set_halign(gtk4::Align::Start);
    action_label.add_css_class("notification-action");

    content_box.append(&user_label);
    content_box.append(&action_label);

    let time_label = Label::new(Some(time));
    time_label.add_css_class("notification-time");

    item_box.append(&icon_label);
    item_box.append(&content_box);
    item_box.append(&time_label);

    frame.set_child(Some(&item_box));
    frame
}

fn create_search_view() -> GtkBox {
    let main_box = GtkBox::new(Orientation::Vertical, 20);
    main_box.set_margin_start(40);
    main_box.set_margin_end(40);
    main_box.set_margin_top(30);
    main_box.set_margin_bottom(30);

    let header = create_section_header("🔍 Search", "Find people, groups, and content");
    main_box.append(&header);

    // Search bar
    let search_box = GtkBox::new(Orientation::Horizontal, 12);
    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some("Search for anything..."));
    search_entry.set_hexpand(true);
    search_entry.add_css_class("search-entry-large");

    let search_btn = Button::with_label("🔍 Search");
    search_btn.add_css_class("search-button");

    search_box.append(&search_entry);
    search_box.append(&search_btn);
    main_box.append(&search_box);

    // Filter tabs
    let tabs_box = GtkBox::new(Orientation::Horizontal, 12);
    let tabs = vec!["All", "People", "Groups", "Posts", "Media"];

    for tab in tabs {
        let tab_btn = Button::with_label(tab);
        tab_btn.add_css_class("tab-button");
        if tab == "All" {
            tab_btn.add_css_class("tab-active");
        }
        tabs_box.append(&tab_btn);
    }
    main_box.append(&tabs_box);

    // Search results
    let results_label = Label::new(Some("Recent Searches"));
    results_label.add_css_class("section-subtitle");
    results_label.set_halign(gtk4::Align::Start);
    results_label.set_margin_top(20);
    main_box.append(&results_label);

    let recent_searches = vec![
        ("👤", "Alice Johnson", "User"),
        ("👥", "Rust Developers", "Group"),
        ("📝", "GTK4 tutorial", "Post"),
        ("👤", "Bob Smith", "User"),
    ];

    for (icon, title, category) in recent_searches {
        let search_result = create_search_result(icon, title, category);
        main_box.append(&search_result);
    }

    // Trending section
    let trending_label = Label::new(Some("🔥 Trending Topics"));
    trending_label.add_css_class("section-subtitle");
    trending_label.set_halign(gtk4::Align::Start);
    trending_label.set_margin_top(20);
    main_box.append(&trending_label);

    let trending_box = GtkBox::new(Orientation::Horizontal, 12);
    let trending_topics = vec!["#Rust", "#GTK4", "#OpenSource", "#Mentorship", "#WebDev"];

    for topic in trending_topics {
        let topic_btn = Button::with_label(topic);
        topic_btn.add_css_class("trending-tag");
        trending_box.append(&topic_btn);
    }
    main_box.append(&trending_box);

    main_box
}

fn create_search_result(icon: &str, title: &str, category: &str) -> Frame {
    let frame = Frame::new(None);
    frame.add_css_class("search-result");

    let result_box = GtkBox::new(Orientation::Horizontal, 16);
    result_box.set_margin_start(16);
    result_box.set_margin_end(16);
    result_box.set_margin_top(12);
    result_box.set_margin_bottom(12);

    let icon_label = Label::new(Some(icon));
    icon_label.add_css_class("search-result-icon");

    let content_box = GtkBox::new(Orientation::Vertical, 4);
    content_box.set_hexpand(true);

    let title_label = Label::new(Some(title));
    title_label.set_halign(gtk4::Align::Start);
    title_label.add_css_class("search-result-title");

    let category_label = Label::new(Some(category));
    category_label.set_halign(gtk4::Align::Start);
    category_label.add_css_class("search-result-category");

    content_box.append(&title_label);
    content_box.append(&category_label);

    let view_btn = Button::with_label("View");
    view_btn.add_css_class("view-button");

    result_box.append(&icon_label);
    result_box.append(&content_box);
    result_box.append(&view_btn);

    frame.set_child(Some(&result_box));
    frame
}

fn create_settings_view() -> ScrolledWindow {
    let scroll = ScrolledWindow::new();
    let settings_box = GtkBox::new(Orientation::Vertical, 24);
    settings_box.set_margin_start(40);
    settings_box.set_margin_end(40);
    settings_box.set_margin_top(30);
    settings_box.set_margin_bottom(30);

    let header = create_section_header("⚙️ Settings", "Customize your experience");
    settings_box.append(&header);

    // Account section
    let account_section = create_settings_section(
        "👤 Account",
        vec![
            ("Email", "user@example.com", true),
            ("Password", "••••••••", true),
            ("Phone", "+1 234 567 8900", true),
        ],
    );
    settings_box.append(&account_section);

    // Preferences section
    let preferences_section = create_settings_section(
        "🎨 Preferences",
        vec![
            ("Theme", "Auto (System)", false),
            ("Language", "English", false),
            ("Font Size", "Medium", false),
        ],
    );
    settings_box.append(&preferences_section);

    // Notifications section
    let notif_section = create_settings_section(
        "🔔 Notifications",
        vec![
            ("Push Notifications", "Enabled", false),
            ("Email Notifications", "Enabled", false),
            ("Sound", "Enabled", false),
        ],
    );
    settings_box.append(&notif_section);

    // Privacy section
    let privacy_section = create_settings_section(
        "🔒 Privacy & Security",
        vec![
            ("Profile Visibility", "Public", false),
            ("Two-Factor Auth", "Disabled", true),
            ("Data Sharing", "Limited", false),
        ],
    );
    settings_box.append(&privacy_section);

    // About section
    let about_box = GtkBox::new(Orientation::Vertical, 12);
    about_box.add_css_class("settings-section");
    about_box.set_margin_top(20);

    let about_title = Label::new(Some("ℹ️ About"));
    about_title.add_css_class("settings-section-title");
    about_title.set_halign(gtk4::Align::Start);

    let version_label = Label::new(Some("Version 1.0.0"));
    version_label.set_halign(gtk4::Align::Start);
    version_label.add_css_class("settings-value");

    let copyright_label = Label::new(Some("© 2025 LinkWithMentor. Built with GTK4 & Rust."));
    copyright_label.set_halign(gtk4::Align::Start);
    copyright_label.add_css_class("settings-value");

    about_box.append(&about_title);
    about_box.append(&version_label);
    about_box.append(&copyright_label);
    settings_box.append(&about_box);

    scroll.set_child(Some(&settings_box));
    scroll
}

fn create_settings_section(title: &str, items: Vec<(&str, &str, bool)>) -> GtkBox {
    let section_box = GtkBox::new(Orientation::Vertical, 12);
    section_box.add_css_class("settings-section");

    let title_label = Label::new(Some(title));
    title_label.add_css_class("settings-section-title");
    title_label.set_halign(gtk4::Align::Start);
    section_box.append(&title_label);

    for (label, value, has_edit) in items {
        let item_box = GtkBox::new(Orientation::Horizontal, 12);
        item_box.add_css_class("settings-item");

        let label_label = Label::new(Some(label));
        label_label.set_halign(gtk4::Align::Start);
        label_label.set_hexpand(true);
        label_label.add_css_class("settings-label");

        let value_label = Label::new(Some(value));
        value_label.add_css_class("settings-value");

        item_box.append(&label_label);
        item_box.append(&value_label);

        if has_edit {
            let edit_btn = Button::with_label("Edit");
            edit_btn.add_css_class("edit-button");
            item_box.append(&edit_btn);
        }

        section_box.append(&item_box);
    }

    section_box
}

fn create_video_call_view() -> GtkBox {
    let main_box = GtkBox::new(Orientation::Vertical, 0);
    main_box.add_css_class("video-call-container");

    // Video area
    let video_box = GtkBox::new(Orientation::Vertical, 0);
    video_box.set_vexpand(true);
    video_box.add_css_class("video-area");

    // Remote video placeholder
    let remote_video = GtkBox::new(Orientation::Vertical, 12);
    remote_video.set_vexpand(true);
    remote_video.set_valign(gtk4::Align::Center);
    remote_video.set_halign(gtk4::Align::Center);

    let remote_avatar = Label::new(Some("👤"));
    remote_avatar.add_css_class("video-avatar");

    let remote_name = Label::new(Some("Alice Johnson"));
    remote_name.add_css_class("video-name");

    let call_status = Label::new(Some("Connecting..."));
    call_status.add_css_class("call-status");

    remote_video.append(&remote_avatar);
    remote_video.append(&remote_name);
    remote_video.append(&call_status);

    video_box.append(&remote_video);

    // Local video preview (small corner)
    let local_preview = Frame::new(None);
    local_preview.add_css_class("local-preview");
    local_preview.set_halign(gtk4::Align::End);
    local_preview.set_valign(gtk4::Align::Start);
    local_preview.set_margin_top(20);
    local_preview.set_margin_end(20);
    local_preview.set_width_request(200);
    local_preview.set_height_request(150);

    let local_box = GtkBox::new(Orientation::Vertical, 8);
    local_box.set_valign(gtk4::Align::Center);
    local_box.set_halign(gtk4::Align::Center);

    let local_avatar = Label::new(Some("📹"));
    local_avatar.add_css_class("local-avatar");

    let local_label = Label::new(Some("You"));
    local_label.add_css_class("local-label");

    local_box.append(&local_avatar);
    local_box.append(&local_label);
    local_preview.set_child(Some(&local_box));

    // Controls bar
    let controls_box = GtkBox::new(Orientation::Horizontal, 20);
    controls_box.set_halign(gtk4::Align::Center);
    controls_box.set_margin_top(20);
    controls_box.set_margin_bottom(20);
    controls_box.add_css_class("call-controls");

    let mute_btn = Button::with_label("🎤 Mute");
    mute_btn.add_css_class("call-control-button");

    let video_btn = Button::with_label("📹 Video");
    video_btn.add_css_class("call-control-button");

    let screen_btn = Button::with_label("🖥️ Share");
    screen_btn.add_css_class("call-control-button");

    let end_btn = Button::with_label("📞 End Call");
    end_btn.add_css_class("end-call-button");

    let settings_btn = Button::with_label("⚙️");
    settings_btn.add_css_class("call-control-button");

    controls_box.append(&mute_btn);
    controls_box.append(&video_btn);
    controls_box.append(&screen_btn);
    controls_box.append(&end_btn);
    controls_box.append(&settings_btn);

    main_box.append(&video_box);
    main_box.append(&local_preview);
    main_box.append(&controls_box);

    main_box
}

fn create_create_post_view() -> ScrolledWindow {
    let scroll = ScrolledWindow::new();
    let post_box = GtkBox::new(Orientation::Vertical, 24);
    post_box.set_margin_start(40);
    post_box.set_margin_end(40);
    post_box.set_margin_top(30);
    post_box.set_margin_bottom(30);

    let header = create_section_header("✨ Create Post", "Share your thoughts with the community");
    post_box.append(&header);

    // Post content area
    let content_frame = Frame::new(None);
    content_frame.add_css_class("create-post-frame");

    let content_box = GtkBox::new(Orientation::Vertical, 16);
    content_box.set_margin_start(20);
    content_box.set_margin_end(20);
    content_box.set_margin_top(20);
    content_box.set_margin_bottom(20);

    // User info
    let user_box = GtkBox::new(Orientation::Horizontal, 12);
    let user_avatar = Label::new(Some("👤"));
    user_avatar.add_css_class("create-post-avatar");
    let user_name = Label::new(Some("Your Name"));
    user_name.add_css_class("create-post-user");
    user_box.append(&user_avatar);
    user_box.append(&user_name);
    content_box.append(&user_box);

    // Text entry (simulated with label - GTK4 TextView would be better)
    let text_frame = Frame::new(None);
    text_frame.add_css_class("post-text-area");
    text_frame.set_height_request(150);

    let text_label = Label::new(Some("What's on your mind?"));
    text_label.set_halign(gtk4::Align::Start);
    text_label.set_valign(gtk4::Align::Start);
    text_label.add_css_class("post-placeholder");
    text_frame.set_child(Some(&text_label));
    content_box.append(&text_frame);

    // Media attachments
    let media_label = Label::new(Some("📎 Attachments"));
    media_label.set_halign(gtk4::Align::Start);
    media_label.add_css_class("section-subtitle");
    content_box.append(&media_label);

    let media_grid = gtk4::Grid::new();
    media_grid.set_row_spacing(12);
    media_grid.set_column_spacing(12);

    let media_types = vec![
        ("🖼️", "Photo"),
        ("📹", "Video"),
        ("📄", "Document"),
        ("📍", "Location"),
        ("📊", "Poll"),
        ("😊", "Emoji"),
    ];

    for (i, (icon, label)) in media_types.iter().enumerate() {
        let media_btn = Button::new();
        let btn_box = GtkBox::new(Orientation::Vertical, 6);

        let icon_label = Label::new(Some(icon));
        icon_label.add_css_class("media-type-icon");

        let text_label = Label::new(Some(label));
        text_label.add_css_class("media-type-label");

        btn_box.append(&icon_label);
        btn_box.append(&text_label);
        media_btn.set_child(Some(&btn_box));
        media_btn.add_css_class("media-type-button");

        media_grid.attach(&media_btn, (i % 3) as i32, (i / 3) as i32, 1, 1);
    }
    content_box.append(&media_grid);

    content_frame.set_child(Some(&content_box));
    post_box.append(&content_frame);

    // Post options
    let options_box = GtkBox::new(Orientation::Horizontal, 16);
    options_box.set_margin_top(12);

    let visibility_label = Label::new(Some("👁️ Visibility:"));
    visibility_label.add_css_class("option-label");

    let visibility_btn = Button::with_label("Public 🌍");
    visibility_btn.add_css_class("option-button");

    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);

    let draft_btn = Button::with_label("💾 Save Draft");
    draft_btn.add_css_class("secondary-button");

    let post_btn = Button::with_label("🚀 Post");
    post_btn.add_css_class("create-post-button");

    options_box.append(&visibility_label);
    options_box.append(&visibility_btn);
    options_box.append(&spacer);
    options_box.append(&draft_btn);
    options_box.append(&post_btn);

    post_box.append(&options_box);

    scroll.set_child(Some(&post_box));
    scroll
}
