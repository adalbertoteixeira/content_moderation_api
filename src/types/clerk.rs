use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClerkUserEmailAddressVerification {
    pub status: Option<String>,
    pub strategy: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClerkUserEmailAddress {
    pub email_address: Option<String>,
    pub id: Option<String>,
    pub linked_to: Vec<Value>,
    pub object: String,
    pub verification: ClerkUserEmailAddressVerification,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClerkUser {
    pub id: Option<String>,
    pub username: Option<String>,
    pub birthday: Option<String>,
    pub email_addresses: Vec<ClerkUserEmailAddress>,
    pub first_name: Option<String>,
    pub gender: Option<String>,
    pub image_url: Option<String>,
    pub last_name: Option<String>,
    // created_at: 1654012591514,
    // external_accounts: [],
    // external_id: '567772',
    // last_sign_in_at: 1654012591514,
    // object: 'user',
    // password_enabled: true,
    // phone_numbers: [],
    // primary_phone_number_id: null,
    // primary_web3_wallet_id: null,
    // private_metadata: {},
    // profile_image_url: 'https://www.gravatar.com/avatar?d=mp',
    // public_metadata: {},
    // two_factor_enabled: false,
    // unsafe_metadata: {},
    // updated_at: 1654012591835,
    // web3_wallets: []
}
