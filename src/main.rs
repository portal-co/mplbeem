use std::fs::{FileType, exists};

use walkdir::WalkDir;

fn main() {
    let mut args = std::env::args();
    args.next();
    let src = args.next().expect("an input");
    let mut d = false;
    let out = loop {
        let a = args.next().expect("an output or argument");
        if d {
            break a;
        };
        match a.strip_prefix("-") {
            None => break a,
            Some(a) => match a {
                "-" => {
                    d = true;
                }
                _ => panic!("invalid option"),
            },
        }
    };
    for w in WalkDir::new(out.clone()) {
        let w = w.expect("to succeed");
        if !w.file_type().is_file() {
            continue;
        }
        let src_path = w.path().to_string_lossy().replace(out.as_str(), &src);

        if exists(&src_path).expect("to have a result") {
            std::fs::copy(src_path, w.path()).expect("to copy the file");
        } else {
            std::fs::remove_file(w.path()).expect("to erse the file");
        }
    }
    if !std::process::Command::new("env")
        .arg("git")
        .arg("add")
        .arg("-A")
        .current_dir(&out)
        .spawn()
        .expect("to add the files")
        .wait()
        .expect("to exit")
        .success()
    {
        panic!("unsuccessfull git invocation")
    }
    if !std::process::Command::new("env")
        .arg("git")
        .arg("commit")
        .arg("-m")
        .arg("MPLSync: sync")
        .current_dir(&out)
        .spawn()
        .expect("to commit them")
        .wait()
        .expect("to exit")
        .success()
    {
        panic!("unsuccessfull git invocation")
    }
    if !std::process::Command::new("env")
        .arg("git")
        .arg("push")
        .current_dir(&out)
        .spawn()
        .expect("to push the files")
        .wait()
        .expect("to exit")
        .success()
    {
        panic!("unsuccessfull git invocation")
    }
}
