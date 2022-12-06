// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use http::Uri;
use tauri::{
  plugin::{Builder as PluginBuilder, TauriPlugin},
  Runtime,
};
use tiny_http::{Header, Response as HttpResponse, Server};

pub struct Request {
  url: String,
}

impl Request {
  pub fn url(&self) -> &str {
    &self.url
  }
}

pub struct Response {
  headers: HashMap<String, String>,
}

impl Response {
  pub fn add_header<H: Into<String>, V: Into<String>>(&mut self, header: H, value: V) {
    self.headers.insert(header.into(), value.into());
  }
}

type OnRequest = Option<Box<dyn Fn(&Request, &mut Response) + Send + Sync>>;

pub struct Builder {
  port: u16,
  on_request: OnRequest,
}

impl Builder {
  pub fn new(port: u16) -> Self {
    Self {
      port,
      on_request: None,
    }
  }

  pub fn on_request<F: Fn(&Request, &mut Response) + Send + Sync + 'static>(
    mut self,
    f: F,
  ) -> Self {
    self.on_request.replace(Box::new(f));
    self
  }

  pub fn build<R: Runtime>(mut self) -> TauriPlugin<R> {
    let port = self.port;
    let on_request = self.on_request.take();

    PluginBuilder::new("localhost")
      .setup(move |app| {
        let asset_resolver = app.asset_resolver();
        std::thread::spawn(move || {
          let server =
            Server::http(&format!("localhost:{}", port)).expect("Unable to spawn server");
          for req in server.incoming_requests() {
            let path = req
              .url()
              .parse::<Uri>()
              .map(|uri| uri.path().into())
              .unwrap_or_else(|_| req.url().into());

            #[allow(unused_mut)]
            if let Some(mut asset) = asset_resolver.get(path) {
              let request = Request {
                url: req.url().into(),
              };
              let mut response = Response {
                headers: Default::default(),
              };

              response.add_header("Content-Type", asset.mime_type);
              if let Some(csp) = asset.csp_header {
                response
                  .headers
                  .insert("Content-Security-Policy".into(), csp);
              }

              if let Some(on_request) = &on_request {
                on_request(&request, &mut response);
              }

              #[cfg(target_os = "linux")]
              if let Some(response_csp) = response.headers.get("Content-Security-Policy") {
                let html = String::from_utf8_lossy(&asset.bytes);
                let body = html.replacen(tauri::utils::html::CSP_TOKEN, response_csp, 1);
                asset.bytes = body.as_bytes().to_vec();
              }

              let mut resp = HttpResponse::from_data(asset.bytes);
              for (header, value) in response.headers {
                if let Ok(h) = Header::from_bytes(header.as_bytes(), value) {
                  resp.add_header(h);
                }
              }
              req.respond(resp).expect("unable to setup response");
            }
          }
        });
        Ok(())
      })
      .build()
  }
}
