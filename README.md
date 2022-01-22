# Simple Launcher for Windows
Launches programs or opens websites with a click.  
Just provide a simple config file like:
```
[
  {
    "action": "openurl",
    "title": "Some Homepage",
    "description": "Opens a new browser window",
    "path": "https://test.com"
  },
  {
    "action": "launchscript",
    "title": "A Script",
    "description": "Starts a script",
    "path": "c:\\script.bat"
  }
]
```

## Build for Windows
```
cargo build --target x86_64-pc-windows-gnu
```
But before that install all dependencies like described here:

https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html

I had to install ninja from here:  
https://pypi.org/project/ninja/


And fix an issue in libjpeg-turbo's meson.build-file:  
https://github.com/mesonbuild/wrapdb/commit/f24069784f129c35ea3c6750af0b7dab612216a9

