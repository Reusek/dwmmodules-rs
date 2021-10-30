use ansi_term::Style;
use ansi_term::Colour::{Blue, Cyan, Yellow};
use crate::modules;

fn plug_extractor() -> String {
    let c = modules::cmd("upower -i /org/freedesktop/UPower/devices/battery_BAT1 | grep -E \"state\"".to_string());
    let r = if c.contains("discharging") {
        " ".to_string()
    } else {
        "\u{f740}".to_string()
    };
    r
}

fn battery_extractor() -> String {
    let mut buf = String::new();
    let percentage = modules::cmd("upower -i /org/freedesktop/UPower/devices/battery_BAT1 | grep -E \"percentage\" | tr -d [:alpha:][:space:]:%".to_string());
    let num_p = percentage.parse::<u32>().unwrap();
    let icon = if num_p < 20 {
        "\u{f244}".to_string()
    } else if num_p < 40 {
        "\u{f243}".to_string()
    } else if num_p < 60 {
        "\u{f242}".to_string()
    } else if num_p < 80 {
        "\u{f241}".to_string()
    } else {
        "\u{f240}".to_string()
    };
    buf.push_str(&icon);
    let per = format!("  {}%", num_p);
    buf.push_str(&per);
    buf
}

pub const M: [modules::Module; 3] = [
    modules::Module {
        prefix: " ",
        postfix: "",
        command: "",
        timeout: 500,
        extractor: Some(plug_extractor)
    },
    modules::Module {
        prefix: "",
        postfix: "  ",
        command: "upower -i /org/freedesktop/UPower/devices/battery_BAT1 | grep -E \"percentage\" | tr -d [:alpha:][:space:]: ",
        timeout: 5000,
        extractor: Some(battery_extractor)
    },
    modules::Module {
        prefix: "\u{f017} ",
        postfix: "",
        command: "date +\"%T\"",
        timeout: 500,
        extractor: None
    }
];

