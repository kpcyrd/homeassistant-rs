use tokio_core;
use serde::Serialize;
use serde::de::DeserializeOwned;
use roadrunner::{self, RestClient, RestClientMethods};

use Error;
use structs;

pub struct Client {
    endpoint: String,
    password: Option<String>,
}

impl Client {
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
    pub fn ping(&self) -> Result<structs::PlainMessage, Error> {
        self.get("/api/")
    }

    pub fn get_config(&self) -> Result<structs::Config, Error> {
        self.get("/api/config")
    }

    pub fn get_discovery_info(&self) -> Result<structs::DiscoveryInfo, Error> {
        self.get("/api/discovery_info")
    }

    pub fn get_event_listeners(&self) -> Result<Vec<structs::EventListener>, Error> {
        self.get("/api/events")
    }

    pub fn get_services(&self) -> Result<Vec<structs::Service>, Error> {
        self.get("/api/services")
    }

    pub fn call_service<T: Serialize>(&self, domain: &str, service: &str, service_data: T) -> Result<Vec<structs::State>, Error> {
        self.post(format!("/api/services/{}/{}", domain, service).as_str(), service_data)
    }

    pub fn fire_event<T: Serialize>(&self, event_type: &str, event: T) -> Result<structs::PlainMessage, Error> {
        self.post(format!("/api/events/{}", event_type).as_str(), event)
    }

    pub fn get_state(&self, entity_id: &str) -> Result<structs::State, Error> {
        self.get(format!("/api/states/{}", entity_id).as_str())
    }

    pub fn get_states(&self) -> Result<Vec<structs::State>, Error> {
        self.get("/api/states")
    }

    pub fn is_state(&self, entity_id: &str, state: &str) -> Result<bool, Error> {
        let response = self.get_state(entity_id)?;
        Ok(response.state == state)
    }

    pub fn remove_state(&self, entity_id: &str) -> Result<structs::PlainMessage, Error> {
        self.delete(format!("/api/states/{}", entity_id).as_str())
    }

    pub fn set_state(&self) -> Result<structs::PlainMessage, Error> {
        // self.get("/api/")
        unimplemented!()
    }

    pub fn validate_api(&self) -> bool {
        self.ping().is_ok()
    }
}
