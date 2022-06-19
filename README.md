# Tauri Plugin Localhost
![Test](https://github.com/tauri-apps/tauri-plugin-localhost/workflows/Test/badge.svg)

A Tauri Plugin that allows your application to use a localhost server instead of Tauri's custom protocol (`tauri://localhost` on Linux and macOS and `https://tauri.localhost` on Windows).

Note that for security reasons Tauri recommends the custom protocol implementation. Use a localhost server only if it is really needed and be careful with your assets.

## Installation
There are three general methods of installation that we can recommend.
1. Pull sources directly from Github using git tags / revision hashes (most secure, good for developement, shown below)
2. Git submodule install this repo in your tauri project and then use `file` protocol to ingest the source
3. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)

For more details and usage see [the example app](examples/vanilla/src-tauri/src/main.rs).
Please note, below in the dependencies you can also lock to a revision/tag in the `Cargo.toml`.

`src-tauri/Cargo.toml`
```yaml
[dependencies]
tauri = { version = "1.0.0" }
portpicker = "0.1"

[dependencies.tauri-plugin-localhost]
git = "https://github.com/tauri-apps/tauri-plugin-localhost"
tag = "tauri-plugin-localhost-v0.1.0"
#branch = "main"
```

Use in `src-tauri/src/main.rs`:

```rust

use tauri::{window::WindowBuilder, WindowUrl};

fn main() {
  let port = portpicker::pick_unused_port().expect("failed to find unused port");
  tauri::Builder::default()
    .plugin(tauri_plugin_localhost::Builder::new(port).build())
    .setup(move |app| {
      WindowBuilder::new(
        app,
        "main".to_string(),
        WindowUrl::External(format!("http://localhost:{}", port).parse().unwrap()),
      )
      .title("Localhost Example")
      .build()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

# License
MIT / Apache-2.0
