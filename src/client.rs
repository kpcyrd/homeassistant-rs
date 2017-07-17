use tokio_core;
use serde::Serialize;
use serde::de::DeserializeOwned;
use roadrunner::{self, RestClient, RestClientMethods};

use Error;
use structs;

/// The API client
pub struct Client {
    endpoint: String,
    password: Option<String>,
}

impl Client {
    /// Create new api client. `endpoint` is the base url *without* a final slash:
    ///
    /// ```no_run
    /// homeassistant::Client::new("http://192.168.1.2:8123".to_owned(), None);
    /// ```
    pub fn new(endpoint: String, password: Option<String>) -> Client {
        Client {
            endpoint: endpoint,
            password: password,
        }
    }

    fn deserialize<T: DeserializeOwned>(&self, response: roadrunner::Response) -> Result<T, Error> {
        info!("Response: {:?}", response);

        let obj = response.content().as_typed()?;
        Ok(obj)
    }

    fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let mut core = tokio_core::reactor::Core::new().unwrap();

        let uri = format!("{}{}", self.endpoint, url); // TODO

        let response = self.auth(RestClient::get(&uri))
                            .execute_on(&mut core)?;

        self.deserialize(response)
    }

    fn delete<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let mut core = tokio_core::reactor::Core::new().unwrap();

        let uri = format!("{}{}", self.endpoint, url); // TODO

        let response = self.auth(RestClient::delete(&uri))
                            .execute_on(&mut core)?;

        self.deserialize(response)
    }

    fn post<T: Serialize, R: DeserializeOwned>(&self, url: &str, msg: T) -> Result<R, Error> {
        let mut core = tokio_core::reactor::Core::new().unwrap();

        let uri = format!("{}{}", self.endpoint, url); // TODO

        let response = self.auth(RestClient::post(&uri))
                            .json_body_typed(&msg)
                            .execute_on(&mut core)?;

        self.deserialize(response)
    }

    fn auth(&self, client: Result<RestClient, roadrunner::Error>) -> Result<RestClient, roadrunner::Error> {
        match self.password {
            Some(ref password) => {
                client.header_set_raw("x-ha-access", vec!(password.to_owned()))
            },
            None => client,
        }
    }
}

impl Client {
    /// Returns a message if the API is up and running.
    pub fn ping(&self) -> Result<structs::PlainMessage, Error> {
        self.get("/api/")
    }

    /// Returns the current configuration.
    pub fn get_config(&self) -> Result<structs::Config, Error> {
        self.get("/api/config")
    }

    /// Returns basic information about the Home Assistant instance.
    pub fn get_discovery_info(&self) -> Result<structs::DiscoveryInfo, Error> {
        self.get("/api/discovery_info")
    }

    /// Returns an array of event objects. Each event object contains event name and listener count.
    pub fn get_event_listeners(&self) -> Result<Vec<structs::EventListener>, Error> {
        self.get("/api/events")
    }

    /// Returns an array of service objects. Each object contains the domain and which services it contains.
    pub fn get_services(&self) -> Result<Vec<structs::Service>, Error> {
        self.get("/api/services")
    }

    /// Call a service at the remote API.
    pub fn call_service<T: Serialize>(&self, domain: &str, service: &str, service_data: T) -> Result<Vec<structs::State>, Error> {
        self.post(format!("/api/services/{}/{}", domain, service).as_str(), service_data)
    }

    /// Fires an event with event_type.
    ///
    /// You can pass an optional object to be used as `event_data`.
    ///
    /// ```json
    /// {
    ///     "next_rising":"2016-05-31T03:39:14+00:00"
    /// }
    /// ```
    pub fn fire_event<T: Serialize>(&self, event_type: &str, event: T) -> Result<structs::PlainMessage, Error> {
        self.post(format!("/api/events/{}", event_type).as_str(), event)
    }

    /// Query for state of specified entity_id.
    pub fn get_state(&self, entity_id: &str) -> Result<structs::State, Error> {
        self.get(format!("/api/states/{}", entity_id).as_str())
    }

    /// Query for all states.
    pub fn get_states(&self) -> Result<Vec<structs::State>, Error> {
        self.get("/api/states")
    }

    /// Query to see if entity_id is specified state.
    pub fn is_state(&self, entity_id: &str, state: &str) -> Result<bool, Error> {
        let response = self.get_state(entity_id)?;
        Ok(response.state == state)
    }

    pub fn remove_state(&self, entity_id: &str) -> Result<structs::PlainMessage, Error> {
        self.delete(format!("/api/states/{}", entity_id).as_str())
    }

    /// Updates or creates the current state of an entity. (UNIMPLEMENTED!)
    pub fn set_state(&self) -> Result<structs::PlainMessage, Error> {
        // self.get("/api/")
        unimplemented!()
    }

    /// Test if we can communicate with the API.
    pub fn validate_api(&self) -> bool {
        self.ping().is_ok()
    }
}
