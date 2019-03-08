use nix::unistd::*;

fn main() {
    let pid = fork();
    match pid {
        Ok(x) => {
            match x {
                ForkResult::Child => {
                    println!("[PID {}] This is the child process", getpid());    
                }
                _ => {
                    println!("[PID {}] This is the parent process", getpid());
                }
            }
        }
        _ => {
            panic!("fork error");
        }
    }
}