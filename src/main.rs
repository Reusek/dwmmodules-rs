use std::sync::mpsc::channel;

mod modules;
mod config;

fn build(module: &modules::Module) -> String {
    let mut buf = String::new();
    buf.push_str(module.prefix);

    let content = match module.extractor {
        Some(x) => {
            (x)()
        },
        None => {
            modules::cmd(module.command.to_string())
        }
    };

    buf.push_str(&content);
    buf.push_str(module.postfix);
    buf
}


fn main() {
    let conflen = config::M.len();
    let (tx, rx) = channel::<(usize, String)>();
    
    for i in 0..conflen {
        let tx = tx.clone();
        println!("{}", i);
        std::thread::spawn(move || {
           loop {
                tx.send((i, build(&config::M[i]))).unwrap();
                let time_to_sleep = std::time::Duration::from_millis(config::M[i].timeout.into());
                std::thread::sleep(time_to_sleep);
            }
        });
    }

    let mut data = vec![String::new(); conflen];


    loop {
        let (id, msg) = rx.recv().unwrap();
        data[id] = msg;
        let builded_string = data.join("");
        std::process::Command::new("xsetroot")
            .arg("-name")
            .arg(builded_string)
            .output()
            .expect("failed to execute process");

        // println!("id: {} msg: {}", id, msg);
    }
}
