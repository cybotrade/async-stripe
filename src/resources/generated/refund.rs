// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{ChargeId, CustomerId, PaymentIntentId, RefundId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{BalanceTransaction, Charge, Currency, PaymentIntent, TransferReversal};

/// The resource representing a Stripe "Refund".
///
/// For more details see <https://stripe.com/docs/api/refunds/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct Refund {
    /// Unique identifier for the object.
    pub id: RefundId,

    /// Amount, in cents (or local equivalent).
    pub amount: i64,

    /// Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// ID of the charge that's refunded.
    pub charge: Option<Expandable<Charge>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// You can use this for displaying to users (available on non-card refunds only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_details: Option<RefundDestinationDetails>,

    /// After the refund fails, this balance transaction describes the adjustment made on your account balance that reverses the initial balance transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Provides the reason for the refund failure.
    ///
    /// Possible values are: `lost_or_stolen_card`, `expired_or_canceled_card`, `charge_for_pending_refund_disputed`, `insufficient_funds`, `declined`, `merchant_request`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,

    /// For payment methods without native refund support (for example, Konbini, PromptPay), provide an email address for the customer to receive refund instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_email: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<RefundNextAction>,

    /// ID of the PaymentIntent that's refunded.
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// Reason for the refund, which is either user-provided (`duplicate`, `fraudulent`, or `requested_by_customer`) or generated by Stripe internally (`expired_uncaptured_charge`).
    pub reason: Option<RefundReason>,

    /// This is the transaction number that appears on email receipts sent for this refund.
    pub receipt_number: Option<String>,

    /// The transfer reversal that's associated with the refund.
    ///
    /// Only present if the charge came from another Stripe account.
    pub source_transfer_reversal: Option<Expandable<TransferReversal>>,

    /// Status of the refund.
    ///
    /// This can be `pending`, `requires_action`, `succeeded`, `failed`, or `canceled`.
    /// Learn more about [failed refunds](https://stripe.com/docs/refunds#failed-refunds).
    pub status: Option<String>,

    /// This refers to the transfer reversal object if the accompanying transfer reverses.
    ///
    /// This is only applicable if the charge was created using the destination parameter.
    pub transfer_reversal: Option<Expandable<TransferReversal>>,
}

impl Refund {
    /// Returns a list of all refunds you created.
    ///
    /// We return the refunds in sorted order, with the most recent refunds appearing first The 10 most recent refunds are always available by default on the Charge object.
    pub fn list(client: &Client, params: &ListRefunds<'_>) -> Response<List<Refund>> {
        client.get_query("/refunds", &params)
    }

    /// When you create a new refund, you must specify a Charge or a PaymentIntent object on which to create it.
    ///
    /// Creating a new refund will refund a charge that has previously been created but not yet refunded.
    /// Funds will be refunded to the credit or debit card that was originally charged.
    ///
    /// You can optionally refund only part of a charge.
    /// You can do so multiple times, until the entire charge has been refunded.
    ///
    /// Once entirely refunded, a charge can’t be refunded again.
    /// This method will raise an error when called on an already-refunded charge,
    /// or when trying to refund more money than is left on a charge.
    pub fn create(client: &Client, params: CreateRefund<'_>) -> Response<Refund> {
        client.post_form("/refunds", &params)
    }

    /// Retrieves the details of an existing refund.
    pub fn retrieve(client: &Client, id: &RefundId, expand: &[&str]) -> Response<Refund> {
        client.get_query(&format!("/refunds/{}", id), &Expand { expand })
    }

    /// Updates the refund that you specify by setting the values of the passed parameters.
    ///
    /// Any parameters that you don’t provide remain unchanged.  This request only accepts `metadata` as an argument.
    pub fn update(client: &Client, id: &RefundId, params: UpdateRefund<'_>) -> Response<Refund> {
        client.post_form(&format!("/refunds/{}", id), &params)
    }
}

impl Object for Refund {
    type Id = RefundId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "refund"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct RefundDestinationDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_bank_transfer: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<RefundDestinationDetailsGeneric>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub br_bank_transfer: Option<RefundDestinationDetailsGeneric>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<RefundDestinationDetailsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_cash_balance: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<RefundDestinationDetailsGeneric>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer: Option<RefundDestinationDetailsGeneric>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer: Option<RefundDestinationDetailsGeneric>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer: Option<RefundDestinationDetailsGeneric>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<RefundDestinationDetailsGeneric>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<RefundDestinationDetailsGeneric>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub th_bank_transfer: Option<RefundDestinationDetailsGeneric>,

    /// The type of transaction-specific details of the payment method used in the refund (e.g., `card`).
    ///
    /// An additional hash is included on `destination_details` with a name matching this value.
    /// It contains information specific to the refund transaction.
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer: Option<RefundDestinationDetailsGeneric>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<DestinationDetailsUnimplemented>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<DestinationDetailsUnimplemented>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct DestinationDetailsUnimplemented {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct RefundDestinationDetailsCard {
    /// Value of the reference number assigned to the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Status of the reference number on the refund.
    ///
    /// This can be `pending`, `available` or `unavailable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_status: Option<String>,

    /// Type of the reference number assigned to the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,

    /// The type of refund.
    ///
    /// This can be `refund`, `reversal`, or `pending`.
    #[serde(rename = "type")]
    pub type_: RefundDestinationDetailsCardType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct RefundDestinationDetailsGeneric {
    /// The reference assigned to the refund.
    pub reference: Option<String>,

    /// Status of the reference on the refund.
    ///
    /// This can be `pending`, `available` or `unavailable`.
    pub reference_status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct RefundNextAction {
    /// Contains the refund details.
    pub display_details: Option<RefundNextActionDisplayDetails>,

    /// Type of the next action to perform.
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct RefundNextActionDisplayDetails {
    pub email_sent: EmailSent,

    /// The expiry timestamp.
    pub expires_at: Timestamp,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct EmailSent {
    /// The timestamp when the email was sent.
    pub email_sent_at: Timestamp,

    /// The recipient's email address.
    pub email_sent_to: String,
}

/// The parameters for `Refund::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateRefund<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// The identifier of the charge to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<ChargeId>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Customer whose customer balance to refund from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// For payment methods without native refund support (e.g., Konbini, PromptPay), use this email from the customer to receive refund instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_email: Option<&'a str>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Origin of the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<RefundOrigin>,

    /// The identifier of the PaymentIntent to refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntentId>,

    /// String indicating the reason for the refund.
    ///
    /// If set, possible values are `duplicate`, `fraudulent`, and `requested_by_customer`.
    /// If you believe the charge to be fraudulent, specifying `fraudulent` as the reason will add the associated card and email to your [block lists](https://stripe.com/docs/radar/lists), and will also help us improve our fraud detection algorithms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundReasonFilter>,

    /// Boolean indicating whether the application fee should be refunded when refunding this charge.
    ///
    /// If a full charge refund is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded in an amount proportional to the amount of the charge refunded.
    /// An application fee can be refunded only by the application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,

    /// Boolean indicating whether the transfer should be reversed when refunding this charge.
    ///
    /// The transfer will be reversed proportionally to the amount being refunded (either the entire or partial amount).  A transfer can be reversed only by the application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_transfer: Option<bool>,
}

impl<'a> CreateRefund<'a> {
    pub fn new() -> Self {
        CreateRefund {
            amount: Default::default(),
            charge: Default::default(),
            currency: Default::default(),
            customer: Default::default(),
            expand: Default::default(),
            instructions_email: Default::default(),
            metadata: Default::default(),
            origin: Default::default(),
            payment_intent: Default::default(),
            reason: Default::default(),
            refund_application_fee: Default::default(),
            reverse_transfer: Default::default(),
        }
    }
}

/// The parameters for `Refund::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListRefunds<'a> {
    /// Only return refunds for the charge specified by this charge ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<ChargeId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<RefundId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return refunds for the PaymentIntent specified by this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntentId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<RefundId>,
}

impl<'a> ListRefunds<'a> {
    pub fn new() -> Self {
        ListRefunds {
            charge: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            payment_intent: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListRefunds<'_> {
    type O = Refund;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `Refund::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateRefund<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl<'a> UpdateRefund<'a> {
    pub fn new() -> Self {
        UpdateRefund { expand: Default::default(), metadata: Default::default() }
    }
}

/// An enum representing the possible values of an `RefundDestinationDetailsCard`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RefundDestinationDetailsCardType {
    Pending,
    Refund,
    Reversal,
}

impl RefundDestinationDetailsCardType {
    pub fn as_str(self) -> &'static str {
        match self {
            RefundDestinationDetailsCardType::Pending => "pending",
            RefundDestinationDetailsCardType::Refund => "refund",
            RefundDestinationDetailsCardType::Reversal => "reversal",
        }
    }
}

impl AsRef<str> for RefundDestinationDetailsCardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RefundDestinationDetailsCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for RefundDestinationDetailsCardType {
    fn default() -> Self {
        Self::Pending
    }
}

/// An enum representing the possible values of an `CreateRefund`'s `origin` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RefundOrigin {
    CustomerBalance,
}

impl RefundOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            RefundOrigin::CustomerBalance => "customer_balance",
        }
    }
}

impl AsRef<str> for RefundOrigin {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RefundOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for RefundOrigin {
    fn default() -> Self {
        Self::CustomerBalance
    }
}

/// An enum representing the possible values of an `Refund`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    Duplicate,
    ExpiredUncapturedCharge,
    Fraudulent,
    RequestedByCustomer,
}

impl RefundReason {
    pub fn as_str(self) -> &'static str {
        match self {
            RefundReason::Duplicate => "duplicate",
            RefundReason::ExpiredUncapturedCharge => "expired_uncaptured_charge",
            RefundReason::Fraudulent => "fraudulent",
            RefundReason::RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl AsRef<str> for RefundReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RefundReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for RefundReason {
    fn default() -> Self {
        Self::Duplicate
    }
}

/// An enum representing the possible values of an `CreateRefund`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RefundReasonFilter {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

impl RefundReasonFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            RefundReasonFilter::Duplicate => "duplicate",
            RefundReasonFilter::Fraudulent => "fraudulent",
            RefundReasonFilter::RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl AsRef<str> for RefundReasonFilter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RefundReasonFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for RefundReasonFilter {
    fn default() -> Self {
        Self::Duplicate
    }
}
