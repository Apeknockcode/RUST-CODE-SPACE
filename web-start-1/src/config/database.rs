use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct DataBaseConfig {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
    schema: Option<String>,
}

impl DataBaseConfig {
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3306)
    }
    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("root")
    }
    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("")
    }
    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("rust_code_space")
    }
    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("public")
    }
}
