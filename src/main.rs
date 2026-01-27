use crate::utils::Logger;
use base64::prelude::*;
use dotenvy::dotenv;
use std::{env, sync::Arc, thread};
use tiny_http::{Method, Response, Server};

impl Config {
    fn from_env() -> Self {
        Self {
            port: env::var("MULTIWAKA_PORT")
                .unwrap_or_else(|_| "1234".to_string())
                .parse()
                .expect("Port invalide"),
            url1: env::var("MULTIWAKA_URL_1").expect("MULTIWAKA_URL_1 is missing"),
            key1: env::var("MULTIWAKA_KEY_1").expect("MULTIWAKA_KEY_1 is missing"),
            url2: env::var("MULTIWAKA_URL_2").expect("MULTIWAKA_URL_2 is missing"),
            key2: env::var("MULTIWAKA_KEY_2").expect("MULTIWAKA_KEY_2 manquante"),
            debug: env::var("MULTIWAKA_DEBUG")
                .map(|v| v == "true")
                .unwrap_or(false),
        }
    }
}

fn main() {
    dotenv().ok();
    let config = Arc::new(Config::from_env());
    let server_addr = format!("127.0.0.1:{}", config.port);
    let server = Server::http(&server_addr).unwrap();

    Logger::info(format!("MultiWaka is available at {}", server_addr).as_str());
    if config.debug {
        Logger::highlight("Debug mode enabled");
    }

    for mut request in server.incoming_requests() {
        let config = Arc::clone(&config);

        thread::spawn(move || {
            let path = request.url().to_string();
            let method_enum = request.method().clone();
            let method_str = method_enum.to_string();

            if config.debug {
                Logger::info(format!("New {} request on {}", method_str, path).as_str());
            }

            let mut body = Vec::new();
            if let Err(e) = request.as_reader().read_to_end(&mut body) {
                if config.debug {
                    Logger::error(format!("Failed to read request body: {}", e).as_str());
                }
                return;
            }

            let is_heartbeat = path.contains("heartbeat");

            if is_heartbeat && method_enum == Method::Post {
                if config.debug {
                    Logger::info("Mirroring heartbeat to second instance...");
                }

                let body_clone = body.clone();
                let config_clone = Arc::clone(&config);
                let path_clone = path.clone();
                let method_clone = method_str.clone();

                thread::spawn(move || {
                    match forward_to_instance(
                        &config_clone.url2,
                        &config_clone.key2,
                        &path_clone,
                        &method_clone,
                        &body_clone,
                    ) {
                        Ok((status, _)) => {
                            if config_clone.debug {
                                Logger::info(
                                    format!(
                                        "Heartbeat sent to second instance (Status: {})",
                                        status
                                    )
                                    .as_str(),
                                );
                            }
                        }
                        Err(e) => {
                            if config_clone.debug {
                                Logger::error(
                                    format!("Failed to send heartbeat to second instance: {}", e)
                                        .as_str(),
                                );
                            }
                        }
                    }
                });
            }

            match forward_to_instance(&config.url1, &config.key1, &path, &method_str, &body) {
                Ok((status, resp_body)) => {
                    if config.debug {
                        Logger::info(
                            format!(
                                "Request forwarded to first instance responded with status {}",
                                status
                            )
                            .as_str(),
                        );
                    }
                    let response = Response::from_string(resp_body).with_status_code(status);
                    let _ = request.respond(response);
                }
                Err(e) => {
                    if config.debug {
                        Logger::error(
                            format!("Failed to forward request to first instance: {}", e).as_str(),
                        );
                    }
                    let response =
                        Response::from_string(format!("Error: {}", e)).with_status_code(500);
                    let _ = request.respond(response);
                }
            }
        });
    }
}

fn forward_to_instance(
    base_url: &str,
    api_key: &str,
    path: &str,
    method: &str,
    body: &[u8],
) -> Result<(u16, String), String> {
    let target_url = format!("{}{}", base_url.trim_end_matches('/'), path);

    let agent = ureq::Agent::new();
    let request = agent
        .request(method, &target_url)
        .set(
            "Authorization",
            format!("Basic {}", BASE64_STANDARD.encode(api_key)).as_str(),
        )
        .set("Content-Type", "application/json");

    let response_result = if method == "GET" || body.is_empty() {
        request.call()
    } else {
        request.send_bytes(body)
    };

    match response_result {
        Ok(response) => {
            let status = response.status();
            let body_text = response.into_string().unwrap_or_default();
            Ok((status, body_text))
        }
        Err(ureq::Error::Status(code, response)) => {
            let body_text = response.into_string().unwrap_or_default();
            Ok((code, body_text))
        }
        Err(e) => Err(e.to_string()),
    }
}

struct Config {
    port: u16,
    url1: String,
    key1: String,
    url2: String,
    key2: String,
    debug: bool,
}

mod utils;
