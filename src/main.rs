use std::process::Command;

fn main() -> Result<(), serde_yaml::Error> {
    // println!("Hello, world!");

    let yaml = std::fs::read_to_string("sample.yml").unwrap();

    let yaml_map: serde_yaml::Value = serde_yaml::from_str(&yaml)?;

    let session_name = match &yaml_map["name"] {
        serde_yaml::Value::String(s) => s.clone(),
        _ => String::from("default"),
    };

    // println!("Yaml: {:#?}", yaml_map["name"]);
    // println!("Yaml: {:#?}", yaml_map["root"]);
    // println!("Yaml: {:#?}", yaml_map["windows"]);
    // println!("Yaml: {:#?}", yaml_map["windows"][0]["doom"]);

    Command::new("tmux")
        .args(&["new", "-d", "-s", &session_name])
        .output()
        .expect("Failed to execute process.");

    let mut is_first_window = true;
    match &yaml_map["windows"] {
        serde_yaml::Value::Sequence(s) => {
            for (wi, w) in s.iter().enumerate() {
                match w {
                    serde_yaml::Value::String(cmd) => {
                        Command::new("tmux")
                            .args(&[
                                "send-keys",
                                "-t",
                                format!("{}:{}.{}", &session_name, wi, 0).as_str(),
                                format!("{}", cmd).as_str(),
                                "C-m",
                            ])
                            .output()
                            .expect("Failed to execute process.");
                    }
                    serde_yaml::Value::Mapping(m) => {
                        for (k, v) in m.iter() {
                            // println!("({:#?}, {:#?})", k, v);
                            if !is_first_window {
                                Command::new("tmux")
                                    .args(&["new-window", "-t", &session_name])
                                    .output()
                                    .expect("Failed to execute process.");
                            }

                            is_first_window = false;
                            Command::new("tmux")
                                .args(&[
                                    "rename-window",
                                    "-t",
                                    &session_name,
                                    k.as_str().unwrap_or("shell"),
                                ])
                                .output()
                                .expect("Failed to execute process.");
                            match v {
                                serde_yaml::Value::String(cmd) => {
                                    Command::new("tmux")
                                        .args(&[
                                            "send-keys",
                                            "-t",
                                            format!("{}:{}.{}", &session_name, wi, 0).as_str(),
                                            format!("{}", cmd).as_str(),
                                            "C-m",
                                        ])
                                        .output()
                                        .expect("Failed to execute process.");
                                }
                                serde_yaml::Value::Mapping(inner) => {
                                    if let Some(layout) =
                                        inner.get(&serde_yaml::Value::String("layout".into()))
                                    {
                                        // println!("Has Layout...{:#?}", layout);
                                        if let serde_yaml::Value::String(l) = layout {
                                            match l.as_str() {
                                                "main-horizontal" => {
                                                    Command::new("tmux")
                                                        .args(&[
                                                            "split-window",
                                                            "-t",
                                                            &session_name,
                                                        ])
                                                        .output()
                                                        .expect("Failed to execute process.");
                                                }
                                                "main-vertical" => {
                                                    Command::new("tmux")
                                                        .args(&[
                                                            "split-window",
                                                            "-v",
                                                            "-t",
                                                            &session_name,
                                                        ])
                                                        .output()
                                                        .expect("Failed to execute process.");
                                                }
                                                _ => {}
                                            }
                                        }
                                    }

                                    if let Some(panes) =
                                        inner.get(&serde_yaml::Value::String("panes".into()))
                                    {
                                        if let serde_yaml::Value::Sequence(s) = panes {
                                            println!("Has Panes...{:#?}", s);
                                            for (pi, p) in s.iter().enumerate() {
                                                if let serde_yaml::Value::String(cmd) = p {
                                                    Command::new("tmux")
                                                        .args(&[
                                                            "send-keys",
                                                            "-t",
                                                            format!(
                                                                "{}:{}.{}",
                                                                &session_name, wi, pi
                                                            )
                                                            .as_str(),
                                                            format!("{}", cmd).as_str(),
                                                            "C-m",
                                                        ])
                                                        .output()
                                                        .expect("Failed to execute process.");
                                                }
                                            }
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }
                    }

                    _ => {}
                }
            }
        }
        _ => {}
    }

    // println!("Attaching...");
    Command::new("tmux")
        .args(&["attach-session", "-t", &session_name])
        .status()
        .expect("Failed to execute process.");

    Ok(())
}
