# What even is this?

A thorough demo of everything you might want to configure for rust development in VS Code,
along with a small hello world style program to test it out on.

# Possible usage

```cmd
git clone https://github.com/MaulingMonkey/template_rust_vscode.git
cargo new NEW_PROJECT --bin
robocopy template_rust_vscode\.vscode NEW_PROJECT\.vscode *.*
robocopy template_rust_vscode         NEW_PROJECT         Setup.md
```
* [Setup everything](Setup.md)
* Replace `${workspaceFolderBasename}` throughout `.vscode\launch.json` with your new `Cargo.toml` `package.name`, so you can rename the folder locally.
* Cull unused stuff from `Setup.md`
