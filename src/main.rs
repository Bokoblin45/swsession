use rsbash::rash;
use std::io;

fn main() {
    let (_ret_val, stdout, _stderr) = rash!("loginctl session-status | grep Desktop:").expect("Failed to fetch active desktop info");
    let now = stdout.trim_start_matches("Desktop:");
    println!("Active: {now}");
    let (_, active_r, err) = rash!("cat /etc/plasmalogin.conf.d/zz-steamos-autologin.conf | grep .desktop").expect("failed to fetch selected session, are you sure you are using plasmalogin and your autologin is in /etc/plasmalogin.conf.d/zz-steamos-autologin.conf ?");
    let active_st = active_r.trim().trim_start_matches("Session=");
    let active = active_st.trim_end_matches(".desktop");
    println!("Selected: {active}");
    println!("{err}");
    println!("pick DE:
    1: Hyprland 
    -dynamic tyling wm with in this case end_4 ii dots.
    
    2: Gamescope 
    -Steamos session for purely gaming, great on handhelds.
    
    3: Plasma 
    -stacking DE, works great with a steam deck, thing that you use when you click desktop mode in gamescope.
    
    4: niri
    -scrollable tiling wm with iNiR dotfiles.
    
    5: COSMIC
    -system76's new DE, written in rust");
    
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("failed to read input");
    let input = input.trim().to_lowercase();

    match &input as &str {
        "1" => {let (_ret_val, stdout, stderr) = rash!("pkexec /usr/lib/steamos/steam-set-session hyprland.desktop").expect("failed to set session! check your autologin config for errors!");
                println!("session set to hyprland, {stdout} {stderr}");},
        "2" => {let (_ret_val, stdout, stderr) = rash!("steamos-session-select oneshot").expect("failed to set session!");
                println!("session set to: Gamescope, {stdout} {stderr}");},
        "3" => {let (_ret_val, stdout, stderr) = rash!("pkexec /usr/lib/steamos/steam-set-session plasma.desktop").expect("failed to set session!");
                println!("session set to plasma, {stdout} {stderr}");},
        "4" => {let (_ret_val, stdout, stderr) = rash!("pkexec /usr/lib/steamos/steam-set-session niri.desktop").expect("failed to set session!");
        println!("session set to niri, {stdout} {stderr}");},
        "5" => {let (_ret_val, stdout, stderr) = rash!("pkexec /usr/lib/steamos/steam-set-session cosmic.desktop").expect("failed to set session!");
        println!("session set to COSMIC, {stdout} {stderr}");},
        _ => {println!("pick a valid option, you typed: {}", input);}
    }
    println!("please reboot for any changes to apply.");
}