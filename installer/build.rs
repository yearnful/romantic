fn main() {
    slint_build::compile("ui/app.slint").unwrap();
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/brand/romantic.ico");
        res.set("ProductName", "romantic installer");
        let _ = res.compile();
    }
}
