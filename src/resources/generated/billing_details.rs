// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::resources::Address;

/// The resource representing a Stripe "billing_details".
#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct BillingDetails {
    /// Billing address.
    pub address: Option<Address>,

    /// Email address.
    pub email: Option<String>,

    /// Full name.
    pub name: Option<String>,

    /// Billing phone number (including extension).
    pub phone: Option<String>,
}
