// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::params::Timestamp;

/// The resource representing a Stripe "PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode".
#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
    /// The URL to the hosted Cash App Pay instructions page, which allows customers to view the QR code, and supports QR code refreshing on expiration.
    pub hosted_instructions_url: String,

    /// The url for mobile redirect based auth.
    pub mobile_auth_url: String,

    pub qr_code: PaymentIntentNextActionCashappQrCode,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct PaymentIntentNextActionCashappQrCode {
    /// The date (unix timestamp) when the QR code expires.
    pub expires_at: Timestamp,

    /// The image_url_png string used to render QR code.
    pub image_url_png: String,

    /// The image_url_svg string used to render QR code.
    pub image_url_svg: String,
}
