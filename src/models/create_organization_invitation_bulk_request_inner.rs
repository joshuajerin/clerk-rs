/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrganizationInvitationBulkRequestInner {
	/// The email address of the new member that is going to be invited to the organization
	#[serde(rename = "email_address")]
	pub email_address: String,
	/// The ID of the user that invites the new member to the organization. Must be an administrator in the organization.
	#[serde(rename = "inviter_user_id")]
	pub inviter_user_id: String,
	/// The role of the new member in the organization.
	#[serde(rename = "role")]
	pub role: String,
	/// Metadata saved on the organization invitation, read-only from the Frontend API and fully accessible (read/write) from the Backend API.
	#[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
	pub public_metadata: Option<serde_json::Value>,
	/// Metadata saved on the organization invitation, fully accessible (read/write) from the Backend API but not visible from the Frontend API.
	#[serde(rename = "private_metadata", skip_serializing_if = "Option::is_none")]
	pub private_metadata: Option<serde_json::Value>,
	/// Optional URL that the invitee will be redirected to once they accept the invitation by clicking the join link in the invitation email.
	#[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
	pub redirect_url: Option<String>,
}

impl CreateOrganizationInvitationBulkRequestInner {
	pub fn new(email_address: String, inviter_user_id: String, role: String) -> CreateOrganizationInvitationBulkRequestInner {
		CreateOrganizationInvitationBulkRequestInner {
			email_address,
			inviter_user_id,
			role,
			public_metadata: None,
			private_metadata: None,
			redirect_url: None,
		}
	}
}
