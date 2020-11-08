//! Assorted utility functions (missing batteries).

use std::time;

use serde::de::DeserializeOwned;
use serenity::async_trait;
use url::Url;

/// Shortcut for defining a lazily-compiled regular expression
macro_rules! _regex {
    ($regex_body:literal) => {{
        static RE: ::once_cell::sync::OnceCell<regex::Regex> = ::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| ::regex::Regex::new($regex_body).unwrap())
    }};
}

macro_rules! _def_url_base {
    ($ident:ident, $url:literal) => {
        fn $ident<T: AsRef<str>>(segments: impl IntoIterator<Item = T>) -> ::url::Url {
            let mut url: ::url::Url = $url.parse().unwrap();
            url.path_segments_mut().unwrap().extend(segments);
            url
        }
    };
}

pub(crate) use {_def_url_base as def_url_base, _regex as regex};

#[async_trait]
pub(crate) trait ReqwestClientExt {
    async fn send_get_json_request<T: DeserializeOwned>(
        &self,
        url: Url,
        query: &[(&str, &str)],
    ) -> crate::Result<T>;
}

#[async_trait]
impl ReqwestClientExt for reqwest::Client {
    async fn send_get_json_request<T: DeserializeOwned>(
        &self,
        url: Url,
        query: &[(&str, &str)],
    ) -> crate::Result<T> {
        let res = self
            .get(url)
            .query(query)
            .header("User-Agent", "Veebot")
            .send()
            .await
            .map_err(|err| crate::err!(SendRequest(err)))?;

        let status = res.status();

        if status.is_client_error() || status.is_server_error() {
            let body = match res.text().await {
                Ok(it) => it,
                Err(err) => format!("Could not collect the GET request body: {}", err),
            };

            return Err(crate::err!(GetRequest { status, body }));
        }

        res.json()
            .await
            .map_err(|err| crate::err!(UnexpectedJsonShape(err)))
    }
}

pub(crate) fn create_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(time::Duration::from_secs(30))
        .connect_timeout(time::Duration::from_secs(30))
        .build()
        .expect("rustls backend initialization should never error out")
}
