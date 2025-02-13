use steel_editorconfig::build_module;

fn main() {
    build_module()
        .emit_package_to_file("libsteel_editorconfig", "editor-config.scm")
        .unwrap()
}
