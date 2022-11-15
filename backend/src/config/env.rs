use shared::{
    init_database_url, Url, BACKEND_HOST_KEY, BACKEND_PORT_KEY, BACKEND_PROTOCOL_KEY,
    DEFAULT_BACKEND_URL, DEFAULT_FRONTEND_URL, FRONTEND_HOST_KEY, FRONTEND_PORT_KEY,
    FRONTEND_PROTOCOL_KEY,
};

#[derive(Debug, Clone)]
pub struct Env {
    frontend_url: Url,
    db_url: Url,
    backend_url: Url,
}

impl Env {
    pub fn init() -> Self {
        Self {
            frontend_url: Self::init_frontend_url(),
            db_url: init_database_url(),
            backend_url: Self::init_backend_url(),
        }
    }

    pub fn get_frontend_url(&self) -> &Url {
        &self.frontend_url
    }

    pub fn get_db_url(&self) -> &Url {
        &self.db_url
    }

    pub fn get_backend_url(&self) -> &Url {
        &self.backend_url
    }

    fn init_frontend_url() -> Url {
        let mut url =
            Url::parse(DEFAULT_FRONTEND_URL).expect("the default frontend url to be parseable");

        if let Ok(protocol) = std::env::var(FRONTEND_PROTOCOL_KEY) {
            url.set_scheme(&protocol).unwrap();
        }

        if let Ok(host) = std::env::var(FRONTEND_HOST_KEY) {
            url.set_host(Some(&host)).unwrap();
        }

        if let Ok(port) = std::env::var(FRONTEND_PORT_KEY) {
            let port = port
                .parse::<u16>()
                .expect("the frontend port to parseable to u16");
            url.set_port(Some(port)).unwrap();
        };

        url
    }

    fn init_backend_url() -> Url {
        let mut url =
            Url::parse(DEFAULT_BACKEND_URL).expect("the default backend url to be parseable");

        if let Ok(protocol) = std::env::var(BACKEND_PROTOCOL_KEY) {
            url.set_scheme(&protocol).unwrap();
        }

        if let Ok(host) = std::env::var(BACKEND_HOST_KEY) {
            url.set_host(Some(&host)).unwrap();
        }

        if let Ok(port) = std::env::var(BACKEND_PORT_KEY) {
            let port = port
                .parse::<u16>()
                .expect("the frontend port to parseable to u16");
            url.set_port(Some(port)).unwrap();
        };

        url
    }
}
