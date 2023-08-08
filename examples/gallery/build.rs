// SPDX-FileCopyrightText: 2022 Florian Blasius <co_sl@tutanota.com>
// SPDX-License-Identifier: MIT

fn generate_imports() {
    book_flip::generate_import().unwrap();
    coop_widgets::generate_import().unwrap();
}

const APP: &str = "ui/app.slint";

#[cfg(feature = "default")]
fn main() {
    use slint_build::CompilerConfiguration;

    generate_imports();
    slint_build::compile_with_config(
        APP,
        CompilerConfiguration::new().with_style("fluent".into()),
    )
    .unwrap();
}

#[cfg(any(
    feature = "mcu-board-support",
    target_os = "redox",
    feature = "slint_coop"
))]
fn main() {
    generate_imports();
    let config = slint_build::CompilerConfiguration::new()
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);
    slint_build::compile_with_config(APP, config).unwrap();
    slint_build::print_rustc_flags().unwrap();
}
