// TODO: add colors

pub fn log_error(err: &str) {
    eprintln!("Error: {}", err);
}

pub fn log_info(info: &str) {
    println!("Info: {}", info);
}

pub fn log_success(success: &str) {
    println!("Success: {}", success);
}