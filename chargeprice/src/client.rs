use super::api::{ErrorResponse, VehiculeResponse};
use header::InvalidHeaderValue;
use log::debug;
use reqwest::{header, Client, Method, Request, Url};
use serde::de::DeserializeOwned;
use thiserror::Error;

const USER_AGENT: &'static str = concat!("chargeprice-rs ", env!("CARGO_PKG_VERSION"));
const BASE_HOST: &'static str = "https://api.chargeprice.app";
const API_KEY_HEADER: &'static str = "api-key";

#[derive(Debug)]
pub struct APIClient {
    client: Client,
}

#[derive(Debug, Error)]
pub enum APIError {
    #[error("http request failed")]
    Network(#[from] reqwest::Error),
    #[error("invalid api key")]
    APIAuthenticationError(#[from] InvalidHeaderValue),
    #[error("response error: response {response:?}")]
    APIClientError { response: ErrorResponse },
    #[error("unknown response: response {response:?}")]
    APIUnknownResponse { response: reqwest::Response },
}

impl APIClient {
    /// Creates a new client with a specific API key.
    pub fn new(api_key: &str) -> Result<Self, APIError> {
        APIClient::new_generic(api_key, None)
    }

    /// Creates a new client with a specific API key and adds a
    /// flavor to the user agent
    pub fn new_with_agent_flavor(api_key: &str, flavor: &str) -> Result<Self, APIError> {
        APIClient::new_generic(api_key, Some(flavor))
    }

    fn new_generic(api_key: &str, user_agent_flavor: Option<&str>) -> Result<Self, APIError> {
        // We use the provided api key as default header
        let mut headers = header::HeaderMap::new();
        let key_name = header::HeaderName::from_static(API_KEY_HEADER);
        let key_value = header::HeaderValue::from_str(api_key)?;
        headers.insert(key_name, key_value);

        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let mut client = Client::builder();

        if let Some(flavor) = user_agent_flavor {
            let agent = format!("{} - {}", USER_AGENT, flavor);
            client = client.user_agent(agent);
        } else {
            client = client.user_agent(USER_AGENT);
        }

        let client = client.default_headers(headers).build()?;

        Ok(APIClient { client })
    }

    async fn execute_request<T: DeserializeOwned>(&self, request: Request) -> Result<T, APIError> {
        debug!("starting request {:?}", request);
        let response = self.client.execute(request).await?;

        let status = response.status();

        if status.is_success() {
            let element = response.json().await?;
            Ok(element)
        } else if status.is_client_error() {
            let element: ErrorResponse = response.json().await?;
            Err(APIError::APIClientError { response: element })
        } else {
            Err(APIError::APIUnknownResponse { response })
        }
    }

    fn get_vehicules_request() -> Request {
        let url = Url::parse(&format!("{}/v1/vehicles", BASE_HOST)).unwrap();
        Request::new(Method::GET, url)
    }

    /// Loads the vehicules
    pub async fn load_vehicules(&self) -> Result<VehiculeResponse, APIError> {
        self.execute_request(APIClient::get_vehicules_request())
            .await
    }
}
