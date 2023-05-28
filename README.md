# rust_windows_revshell

This is a simple Rust reverse shell written to run on a Windows machine and provide a reverse shell connection back to a listener.  It takes two arguments, an IP and a port, and will run any commands given to it under the user context it was run from.

## compiling

If compiling on linux, you will need the rust language:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## build

To build into a windows binary:

./build.sh (linux)
./build.bat (powershell)
build (cmd)

## usage

Open a listener on the IP and port you compiled with, usually using netcat:

nc -lvp <port>

On windows, run the executable however you would trigger it.

## DISCLAIMER
 This module is for defensive testing, training, and for demonstrative purposes only.  Misusing this code for illegal or unethical purposes may result in severe legal and ethical consequences. So, please use this code responsibly and ethically. I am not responsible for any damages caused by misuse.

## windows build

[Currently not supported, if you want to figure out why ming is failing and get this to compile, please submit a PR]

You will need rust and mingw installed if compiling on Windows. Chocolatey is a package manager that works well for this purpose. You can install with Powershell as admin:

Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

Once you have Chocolatey you can install the 2 packages needed with:

choco install rust
choco install mingw

[Currently failing:]

   Compiling reverse-shell-win v0.1.1 (D:\dev\rust\rust_windows_revshell)
error: linking with `x86_64-w64-mingw32-gcc` failed: exit code: 1
  |
  = note: "x86_64-w64-mingw32-gcc" "-fno-use-linker-plugin" "-Wl,--dynamicbase" "-Wl,--disable-auto-image-base" "-m64" "-Wl,--high-entropy-va" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsbegin.o" "C:\\Users\\DEV\\AppData\\Local\\Temp\\rustc08p1Fw\\symbols.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.0.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.1.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.10.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.11.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.12.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.13.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.14.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.15.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.2.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.3.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.4.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.5.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.6.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.7.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.8.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.reverse_shell_win.5ae6d0b0-cgu.9.rcgu.o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.rnw6vw7m30eyhn5.rcgu.o" "-L" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps" "-L" "D:\\dev\\rust\\rust_windows_revshell\\target\\release\\deps" "-L" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-Wl,-Bstatic" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libstd-4208b5b050761eab.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libpanic_unwind-c24e86db72ac50d0.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libobject-8130621e6418bc58.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libmemchr-08ae56106143c1db.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libaddr2line-ccfb784edb974a4a.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libgimli-4ab6c884d279676f.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\librustc_demangle-9ddceef41b9e9295.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libstd_detect-d8366e410ad5f9d7.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libhashbrown-ce69f37d51637f0b.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libminiz_oxide-19f6886d9ebc20bf.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libadler-25358c2b66d0aa3d.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\librustc_std_workspace_alloc-d18d5ff4d2dae958.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libunwind-0cef8242de7339bc.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libcfg_if-edcd03acb0b5d165.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\liblibc-21b2a42330a7f2e3.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\liballoc-ee725b426657593b.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\librustc_std_workspace_core-d6f1a8662e8b71a9.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libcore-84ddc19bfcf64c94.rlib" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libcompiler_builtins-6b61207aa476e831.rlib" "-Wl,-Bdynamic" "-lkernel32" "-ladvapi32" "-luserenv" "-lkernel32" "-lws2_32" "-lbcrypt" "-lgcc_eh" "-l:libpthread.a" "-lmsvcrt" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-luser32" "-lkernel32" "-Wl,--nxcompat" "-L" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-o" "D:\\dev\\rust\\rust_windows_revshell\\target\\x86_64-pc-windows-gnu\\release\\deps\\reverse_shell_win-cbe819160bd2e214.exe" "-Wl,--gc-sections" "-no-pie" "-Wl,-O1" "-nodefaultlibs" "C:\\Users\\DEV\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsend.o"
  = note:

error: could not compile `reverse-shell-win` due to previous error