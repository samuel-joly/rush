use std::process::Command;
use std::str;

fn main() {
    let mut git = Command::new("git");
    let git_status = git.args(["status", "--porcelain"]).output().expect("[ERROR] git status");
    let mut flagged = Status(0,0,0,0,0,false,0);

    if let Ok(stdout) = str::from_utf8(&git_status.stdout) {
        match_flag(&mut flagged, &stdout);
        let ret_str = format!("%B%F{{yellow}}(↻{})%f%b%F{{blue}}★{}%f%B%F{{green}}▲{}%f%b%F{{red}}▼{}%f{}%F{{red}}{}%f{}",
                              flagged.1, // Untracked
                              flagged.0, // Modified
                              flagged.2, // Added
                              flagged.3, // Deleted
                              flagged.4, // Renamed
                              flagged.5, // Unmerged Bool
                              flagged.6 // Unmerged count
        );
        println!("{}", ret_str);
    } else {
        println!("Wrong git status format given");
    }
}


#[derive(Debug)]
struct Status (i8, i8, i8, i8, i8, bool, i8);

// "UU DU UD AU UA"
fn match_flag(flagged: &mut Status, stdout:&str) {
    for i in stdout.split("\n").into_iter().enumerate() {
        match i.1.get(0..2) {
            Some(val) => {
                match val {
                    " M" => flagged.0 += 1,
                    "??" => flagged.1 += 1,
                    " A" => flagged.2 += 1,
                    " D" => flagged.3 += 1,
                    " R" => flagged.4 += 1,
                    " U" => {
                        flagged.5 = true;
                        flagged.6 += 1;
                    },
                    _ => println!(),
                }
            },
            _ =>()
        }
    }
}
