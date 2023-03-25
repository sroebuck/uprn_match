#![allow(dead_code)]
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use rusqlite::Connection;
use serde::Deserialize;
use std::{error::Error, fmt::Display, fs::File};

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

impl Display for CareProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = vec![
            &self.service_name,
            &self.address_line1,
            &self.address_line2,
            &self.address_line3,
            &self.address_line4,
            &self.service_town,
            &self.service_postcode,
        ];
        let address = x
            .iter()
            .map(|x| (*x).clone())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", address)
    }
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
    let care_providers = load_datastore().unwrap();
    println!("Results length = {}", care_providers.len());
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

impl Display for DeliveryPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let build_number_option = self.building_number.map(|n| n.to_string());
        let street_vec = vec![
            &self.building_name,
            &build_number_option,
            &self.dependent_thoroughfare,
        ];
        let street = Some(
            street_vec
                .iter()
                .flat_map(|x| (*x).clone())
                .collect::<Vec<String>>()
                .join("x "),
        );
        let x = vec![
            &self.organisation_name,
            &self.department_name,
            &self.sub_building_name,
            &street,
            &self.thoroughfare,
            &self.double_dependent_locality,
            &self.dependent_locality,
        ];
        let address = x
            .iter()
            .flat_map(|x| (*x).clone())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}, {}, {}", address, &self.post_town, &self.postcode)
    }
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
    let delivery_points = load_addressbase().unwrap();
    println!("Results length = {}", delivery_points.len());
}

fn match_addresses() {
    let care_providers = load_datastore().unwrap();
    let delivery_points = load_addressbase().unwrap();
    for care_provider in care_providers.iter().take(10) {
        match_address(&delivery_points, care_provider);
    }
}

fn match_address(
    delivery_points: &Vec<DeliveryPoint>,
    care_provider: &CareProvider,
) -> Option<u64> {
    let cp = format!("{}", care_provider).to_uppercase();
    println!("  {}", care_provider);
    let postcode = &care_provider.service_postcode;
    let postcode_len = postcode.len();

    let matches = delivery_points.iter().filter(|d| {
        if postcode_len > 0 {
            &d.postcode == postcode
        } else {
            // d.post_town == care_provider.service_town.to_uppercase()
            false // So far there is no evidence that choosing the post town improves results and it slows down processing a lot!
        }
    });
    let mut match_distances = matches
        .map(|x| {
            let dp = format!("{}", x);
            let distance = strsim::osa_distance(&cp, &dp);
            (distance, dp, x)
        })
        .collect::<Vec<(usize, String, &DeliveryPoint)>>();
    match_distances.sort_by(|(a, ..), (b, ..)| a.cmp(b));
    for (distance, s, ..) in match_distances.iter().take(10) {
        println!("    - {}: {}", distance, s)
    }
    match_distances.get(0).map(|(_, _, dp)| dp.uprn)
}

#[test]
fn test_match_addresses() {
    match_addresses();
}

#[test]
fn test_specific_addresses() {
    let care_providers = load_datastore().unwrap();
    let delivery_points = load_addressbase().unwrap();
    let answers = vec![
        129001382,    // 0
        906700068653, // 1
        0,            // 2 - Looks like this may not have a valid UPRN
        906700286501, // 3
        131040962,    // 4 - Not sure why this one isn't matching - investigate
        9051131974,   // 5
        9051042627,   // 6
        9051030767,   // 7
        9051024604,   // 8
        9051155067,   // 9 - Looks like it has the wrong postcode, should be AB10 7PA?
        9051068080,   // 10 - Near miss - work investigating.
        9051090598,   // 11
        9051057534,   // 12 - Missing a postcode
        9051148837,   // 13
        9051069228,   // 14 - Near miss - seems to be ignoring house number
        9051066941,   // 15 - Near miss - seems to be ignoring house number
        9051082493,   // 16
        9051032579,   // 17
        9051040562,   // 18
        9051015777,   // 19 - Near miss - seems to be ignoring house number
        9051128221,   // 20 - Near miss - seems to be mis matching house number
        9051068040,   // 21 - Near miss - seems to be ignoring house number
        9051040256,   // 22 - Near miss - seems to be ignoring house number
        9051087058,   // 23
        9051063112,   // 24
        9051044743,   // 25 - Near miss - seems to be ignoring house number
        9051128497,   // 26
        9051147105,   // 27
        9051021328,   // 28
        9051002006,   // 29 - Near miss - seems to be ignoring house number
        151085895,    // 30
        151700003,    // 31
        151062069,    // 32
        151087318,    // 33
        151087701,    // 34
        151087426,    // 35
        151085894,    // 36
        151087875,    // 37
        151088292,    // 38 - Near miss - seems to be ignoring house number
        151087974,    // 39 - Really poor address - Google helped
    ];
    let test_size = answers.len();
    let results = care_providers
        .iter()
        .enumerate()
        .take(test_size)
        .zip(answers)
        .map(|((i, cp), a)| {
            println!("{}:", i);
            let r = match_address(&delivery_points, &cp);
            if r != Some(a) {
                println!("Expected {} but matched to {} !", a, r.unwrap_or_default());
            }
            r == Some(a)
        });
    let count = results.filter(|&x| x).count();
    assert_eq!(count, test_size);
}
