#![allow(dead_code)]
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use rusqlite::Connection;
use serde::Deserialize;
use std::{error::Error, fs::File};

fn main() {
    // let _ = load_datastore();
    // let _ = load_addressbase();
    println!("THE END!")
}

#[derive(Debug, Deserialize)]
struct CareProvider {
    #[serde(rename = "CSNumber")]
    cs_number: String,
    #[serde(rename = "CareService")]
    care_service: String,
    #[serde(rename = "Subtype")]
    subtype: String,
    #[serde(rename = "ServiceName")]
    service_name: String,
    #[serde(rename = "Address_line_1")]
    address_line1: String,
    #[serde(rename = "Address_line_2")]
    address_line2: String,
    #[serde(rename = "Address_line_3")]
    address_line3: String,
    #[serde(rename = "Address_line_4")]
    address_line4: String,
    #[serde(rename = "Service_town")]
    service_town: String,
    #[serde(rename = "Service_Postcode")]
    service_postcode: String,
}

fn load_datastore() -> Result<Vec<CareProvider>, Box<dyn Error>> {
    let file = File::open("data/ci_datastore.csv")?;
    let transcoded_file = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    let mut reader = csv::Reader::from_reader(transcoded_file);
    let results = reader.deserialize().flatten().collect();
    Ok(results)
}

#[test]
fn test_load_datastore() {
    let results = load_datastore().unwrap();
    println!("Results length = {}", results.len());
}

#[derive(Debug)]
struct DeliveryPoint {
    uprn: u64,
    organisation_name: Option<String>,
    department_name: Option<String>,
    sub_building_name: Option<String>,
    building_name: Option<String>,
    building_number: Option<u64>,
    dependent_thoroughfare: Option<String>,
    thoroughfare: Option<String>,
    double_dependent_locality: Option<String>,
    dependent_locality: Option<String>,
    post_town: String,
    postcode: String,
}

fn load_addressbase() -> Result<Vec<DeliveryPoint>, Box<dyn Error>> {
    let conn = Connection::open("data/addressbase_premium_scotland.gpkg")?;

    let mut stmt = conn.prepare("SELECT uprn, organisation_name, department_name, sub_building_name, building_name, building_number, dependent_thoroughfare, thoroughfare, double_dependent_locality, dependent_locality, post_town, postcode FROM delivery_point_address")?;
    let delivery_point_iter = stmt.query_map([], |row| {
        Ok(DeliveryPoint {
            uprn: row.get(0)?,
            organisation_name: row.get(1)?,
            department_name: row.get(2)?,
            sub_building_name: row.get(3)?,
            building_name: row.get(4)?,
            building_number: row.get(5)?,
            dependent_thoroughfare: row.get(6)?,
            thoroughfare: row.get(7)?,
            double_dependent_locality: row.get(8)?,
            dependent_locality: row.get(9)?,
            post_town: row.get(10)?,
            postcode: row.get(11)?,
        })
    })?;

    let results: Vec<DeliveryPoint> = delivery_point_iter.flatten().collect();

    Ok(results)
}

#[test]
fn test_load_addressbase() {
    let results = load_addressbase().unwrap();
    println!("Results length = {}", results.len());
}
