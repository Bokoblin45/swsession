use std::fs;
use std::io;
use std::collections::HashMap;
use rsbash::rash;

fn main() {
    let (_ret_val, stdout, _stderr) = rash!("loginctl session-status | grep Desktop:").expect("Failed to fetch active desktop info");
    let now = stdout.trim_start_matches("Desktop:");
    println!("Active: {now}");
    let (_, active_r, err) = rash!("cat /etc/plasmalogin.conf.d/zz-steamos-autologin.conf | grep .desktop").expect("failed to fetch selected session, are you sure you are using plasmalogin and your autologin is in /etc/plasmalogin.conf.d/zz-steamos-autologin.conf ?");
    let active_st = active_r.trim().trim_start_matches("Session=");
    let active = active_st.trim_end_matches(".desktop");
    println!("Selected: {active}");
    print!("  {err}");
    let mut map = HashMap::new();
    let mut now = 0;
    let wayland_sessions_try= fs::read_dir("/usr/share/wayland-sessions"); // read default wayland session directory, make sure it doesn't error if it doesn't exist
    match wayland_sessions_try {
        Ok(wayland_sessions) => {
            println!("Available wayland sessions:");
            for path in wayland_sessions {
                let entry = path.expect("found no wayland sessions in /usr/share/wayland-sessions. (does the directory exist?)");
                now += 1;

                println!("{}: {}", now, entry.file_name().to_string_lossy().trim_end_matches(".desktop"));
                map.entry(now.to_string()).or_insert(entry);
            }
        },
        Err(_error) => {println!("Some kind of weird error while trying to read /usr/share/wayland-sessions.")}
    } 
    
    let xsessions_try= fs::read_dir("/usr/share/xsessions"); // read default xsession directory, make sure it doesn't error if it doesn't exist
    match xsessions_try {
        Ok(xsessions) => {
            println!("\nAvailable x sessions:");
            for path in xsessions {
                let entry = path.expect("found no xsessions. (does the directory exist?)");
                now += 1;

                println!("{}: {}", now, entry.file_name().to_string_lossy().trim_end_matches(".desktop"));
                map.entry(now.to_string()).or_insert(entry);
            }
        },
        Err(_error) => {println!("Some kind of weird error while trying to read /usr/share/xsessions.")}
    } 

    // now get user input
    println!("session to use:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("that's no valid input");
    let session = input.trim().to_string();

    match map.get(&session) {
        Some(session) => {let cmd = format!("pkexec /usr/lib/steamos/steam-set-session {}", session.file_name().to_string_lossy());
                                     let (_ret_val, _stdout, stderr) = rash!(cmd).expect("command failed, check autologin config!");
                                     println!("done {}", stderr)},
        None => {println!("not found");}
    }
}
