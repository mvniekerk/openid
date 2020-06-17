use crate::{CustomClaims, StandardClaims};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct RequestingPartyTokenAuthorizationPermission {
    pub resource_set_id: String,
    pub resource_set_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct RequestingPartyTokenAuthorization {
    pub permissions: Vec<RequestingPartyTokenAuthorizationPermission>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct RequestingPartyToken {
    pub authorization: RequestingPartyTokenAuthorization,

    #[serde(flatten)]
    pub standard_claims: StandardClaims,
}

impl CustomClaims for RequestingPartyToken {
    fn standard_claims(&self) -> &StandardClaims {
        &self.standard_claims
    }
}
