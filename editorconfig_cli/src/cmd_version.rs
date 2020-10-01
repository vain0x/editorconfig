pub(crate) fn string_is_version_flag(s: &str) -> bool {
    match s {
        "-V" | "-version" | "--version" => true,
        _ => false,
    }
}

pub(crate) fn exec_version_cmd() {
    println!(
        "{command} v{version}",
        command = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
    );
}
