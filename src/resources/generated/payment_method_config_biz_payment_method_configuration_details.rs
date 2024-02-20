// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PaymentMethodConfigBizPaymentMethodConfigurationDetails".
#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct PaymentMethodConfigBizPaymentMethodConfigurationDetails {
    /// ID of the parent payment method configuration used.
    pub parent: Option<String>,
}
