//! Webdav client
//!
//! Create a client
//! ```ignore
//! let client = Client::init("username", "password");
//! ```
//! Now you can use the client to call any of the methods listed in the **Client** Struct.
//!
//! All the paths used by the methods should be absolute on the webdav server to the required file, folder, zip.
//!
//! Every method will return a Result<Response, Error>
//! ```rust
//! # let result: Result<&str, String> = Ok("test");
//! if result.is_ok() {
//!    // the method completed with success
//! } else {
//!    // somenting when wrong
//! }
//! ```

use super::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Client {
    username: String,
    password: String,
    client: reqwest::Client,
}

impl Client {
    /// Initialization of the client
    ///
    /// Initialized client will be stored for future requests
    pub fn init(username: &str, password: &str) -> Self {
        Client {
            username: username.to_owned(),
            password: password.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    fn custom_header(&self, name: &str, value: &str) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::HeaderName::from_bytes(name.as_bytes()).unwrap(),
            header::HeaderValue::from_bytes(value.as_bytes()).unwrap(),
        );
        headers
    }

    fn form_params(&self, key: &'static str, value: &'static str) -> HashMap<&str, &str> {
        let mut params = HashMap::new();
        params.insert(key, value);

        params
    }

    /// Main function that creates the RequestBuilder, sets the method, url and the basic_auth
    fn start_request(&self, method: Method, path: &str) -> RequestBuilder {
        self.client
            .request(method, Url::parse(path).unwrap())
            .basic_auth(self.username.as_str(), Some(self.password.as_str()))
    }

    /// Get a file from Webdav server
    ///
    /// Use absolute path to the webdav server file location
    pub async fn get(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::GET, path).send().await
    }

    /// Upload a file/zip on Webdav server
    ///
    /// It can be any type of file as long as it is transformed to a vector of bytes (Vec<u8>).
    /// This can be achieved with **std::fs::File** or **zip-rs** for sending zip files.
    ///
    /// Use absolute path to the webdav server folder location
    pub async fn put<B: Into<Body>>(&self, body: B, path: &str) -> Result<Response, Error> {
        self.start_request(Method::PUT, path)
            .headers(self.custom_header("content-type", "application/octet-stream"))
            .body(body)
            .send()
            .await
    }

    /// Deletes the collection, file, folder or zip archive at the given path on Webdav server
    ///
    /// Use absolute path to the webdav server file location
    pub async fn delete(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::DELETE, path).send().await
    }

    /// Unzips the .zip archieve on Webdav server
    ///
    /// Use absolute path to the webdav server file location
    pub async fn unzip(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::POST, path)
            .form(&self.form_params("method", "UNZIP"))
            .send()
            .await
    }

    /// Creates a directory on Webdav server
    ///
    /// Use absolute path to the webdav server file location
    pub async fn mkcol(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::from_bytes(b"MKCOL").unwrap(), path)
            .send()
            .await
    }

    /// Rename or move a collection, file, folder on Webdav server
    ///
    /// If the file location changes it will move the file, if only the file name changes it will rename it.
    ///
    /// Use absolute path to the webdav server file location
    pub async fn mv(&self, from: &str, to: &str) -> Result<Response, Error> {
        self.start_request(Method::from_bytes(b"MOVE").unwrap(), from)
            .headers(self.custom_header("destination", to))
            .send()
            .await
    }

    /// List files and folders at the given path on Webdav server
    ///
    /// Depth of "0" applies only to the resource, "1" to the resource and it's children, "infinity" to the resource and all it's children recursively
    /// The result will contain an xml list with the remote folder contents.
    ///
    /// Use absolute path to the webdav server folder location
    pub async fn list(&self, path: &str, depth: &str) -> Result<Response, Error> {
        let body = r#"<?xml version="1.0" encoding="utf-8" ?>
            <D:propfind xmlns:D="DAV:">
                <D:allprop/>
            </D:propfind>
        "#;

        self.start_request(Method::from_bytes(b"PROPFIND").unwrap(), path)
            .headers(self.custom_header("depth", depth))
            .body(body)
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SERVER_URL: &str = "https://www.webdavserver.com";
    const USER_FOLDER: &str = "User287e257";

    fn get_server_path(path: &str) -> String {
        format!("{0}/{1}/{2}", SERVER_URL, USER_FOLDER, path)
    }

    fn get_client() -> Client {
        Client::init("", "")
    }

    #[tokio::test]
    async fn test_1_mkcol() {
        let webdav_client = get_client();
        let result = webdav_client
            .mkcol(get_server_path("rustydav").as_str())
            .await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_2_put() {
        let webdav_client = get_client();
        let result = webdav_client
            .put(
                "rustydav is a cool small library",
                get_server_path("rustydav/test.txt").as_str(),
            )
            .await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_3_get() {
        let webdav_client = get_client();
        let result = webdav_client
            .get(get_server_path("rustydav/test.txt").as_str())
            .await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_4_mv() {
        let webdav_client = get_client();
        let result = webdav_client
            .mv(
                get_server_path("rustydav/test.txt").as_str(),
                get_server_path("test.txt").as_str(),
            )
            .await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_5_delete() {
        let webdav_client = get_client();
        let result = webdav_client
            .delete(get_server_path("test.txt").as_str())
            .await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_6_unzip() {
        let webdav_client = get_client();
        let result = webdav_client
            .unzip(get_server_path("test.zip").as_str())
            .await;

        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_7_list() {
        let webdav_client = get_client();
        let result = webdav_client.list(get_server_path("").as_str(), "0").await;

        assert_eq!(result.is_ok(), true);
    }
}
