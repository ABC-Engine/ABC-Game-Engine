use std::env;
use std::fs::File;
use std::process::{Command, Stdio};

// i made a random generated argument to prevent anyone from accidentally using this argument
const ABC_ERROR_ARG: &str = "ZAjRo6ydRgerpaPLm8iZ";
const MAX_LOG_COUNT: usize = 20;
const CRASH_DIR: &str = "./crashes";

pub(crate) fn crash_handler() {
    if !env::args().any(|arg| arg == ABC_ERROR_ARG) {
        if std::fs::read_dir(CRASH_DIR).is_err() {
            std::fs::create_dir(CRASH_DIR).expect("Could not create crashes folder");
        }

        remove_empty_logs();
        remove_old_logs();

        let date_and_time = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
        let log_path = format!("crashes/error-{}.log", date_and_time);

        //println!("logging error to: {}", log_path);
        let log_file = File::create(log_path).expect("Could not create log file");
        let stderr = Stdio::from(log_file);

        let new_args: Vec<String> = env::args().chain(vec![ABC_ERROR_ARG.to_string()]).collect();
        let _child =
            Command::new(std::env::current_exe().expect("Could not get current executable"))
                .args(new_args)
                .stderr(stderr)
                .spawn()
                .expect("Failed to start child process");

        // exit the program, the child process will be started which runs the program again the exact same way but skips this block
        std::process::exit(0);
    }
}

fn remove_empty_logs() {
    // get all files in the current directory
    let files = std::fs::read_dir(CRASH_DIR).expect("Could not read directory");

    for file in files {
        let file = file.expect("Could not get file");
        let file_name = file.file_name();
        let file_name = file_name
            .to_str()
            .expect("Could not convert file name to string");

        // check if the file is a log file
        if file_name.ends_with(".log") && file_name.starts_with("error-") {
            let file_path = file.path();
            let file_size = file.metadata().expect("Could not get file metadata").len();

            // if the file is empty, delete it
            if file_size == 0 {
                std::fs::remove_file(file_path).expect("Could not remove file");
            }
        }
    }
}

fn remove_old_logs() {
    // get all files in the current directory
    let files = std::fs::read_dir(CRASH_DIR).expect("Could not read directory");

    let mut logs: Vec<String> = Vec::new();

    // sort the files by date
    for file in files {
        let file = file.expect("Could not get file");
        let file_name = file.file_name();
        let file_name = file_name
            .to_str()
            .expect("Could not convert file name to string");

        // check if the file is a log file
        if file_name.ends_with(".log") && file_name.starts_with("error-") {
            logs.push(file_name.to_string());
        }
    }
    if logs.len() <= MAX_LOG_COUNT {
        return;
    }

    logs.sort();

    // remove the oldest logs
    for log in logs.iter().take(logs.len() - MAX_LOG_COUNT) {
        std::fs::remove_file(log).expect("Could not remove file");
    }
}
