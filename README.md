installation:

1 get the files with git clone                                                             
2 compile it with cargo build --release  OR grab the precompiled binary from the release -->                                                    
3 add the compiled binary to your $PATH for ease of use



How to add your own entry:

- make a new else if input == "number" where number is the desired number to be pressed when it is to be selected.
- make a new array with return value, standard output and standard error as its variables and make the value rash!("pkexec /usr/lib/steamos/steam-set-session <session.desktop>").unwrap(); where <session> is the desired session to boot into.
- (optional but it'll look better if you do) edit the gigantic println! content starting in line 14 so it lists your session.      
