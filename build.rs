fn main() {
    let manifest_dir = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let library_paths = std::collections::HashMap::from([
        ("ui".into(), manifest_dir.join("ui")),
        ("views".into(), manifest_dir.join("ui/views")),
        ("sui".into(), manifest_dir.join("ui/modules/surrealism-ui")),
    ]);
    let config = slint_build::CompilerConfiguration::new().with_library_paths(library_paths);
    slint_build::compile_with_config("ui/app.slint", config).unwrap();
}
