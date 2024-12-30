use cookie_store::{CookieDomain, CookieStore};
use reqwest::{Client, ClientBuilder};
use reqwest_cookie_store::CookieStoreMutex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize)]
struct ServiceSession {
    cookies: Vec<CookieData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CookieData {
    domain: String,
    name: String,
    value: String,
    path: String,
    secure: bool,
    http_only: bool,
    expiry: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SessionStore {
    services: HashMap<String, ServiceSession>,
}

pub struct SessionManager {
    session_file: PathBuf,
    cookie_store: Arc<CookieStoreMutex>,
}

impl SessionManager {
    pub fn new() -> Self {
        let session_file = Self::default_session_path();
        let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::default()));
        SessionManager {
            session_file,
            cookie_store,
        }
    }

    fn default_session_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("online-judge-toolchain");
        path.push("session.json");
        path
    }

    pub fn create_client(&self) -> reqwest::Result<Client> {
        ClientBuilder::new()
            .cookie_provider(Arc::clone(&self.cookie_store))
            .build()
    }

    pub fn save_service_session(
        &self,
        service_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cookies = {
            let store = self.cookie_store.lock().unwrap();
            store
                .iter_any()
                .map(|cookie| CookieData {
                    domain: match &cookie.domain {
                        CookieDomain::HostOnly(domain) | CookieDomain::Suffix(domain) => {
                            domain.clone()
                        }
                        _ => "".to_string(),
                    },
                    name: cookie.name().to_string(),
                    value: cookie.value().to_string(),
                    path: cookie.path().unwrap_or("").to_string(),
                    secure: cookie.secure().unwrap_or(false),
                    http_only: cookie.http_only().unwrap_or(false),
                    expiry: cookie.expires().and_then(|expires| {
                        expires.datetime().map(|dt| dt.unix_timestamp() as u64)
                    }),
                })
                .collect::<Vec<_>>()
        };

        let mut store = SessionStore {
            services: HashMap::new(),
        };

        store
            .services
            .insert(service_name.to_string(), ServiceSession { cookies });

        std::fs::create_dir_all(self.session_file.parent().unwrap())?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.session_file)?;

        file.write_all(serde_json::to_string_pretty(&store)?.as_bytes())?;

        println!("Saved session for service: {}", service_name);

        Ok(())
    }
}
