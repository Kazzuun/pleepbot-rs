use chrono::{DateTime, Utc};
use cynic::{Enum, InlineFragments, QueryFragment};
use ulid::Ulid;

use crate::schema;
use crate::types::images::Image;

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct Paint {
    pub id: Ulid,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub data: PaintData,
    pub created_by_id: Ulid,
    pub updated_at: DateTime<Utc>,
    // search_updated_at
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct PaintData {
    pub layers: Vec<PaintLayer>,
    pub shadows: Vec<PaintShadow>,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct PaintLayer {
    pub id: Ulid,
    pub ty: PaintLayerType,
    pub opacity: f64,
}

#[derive(Clone, Debug, PartialEq, InlineFragments)]
pub enum PaintLayerType {
    PaintLayerTypeSingleColor(PaintLayerTypeSingleColor),
    PaintLayerTypeLinearGradient(PaintLayerTypeLinearGradient),
    PaintLayerTypeRadialGradient(PaintLayerTypeRadialGradient),
    PaintLayerTypeImage(PaintLayerTypeImage),

    #[cynic(fallback)]
    Other,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct Color {
    pub hex: String,
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct PaintLayerTypeSingleColor {
    pub color: Color,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct PaintLayerTypeLinearGradient {
    pub angle: i32,
    pub repeating: bool,
    pub stops: Vec<PaintGradientStop>,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct PaintGradientStop {
    pub at: f64,
    pub color: Color,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct PaintLayerTypeRadialGradient {
    pub repeating: bool,
    pub stops: Vec<PaintGradientStop>,
    pub shape: PaintRadialGradientShape,
}

#[derive(Clone, Debug, PartialEq, Enum)]
pub enum PaintRadialGradientShape {
    Ellipse,
    Circle,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct PaintLayerTypeImage {
    pub images: Vec<Image>,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct PaintShadow {
    color: Color,
    offset_x: f64,
    offset_y: f64,
    blur: f64,
}
