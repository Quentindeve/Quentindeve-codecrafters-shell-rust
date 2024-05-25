use std::path::{Path, PathBuf};

pub fn search_command_in_env(command_name: &str) -> Option<PathBuf> {
    let env = std::env::var("PATH").unwrap();
    let mut paths = env
        .split(":")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    paths.push(
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    );

    for path in paths {
        let path = Path::new(&path);
        let files = std::fs::read_dir(path);

        if let Ok(files) = files {
            for file in files {
                let file = file.unwrap();
                let file_name = file.file_name();
                let file_name = file_name.to_str().unwrap();

                if file_name == command_name {
                    return Some(file.path());
                }
            }
        }
    }

    None
}
