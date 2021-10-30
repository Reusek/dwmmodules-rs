
pub fn cmd(c: String) -> String {
    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(c)
        .output()
        .expect("failed to execute process");

    let sout = String::from_utf8(output.stdout).expect("Found invalid utf-8");

    let mut chars = sout.chars();

    chars.by_ref().take_while(|&c| c != '\n').collect::<String>()
}


pub struct Module {
    pub prefix: &'static str,
    pub postfix: &'static str,
    pub command: &'static str,
    pub timeout: u32,
    pub extractor: Option<fn() -> String>
}

