use sysinfo::{System, Signal};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_processes();

    println!(r#"
    ┓         
    ┣┓┓┏┏┓┏┓┏┏
    ┛┗┗┫┣┛┗┻┛┛
       ┛┛     
    "#);

    for process in sys.processes_by_name("RobloxCrashHandler.exe") {
        println!("[*] Terminating hyperion :3");
        if !process.kill_with(Signal::Kill).unwrap_or(false) {
            eprintln!("[!] Failed to terminate process :C");
        }
    }
}
