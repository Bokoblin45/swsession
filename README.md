installation:

1 get the files with git clone                                                             
2 compile it with cargo build --release  OR grab the precompiled binary from the release -->                                                    
3 add the compiled binary to your $PATH for ease of use



How to add your own entry:

- make a new match arm with "{number}" => {let (_ret_val, stdout, stderr) = rash!("pkexec /usr/lib/steamos/steam-set-session SESSION.desktop").expect("failed to set session! check your autologin config for errors!");
- replace {number} with the number of your choice, this will be what you type to switch.
- replace SESSION.desktop with the name of the .desktop file of your compositor / DE / window manager
- (optional but it'll look better if you do) edit the gigantic println! content starting in line 14 so it lists your session.
- COMPILE AND TEST BEFORE MOVING TO YOUR /bin by executing cat /etc/plasmalogin.conf.d/zz-steamos-autologin.conf, then run swsession with cargo run and pick the current desktop, cat /etc/plasmalogin.conf.d/zz-steamos-autologin.conf and MAKE SURE IT IS EXACTLY THE SAME AS BEFORE. if so, you can compile and move to /bin
