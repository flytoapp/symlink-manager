fn main() {
    tauri_build::build();

    #[cfg(windows)]
    {
        use embed_manifest::{embed_manifest, new_manifest};
        use embed_manifest::manifest::ExecutionLevel;

        embed_manifest(new_manifest("Symlink Manager")
            .requested_execution_level(ExecutionLevel::RequireAdministrator))
            .expect("Failed to embed manifest");
    }
}
