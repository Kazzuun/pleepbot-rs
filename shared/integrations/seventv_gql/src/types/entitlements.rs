use cynic::QueryFragment;

use crate::schema;
use crate::types::badges::Badge;

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EntitlementEdgeAnyBadge {
    pub to: EntitlementNodeBadge,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EntitlementNodeBadge {
    pub badge: Option<Badge>,
}
