fn main() {
    // TODO: Figure out how to get qt style working. Missing libraries in the distrobox?
    let slint_config =slint_build::CompilerConfiguration::new()
        .with_style("material".into())
        ;
    slint_build::compile_with_config("ui/appwindow.slint", slint_config).unwrap();
}
