// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde_json::Value as JsonValue;
use tauri::{
  plugin::{Plugin, Result as PluginResult},
  AppHandle, Runtime,
};
use tiny_http::{Header, Response, Server};

pub struct Localhost {
  port: u16,
}

impl Localhost {
  pub fn new(port: u16) -> Self {
    Self { port }
  }
}

impl<R: Runtime> Plugin<R> for Localhost {
  fn name(&self) -> &'static str {
    "localhost"
  }

  fn initialize(&mut self, app: &AppHandle<R>, _: JsonValue) -> PluginResult<()> {
    let port = self.port;
    let asset_resolver = app.asset_resolver();
    std::thread::spawn(move || {
      let server = Server::http(&format!("localhost:{}", port)).expect("Unable to spawn server");
      for request in server.incoming_requests() {
        if let Some(asset) = asset_resolver.get(request.url().into()) {
          let mut response = Response::from_data(asset.bytes);
          response.add_header(Header::from_bytes(&b"Content-Type"[..], asset.mime_type).unwrap());
          request.respond(response).expect("unable to setup response");
        }
      }
    });
    Ok(())
  }
}
