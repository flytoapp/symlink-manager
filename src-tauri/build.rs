fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_manifest_file("windows-manifest.xml");
        res.compile().unwrap();
    }
    tauri_build::build()
}
