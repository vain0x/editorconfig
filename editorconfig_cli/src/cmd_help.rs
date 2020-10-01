pub(crate) fn string_is_help_flag(s: &str) -> bool {
    match s {
        "-h" | "-help" | "--help" => true,
        _ => false,
    }
}

pub(crate) fn exec_help_cmd() {
    print!(
        include_str!("cmd_help.txt"),
        command = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
    );
}
