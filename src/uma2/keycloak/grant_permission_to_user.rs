use serde::{Deserialize, Serialize};
use crate::{Client, Provider, Claims, OAuth2Error};
use crate::uma2::Uma2Provider;
use biscuit::CompactJson;
use crate::error::ClientError;
use crate::uma2::error::Uma2Error::{NoUma2Discovered, NoPermissionsEndpoint, PermissionEndpointMalformed};
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION, ACCEPT};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Uma2GrantPermissionToUserRequest {
    pub resource: String,
    pub requester: String,
    pub granted: bool,
    #[serde(rename = "scopeName")]
    pub scope_name: Option<String>
}

impl<P, C> Client<P, C>
    where
        P: Provider + Uma2Provider,
        C: CompactJson + Claims,
{
    pub async fn grant_permission_to_user(
        &self,
        token: String,
        resource_id: String,
        requester: String,
        scope_name: Option<String>
    ) -> Result<String, ClientError> {
        if !self.provider.uma2_discovered() {
            return Err(ClientError::Uma2(NoUma2Discovered));
        }

        if self.provider.permission_uri().is_none() {
            return Err(ClientError::Uma2(NoPermissionsEndpoint));
        }
        let mut url = self.provider.permission_uri().unwrap().clone();

        url.path_segments_mut()
            .map_err(|_| ClientError::Uma2(PermissionEndpointMalformed))?
            .extend(&["ticket"]);

        let request = Uma2GrantPermissionToUserRequest {
            resource: resource_id,
            requester,
            scope_name,
            granted: true
        };


        let json = self
            .http_client
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {:}", token))
            .header(ACCEPT, "application/json")
            .json(&request)
            .send()
            .await?
            .json::<Value>()
            .await?;

        let error: Result<OAuth2Error, _> = serde_json::from_value(json.clone());

        if let Ok(error) = error {
            Err(ClientError::from(error))
        } else {
            // TODO inspect this to get proper return value
            let ret = format!("{:}", json);
            Ok(ret)
        }
    }
}
