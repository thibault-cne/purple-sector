use rocket::serde::Serialize;

use super::models::*;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DriversResponse {
    pub drivers: Vec<Driver>,

    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ConstructorResponse {
    pub constructors: Vec<Constructor>,

    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StandingsResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub drivers_standings: Vec<DriverStanding>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub constructors_standings: Vec<ConstructorStanding>,
    #[serde(flatten)]
    pub pagination: Pagination,
    pub series: Series,
}
