use shared::{init_backend_url, init_database_url, init_frontend_url, AppComponent, Url};

#[derive(Debug, Clone)]
pub struct Env {
    frontend_url: Url,
    db_url: Url,
    backend_url: Url,
}

impl Env {
    pub fn init() -> Self {
        Self {
            frontend_url: init_frontend_url(),
            db_url: init_database_url(),
            backend_url: init_backend_url(AppComponent::Backend),
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
}
