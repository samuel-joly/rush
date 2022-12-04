use std::process::Command;

fn main() {
    let mut git = Command::new("git");
    let git_status = git.args(["status", "--porcelain"]).output().expect("[ERROR] git status");
    let mut count : i32 = 0;
    for i in &git_status.stdout {
        match i {
            63 => count += 1,
            _other => continue
        }
    };
    println!("{}", count/2);
}
