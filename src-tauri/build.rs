fn main() {
    tauri_build::build();

    #[cfg(windows)]
    {
        embed_manifest::embed_manifest(embed_manifest::new_manifest("Symlink Manager")
            .requested_execution_level(embed_manifest::ExecutionLevel::RequireAdministrator))
            .expect("Failed to embed manifest");
    }
}
