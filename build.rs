fn main() {
    // Only compile resources on Windows
    #[cfg(target_os = "windows")]
    {
        // Embed icon in Windows executable
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icon.ico");
        res.set("ProductName", "LinkWithMentor");
        res.set("FileDescription", "LinkWithMentor - Connect with your mentor");
        res.set("CompanyName", "LinkWithMentor");
        res.compile().unwrap();
    }
}