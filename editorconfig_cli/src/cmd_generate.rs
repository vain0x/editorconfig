use std::{fs, mem::take, process::Command, str};

static INDEX_URL: &'static str =
    "https://raw.githubusercontent.com/vain0x/editorconfig/main/templates/index.txt";

fn template_url(lang: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/vain0x/editorconfig/main/templates/{}.editorconfig",
        lang
    )
}

fn fetch_index() -> String {
    let output = Command::new("curl")
        .arg("-sL")
        .arg("--fail-early")
        .arg(INDEX_URL)
        .output()
        .expect("curl failed. Some language name may be wrong.");

    let stdout = str::from_utf8(&output.stdout).expect("non UTF-8 output");
    if stdout.contains("404: Not Found") {
        panic!("Encountered 404 Not Found. GitHub is down perhaps?");
    }
    stdout.to_string()
}

pub(crate) fn exec_generate_cmd(mut args: impl Iterator<Item = String>) {
    let langs_arg = args.next().unwrap_or(String::new());
    let mut input_langs = langs_arg
        .split(',')
        .map(|s| s.to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    input_langs.sort();
    input_langs.dedup();
    if input_langs.is_empty() {
        panic!("no language specified");
    }

    let index = fetch_index();

    // Check lang names and resolve aliases.
    let langs = {
        let mut output = vec![];

        for line in index.lines() {
            let names = line.split(',').collect::<Vec<&str>>();
            let (&lang, aliases) = match names.split_first() {
                Some(it) => it,
                None => continue,
            };

            if let Some(i) = input_langs
                .iter()
                .position(|input| [lang].iter().chain(aliases).any(|&alias| alias == input))
            {
                // mark as consumed
                take(&mut input_langs[i]);

                output.push(lang.to_string());
                continue;
            }
        }

        output.sort();
        output.dedup();
        output
    };

    let unused = input_langs
        .iter()
        .map(String::as_str)
        .filter(|&input| !input.is_empty())
        .collect::<Vec<_>>();
    if !unused.is_empty() {
        eprintln!("warn: unknown or duplicated names ignored: '{}'", unused.join(","));
    }

    let urls = langs.into_iter().map(|lang| template_url(&lang));

    let output = Command::new("curl")
        .arg("-sL")
        .arg("--fail-early")
        .args(urls)
        .output()
        .expect("curl failed. Some language name might be wrong.");

    let stdout = str::from_utf8(&output.stdout).expect("non UTF-8 output");
    if stdout.contains("404: Not Found") {
        panic!("Encountered 404 Not Found. Some language name was invalid?");
    }

    let text = format!(
        "root = true\n{}",
        stdout.replace("\r\n", "\n").replace("[", "\n[")
    );

    fs::write(".editorconfig", text).expect("couldn't write to .editorconfig");
}
