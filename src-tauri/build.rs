fn main() {
    let mut builder = tauri_build::Builder::default();

    #[cfg(windows)]
    {
        builder = builder.windows_attributes(
            tauri_build::WindowsAttributes::new()
                .app_manifest(include_str!("windows-manifest.xml"))
        );
    }

    builder.build();
}
