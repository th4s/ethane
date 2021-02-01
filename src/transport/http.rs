use super::Credentials;
use crate::transport::{Request, TransportError};
use log::{debug, trace};
use thiserror::Error;
use ureq::{Agent, Error as UreqError, Request as UreqRequest};

pub struct Http {
    agent: Agent,
    domain: String,
    credentials: Option<Credentials>,
}

impl Http {
    pub(crate) fn new(domain: String, credentials: Option<Credentials>) -> Result<Self, HttpError> {
        debug!("Creating http client to {}", domain);
        Ok(Http {
            agent: Agent::new(),
            domain,
            credentials,
        })
    }

    fn prepare_request(&self, method: &str, path: Option<&str>) -> UreqRequest {
        let mut domain = self.domain.clone();
        if let Some(path) = path {
            domain.push_str(path);
        }

        let mut request = self.agent.request(method, &domain);

        if let Some(credentials) = self.credentials.clone() {
            let auth_string_base64 = String::from("Basic ")
                + &base64::encode(credentials.username + ":" + &credentials.password);
            request = request.set("Authorization", &auth_string_base64);
        }
        request
    }
}

impl Request for Http {
    fn request(&mut self, cmd: String) -> Result<String, TransportError> {
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

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum HttpError {
    #[error("HttpError: {0}")]
    Uri(#[from] http::uri::InvalidUri),
    #[error("HttpError: {0}")]
    Conversion(#[from] std::io::Error),
    #[error("HttpError: {0}")]
    UreqError(#[from] UreqError),
}
