/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;

use std::option::Option;

use reqwest;

use super::{Error, configuration};


/// struct for typed errors of method `add_pet`
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddPetErrors {
    // TODO Generate an enum case for each error described in schema.
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method `delete_pet`
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePetErrors {
    // TODO Generate an enum case for each error described in schema.
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method `find_pets_by_status`
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindPetsByStatusErrors {
    // TODO Generate an enum case for each error described in schema.
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method `find_pets_by_tags`
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindPetsByTagsErrors {
    // TODO Generate an enum case for each error described in schema.
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method `get_pet_by_id`
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPetByIdErrors {
    // TODO Generate an enum case for each error described in schema.
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method `update_pet`
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePetErrors {
    // TODO Generate an enum case for each error described in schema.
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method `update_pet_with_form`
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePetWithFormErrors {
    // TODO Generate an enum case for each error described in schema.
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method `upload_file`
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadFileErrors {
    // TODO Generate an enum case for each error described in schema.
    UnknownList(Vec<serde_json::Value>),
    UnknownValue(serde_json::Value),
}

    pub async fn add_pet(configuration: &configuration::Configuration, body: crate::models::Pet) -> Result<(), Error<AddPetErrors>> {
        let client = &configuration.client;

        let uri_str = format!("{}/pet", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let content = resp.text().await?;
            let entity: Option<AddPetErrors> = serde_json::from_str(&content).ok();
            let error = crate::apis::ResponseErrorContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn delete_pet(configuration: &configuration::Configuration, pet_id: i64, api_key: Option<&str>) -> Result<(), Error<DeletePetErrors>> {
        let client = &configuration.client;

        let uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(param_value) = api_key {
            req_builder = req_builder.header("api_key", param_value.to_string());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let content = resp.text().await?;
            let entity: Option<DeletePetErrors> = serde_json::from_str(&content).ok();
            let error = crate::apis::ResponseErrorContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn find_pets_by_status(configuration: &configuration::Configuration, status: Vec<String>) -> Result<Vec<crate::models::Pet>, Error<FindPetsByStatusErrors>> {
        let client = &configuration.client;

        let uri_str = format!("{}/pet/findByStatus", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("status", &status.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;
        if resp.status().is_success() {
            Ok(resp.json::<Vec<crate::models::Pet>>().await?)
        } else {
            let status = resp.status();
            let content = resp.text().await?;
            let entity: Option<FindPetsByStatusErrors> = serde_json::from_str(&content).ok();
            let error = crate::apis::ResponseErrorContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn find_pets_by_tags(configuration: &configuration::Configuration, tags: Vec<String>) -> Result<Vec<crate::models::Pet>, Error<FindPetsByTagsErrors>> {
        let client = &configuration.client;

        let uri_str = format!("{}/pet/findByTags", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("tags", &tags.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;
        if resp.status().is_success() {
            Ok(resp.json::<Vec<crate::models::Pet>>().await?)
        } else {
            let status = resp.status();
            let content = resp.text().await?;
            let entity: Option<FindPetsByTagsErrors> = serde_json::from_str(&content).ok();
            let error = crate::apis::ResponseErrorContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn get_pet_by_id(configuration: &configuration::Configuration, pet_id: i64) -> Result<crate::models::Pet, Error<GetPetByIdErrors>> {
        let client = &configuration.client;

        let uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("api_key", val);
        };

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;
        if resp.status().is_success() {
            Ok(resp.json::<crate::models::Pet>().await?)
        } else {
            let status = resp.status();
            let content = resp.text().await?;
            let entity: Option<GetPetByIdErrors> = serde_json::from_str(&content).ok();
            let error = crate::apis::ResponseErrorContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn update_pet(configuration: &configuration::Configuration, body: crate::models::Pet) -> Result<(), Error<UpdatePetErrors>> {
        let client = &configuration.client;

        let uri_str = format!("{}/pet", configuration.base_path);
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let content = resp.text().await?;
            let entity: Option<UpdatePetErrors> = serde_json::from_str(&content).ok();
            let error = crate::apis::ResponseErrorContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn update_pet_with_form(configuration: &configuration::Configuration, pet_id: i64, name: Option<&str>, status: Option<&str>) -> Result<(), Error<UpdatePetWithFormErrors>> {
        let client = &configuration.client;

        let uri_str = format!("{}/pet/{petId}", configuration.base_path, petId=pet_id);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form_params = std::collections::HashMap::new();
        if let Some(param_value) = name {
            form_params.insert("name", param_value.to_string());
        }
        if let Some(param_value) = status {
            form_params.insert("status", param_value.to_string());
        }
        req_builder = req_builder.form(&form_params);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let content = resp.text().await?;
            let entity: Option<UpdatePetWithFormErrors> = serde_json::from_str(&content).ok();
            let error = crate::apis::ResponseErrorContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

    pub async fn upload_file(configuration: &configuration::Configuration, pet_id: i64, additional_metadata: Option<&str>, file: Option<std::path::PathBuf>) -> Result<crate::models::ApiResponse, Error<UploadFileErrors>> {
        let client = &configuration.client;

        let uri_str = format!("{}/pet/{petId}/uploadImage", configuration.base_path, petId=pet_id);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        let mut form = reqwest::multipart::Form::new();
        if let Some(param_value) = additional_metadata {
            form = form.text("additionalMetadata", param_value.to_string());
        }
        // TODO: support file upload for 'file' parameter
        req_builder = req_builder.multipart(form);

        let req = req_builder.build()?;
        let resp = client.execute(req).await?;
        if resp.status().is_success() {
            Ok(resp.json::<crate::models::ApiResponse>().await?)
        } else {
            let status = resp.status();
            let content = resp.text().await?;
            let entity: Option<UploadFileErrors> = serde_json::from_str(&content).ok();
            let error = crate::apis::ResponseErrorContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

