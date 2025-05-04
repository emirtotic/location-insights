use crate::clients::geo::GeoApiClient;

#[derive(Clone)]
pub struct ApiClients {
    pub geo: GeoApiClient,
}
