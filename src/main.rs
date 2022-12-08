use std::process::Command;
use std::str;

fn fmt_status(project:Vec<u16>) -> String {
    let mut count = 0;
    let mut fmted_status = String::new();
    for i in project {
        match i {
            0 => (),
            _ => {
                match count {
                    0 => fmted_status.push_str(&format!("%B%F{{yellow}}↻{}%f%b", i)),
                    1 => fmted_status.push_str(&format!("%B%F{{blue}}★{}%f%b", i)),
                    2 => fmted_status.push_str(&format!("%B%F{{blue}}R{}%f%b", i)),
                    3 => fmted_status.push_str(&format!("%B%F{{green}}▲{}%f%b", i)),
                    4 => fmted_status.push_str(&format!("%B%F{{red}}▼{}%f%b",i)),
                    5 => fmted_status.push_str("%B%F{{red}}U%f%b"),
                    6 => fmted_status.push_str(&format!("%F{{green}}+{}%f", i)),
                    7 => fmted_status.push_str(&format!("%F{{red}}-{}%f",i)),
                    _ => break,
                }
            },
        }
        count += 1;
    }
    return fmted_status;
}

fn gst_injest(project:&mut Vec<u16>, stdout:&str){
    for i in stdout.split("\n").into_iter().enumerate() {
        match i.1.get(0..2) {
            Some(val) => {
                match val {
                    " M" => project[0]+= 1,
                    "??" => project[1]+= 1,
                    " A" => project[2]+= 1,
                    " D" => project[3]+= 1,
                    " R" => project[4]+= 1,
                    " U" => project[5]+= 1,
                    _ => (),
                }
            },
            _ =>()
        }
    }
}

fn gd_injest(project:&mut Vec<u16>, stdout:&str) {
    if stdout.len() == 0  {
        return;
    }
    let mut count = 0;
    for splitted in stdout.split(',').enumerate() {
        match count {
            1 => project[6] = splitted.1.get(0..3).unwrap().to_string().trim().parse::<u16>().unwrap(),
            2 => project[7] = splitted.1.get(0..3).unwrap().to_string().trim().parse::<u16>().unwrap(),
            _ => (),
        }
        count += 1;
    }
}

fn main() {
    let mut gst = Command::new("git");
    let git_status = gst.args(["status", "--porcelain"]).output().expect("[ERROR] git status");

    let mut project: Vec<u16> = vec![0;8];

    if let Ok(git_status_stdout) = str::from_utf8(&git_status.stdout) {
        gst_injest(&mut project, &git_status_stdout);
        if project[0] != 0 {
            let mut gd = Command::new("git");
            let git_diff = gd.args(["diff", "--shortstat"]).output().expect("[ERROR] git diff");
            if let Ok(git_diff_stdout) = str::from_utf8(&git_diff.stdout) {
                gd_injest(&mut project, &git_diff_stdout);
                println!("{}",fmt_status(project));
            }
        }
    } else {
        println!("Wrong git status format given");
    }
}
