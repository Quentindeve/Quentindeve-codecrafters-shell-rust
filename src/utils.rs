use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
};

pub fn search_command_in_env(command_name: &str) -> Option<PathBuf> {
    let env = std::env::var("PATH").unwrap();
    println!("ENV: {}", env);
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

pub fn execute_extern_command(executable_path: PathBuf, arguments: Vec<String>) -> ExitStatus {
    let arguments: Vec<OsString> = arguments
        .iter()
        .skip(1)
        .map(|s| OsString::from(s))
        .collect();

    let mut child = Command::new(executable_path);

    child
        .args(&arguments)
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit());

    child.spawn().unwrap().wait_with_output().unwrap().status
}
