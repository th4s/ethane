use super::Credentials;
use crate::transport::{JsonRequest, TransportError};
use http::Uri;
use log::{debug, trace};
use std::str::FromStr;
use thiserror::Error;
use ureq::{Agent, Error as UreqError, Request};

pub struct Http {
    agent: Agent,
    uri: Uri,
    credentials: Option<Credentials>,
}

impl Http {
    pub(crate) fn new(domain: &str, credentials: Option<Credentials>) -> Result<Self, HttpError> {
        debug!("Creating http client to {}", domain);
        Ok(Http {
            agent: Agent::new(),
            uri: Uri::from_str(domain)?,
            credentials,
        })
    }

    fn prepare_request(&self, method: &str, path: Option<&str>) -> Request {
        let mut uri = self.uri.to_string();
        if let Some(path) = path {
            uri.push_str(path);
        }

        let mut request = self.agent.request(method, &uri);

        if let Some(credentials) = self.credentials.clone() {
            let auth_string_base64 = String::from("Basic ")
                + &base64::encode(credentials.username + ":" + &credentials.password);
            request = request.set("Authorization", &auth_string_base64);
        }
        request
    }
}

impl JsonRequest for Http {
    fn json_request(&mut self, cmd: String) -> Result<String, TransportError> {
        let mut request = self.prepare_request("POST", None);
        request = request.set("Content-Type", "application/json");
        trace!("Sending request {:?} with body {}", &request, &cmd);
        let response = request.send_string(&cmd).map_err(HttpError::from)?;
        response
            .into_string()
            .map(|resp| {
                trace!("Received http response: {}", &resp);
                resp
            })
            .map_err(|err| HttpError::from(err).into())
    }
}

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("HttpError: {0}")]
    Url(#[from] http::uri::InvalidUri),
    #[error("HttpError: {0}")]
    Conversion(#[from] std::io::Error),
    #[error("HttpError: {0}")]
    UreqError(#[from] UreqError),
}
