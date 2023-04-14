/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`create_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePhoneNumberError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status403(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePhoneNumberError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status403(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPhoneNumberError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status403(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePhoneNumberError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status403(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// Create a new phone number
pub async fn create_phone_number(
	clerk_configuration: &configuration::ClerkConfiguration,
	create_phone_number_request: Option<crate::models::CreatePhoneNumberRequest>,
) -> Result<crate::models::PhoneNumber, Error<CreatePhoneNumberError>> {
	let local_var_configuration = clerk_configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!("{}/phone_numbers", local_var_configuration.base_path);
	let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}

	local_var_req_builder = local_var_req_builder.json(&create_phone_number_request);

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<CreatePhoneNumberError> = serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Delete the phone number with the given ID
pub async fn delete_phone_number(
	clerk_configuration: &configuration::ClerkConfiguration,
	phone_number_id: &str,
) -> Result<crate::models::DeletedObject, Error<DeletePhoneNumberError>> {
	let local_var_configuration = clerk_configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/phone_numbers/{phone_number_id}",
		local_var_configuration.base_path,
		phone_number_id = crate::apis::urlencode(phone_number_id)
	);
	let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<DeletePhoneNumberError> = serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Returns the details of a phone number
pub async fn get_phone_number(
	clerk_configuration: &configuration::ClerkConfiguration,
	phone_number_id: &str,
) -> Result<crate::models::PhoneNumber, Error<GetPhoneNumberError>> {
	let local_var_configuration = clerk_configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/phone_numbers/{phone_number_id}",
		local_var_configuration.base_path,
		phone_number_id = crate::apis::urlencode(phone_number_id)
	);
	let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<GetPhoneNumberError> = serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}

/// Updates a phone number
pub async fn update_phone_number(
	clerk_configuration: &configuration::ClerkConfiguration,
	phone_number_id: &str,
	update_phone_number_request: Option<crate::models::UpdatePhoneNumberRequest>,
) -> Result<crate::models::PhoneNumber, Error<UpdatePhoneNumberError>> {
	let local_var_configuration = clerk_configuration;

	let local_var_client = &local_var_configuration.client;

	let local_var_uri_str = format!(
		"{}/phone_numbers/{phone_number_id}",
		local_var_configuration.base_path,
		phone_number_id = crate::apis::urlencode(phone_number_id)
	);
	let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

	if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
		local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
	}

	local_var_req_builder = local_var_req_builder.json(&update_phone_number_request);

	let local_var_req = local_var_req_builder.build()?;
	let local_var_resp = local_var_client.execute(local_var_req).await?;

	let local_var_status = local_var_resp.status();
	let local_var_content = local_var_resp.text().await?;

	if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
		serde_json::from_str(&local_var_content).map_err(Error::from)
	} else {
		let local_var_entity: Option<UpdatePhoneNumberError> = serde_json::from_str(&local_var_content).ok();
		let local_var_error = ResponseContent {
			status: local_var_status,
			content: local_var_content,
			entity: local_var_entity,
		};
		Err(Error::ResponseError(local_var_error))
	}
}
