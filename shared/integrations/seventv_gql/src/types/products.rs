use chrono::{DateTime, Utc};
use cynic::{Enum, QueryFragment};
use ulid::Ulid;

use crate::schema;

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct SubscriptionProduct {
    pub id: Ulid,
    // provider_id
    pub description: Option<String>,
    pub name: String,
    pub benefits: Vec<SubscriptionBenefit>,
    pub updated_at: DateTime<Utc>,
    // search_updated_at
    pub default_variant: SubscriptionProductVariant,
    pub variants: Vec<SubscriptionProductVariant>,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct SubscriptionBenefit {
    pub id: Ulid,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct SubscriptionProductVariant {
    // id (Stripe product id)
    // paypal_id
    pub kind: SubscriptionProductKind,
    pub price: Price,
}

#[derive(Clone, Debug, PartialEq, Enum)]
pub enum SubscriptionProductKind {
    Monthly,
    Yearly,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct Price {
    /// Seems to default to 'eur' or it might depend on IP
    pub currency: String,
    /// Amount in cents
    pub amount: i32,
}
