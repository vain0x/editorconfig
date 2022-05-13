fn is_safe_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '+' || c == '#'
}

pub(crate) fn exec_generate_cmd(mut args: impl Iterator<Item = String>) {
    let lang_list = args.next().unwrap_or(String::new());
    let mut langs = lang_list
        .split(',')
        .map(|s| s.trim_matches(|c: char| !is_safe_char(c)))
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    langs.sort();
    langs.dedup();
    if langs.is_empty() {
        panic!("no language specified");
    }

    let urls = langs
        .into_iter()
        .map(|lang| format!("https://raw.githubusercontent.com/vain0x/editorconfig/master/templates/{}.editorconfig", lang));

    let output = std::process::Command::new("curl")
        .arg("-sL")
        .arg("--fail-early")
        .args(urls)
        .output()
        .expect("curl failed. Some language name may be wrong.");

    let stdout = std::str::from_utf8(&output.stdout).expect("non UTF-8 output");
    if stdout.contains("404: Not Found") {
        panic!("Encountered 404 Not Found. Some language name was invalid?");
    }

    let text = format!(
        "root = true\n{}",
        stdout.replace("\r\n", "\n").replace("[", "\n[")
    );

    std::fs::write(".editorconfig", text).expect("couldn't write to .editorconfig");
}
