use std::process::{ Command };

fn message_generator() -> String {
    let msg: String = "dgg".to_string();
    msg
}

fn update_git() {
    let app: &str = "git";
    println!("called");

    let add_cmd = Command::new(app).arg("add").arg("-a").output().expect("Failed to git add");
    if !add_cmd.status.success() {
        eprintln!("Error: Failed to git add");
        // exit(1);
    }

    let commit_cmd = Command::new(app)
        .arg("commit")
        .arg("-m")
        .arg(message_generator())
        .output()
        .expect("faild to git commit");

    if !add_cmd.status.success() {
        eprintln!("Error: Failed to commit");
        // exit(1);
    }

    let push_cmd = Command::new(app)
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to git push");

    if !push_cmd.status.success() {
        eprintln!("Error: Failed to push");
    }
}

fn main() {
    println!("Hello, world");
    // update_git();
}
