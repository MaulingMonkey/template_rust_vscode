# Overkill Windows Setup

1. Run installers:
    - [Visual Studio Code](https://code.visualstudio.com/)
    - [Visual Studio](https://visualstudio.microsoft.com/downloads/) 's C++ tools **or** [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2017)
    - [MSYS2](http://www.msys2.org/) (Use default path - we want `gdb`, useful for buildring `rustc` itself too [if you're into that](https://github.com/rust-lang/rust#mingw))
    - [Rustup](https://rustup.rs/) (just use the regular windows installer, not WSL yet)
2. Windows configuration:
    - [Enable WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) (Windows 10+)
    - **REBOOT**
3. Run `cmd`:
    ```cmd
    rustup toolchain install stable-x86_64-pc-windows-msvc
    rustup toolchain install stable-x86_64-pc-windows-gnu
    rustup target install --toolchain=stable-x86_64-pc-windows-msvc x86_64-pc-windows-msvc
    rustup target install --toolchain=stable-x86_64-pc-windows-msvc i686-pc-windows-msvc
    rustup target install --toolchain=stable-x86_64-pc-windows-gnu  x86_64-pc-windows-gnu
    rustup target install --toolchain=stable-x86_64-pc-windows-gnu  i686-pc-windows-gnu
    REM ...do we want to install these per --toolchain=...?  Per --target=...?
    rustup component add rust-src
    rustup component add rls
    ```
4. Run WSL `bash` (Windows 10+):
    ```sh
    sudo apt-get update
    sudo apt install gcc gdb
    curl https://sh.rustup.rs -sSf | sh
    ```
5. Run `C:\msys64\msys2_shell -mingw64`:
    ```sh
    pacman -Sy pacman-mirrors
    pacman -S mingw-w64-x86_64-gdb
    ```
6. Run `code`:
    - Open extensions (`Ctrl`+`Shift`+`X`)
    - Install workspace recommended extensions:  [ms-vscode.cpptools](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools) (for debugging), [rust-lang.rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) (for syntax highlighting, intellisense.)
    - Try and debug (`F5`) some targets:
        - `Debug x86_64-pc-windows-msvc` should test out your VS install.
        - `Debug x86_64-pc-windows-gnu` should test out your MSYS2 install.  Known issues with pausing / setting breakpoints while unpaused.
        - `Debug [default]-unknown-linux-gnu` should test out your WSL install.
        - `Debug` should test out your default toolchain with a VS style debugger.
