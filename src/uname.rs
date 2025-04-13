use super::{OSInformation, OSType, TryInformation};
use regex::Regex;
use std::process::Command;
use utils::get_first_capture;

#[derive(Debug, PartialEq)]
pub struct Uname {
    distro: Option<String>,
    version: Option<String>,
}

impl TryInformation for Uname {
    fn try_information() -> Option<OSInformation> {
        retrieve().map(parse).and_then(|r| {
            let version = r.version.unwrap_or(OSInformation::default_version());
            let distro = r.distro.unwrap_or("".to_string()).to_lowercase();
            match distro.as_str() {
                "cygwin" => Some(OSInformation::new(OSType::Cygwin, version)),
                "linux" => Some(OSInformation::new(OSType::GenericLinux, version)),
                _ => None,
            }
        })
    }
}

fn retrieve() -> Option<String> {
    Command::new("uname")
        .arg("-or")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .ok()
}

fn parse<S: AsRef<str>>(file: S) -> Uname {
    let trimmed_file = file.as_ref().trim();

    let distrib_regex = Regex::new(r"(\w+)$").unwrap();
    let version_regex = Regex::new(r"^([\w\.]+)").unwrap();

    let distro = get_first_capture(&distrib_regex, trimmed_file);
    let version = get_first_capture(&version_regex, trimmed_file);

    Uname { distro, version }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cygwin_3_4_8() {
        let sample = r#"3.4.8-1.x86_64 Cygwin"#;

        assert_eq!(
            parse(sample),
            Uname {
                distro: Some("Cygwin".to_string()),
                version: Some("3.4.8".to_string()),
            }
        );
    }

    #[test]
    fn fedora_linux_6_13_9() {
        let sample = r#"6.13.9-200.fc41.x86_64 GNU/Linux"#;

        assert_eq!(
            parse(sample),
            Uname {
                distro: Some("Linux".to_string()),
                version: Some("6.13.9".to_string()),
            }
        );
    }
}
