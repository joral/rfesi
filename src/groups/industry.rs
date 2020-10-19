#![allow(unused)]

use crate::prelude::*;
use serde::Deserialize;

/// Endpoints for Industry
pub struct IndustryGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CostIndex {
    pub activity: String,
    pub cost_index: f64
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct IndustryFacility {
    pub facility_id: u64,
    pub owner_id: u64,
    pub region_id: u64,
    pub solar_system_id: u64,
    pub tax: Option<f64>,
    pub type_id: u64
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct IndustrySystem {
    pub cost_indices: Vec<CostIndex>,
    pub solar_system_id: u64
}

impl<'a> IndustryGroup<'a> {
    api_get!(
        /// Get list of facilities.
        list_facilities,
        "get_industry_facilities",
        RequestType::Public,
        Vec<IndustryFacility>,
    );
}

impl<'a> IndustryGroup<'a> {
    api_get!(
        /// Get list of facilities.
        list_systems,
        "get_industry_systems",
        RequestType::Public,
        Vec<IndustrySystem>,
    );
}

