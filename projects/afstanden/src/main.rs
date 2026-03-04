use std::{collections::HashMap, io};

fn input_query<S: AsRef<str>>(query: S) -> String {
    println!("{}", query.as_ref());
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input.");

    buffer
}

const KM: f64 = 1000000.0;
const DAM: f64 = 0100000.0;
const HM: f64 = 0010000.0;
const M: f64 = 0001000.0;
const DM: f64 = 0000100.0;
const CM: f64 = 0000010.0;
const MM: f64 = 0000001.0;

fn main() {
    let mut afstanden_map = HashMap::new();

    afstanden_map.insert("kilometer", KM);
    afstanden_map.insert("decameter", DAM);
    afstanden_map.insert("hectometer", HM);
    afstanden_map.insert("meter", M);
    afstanden_map.insert("decimeter", M);
    afstanden_map.insert("centimere", M);
    afstanden_map.insert("millimeter", M);

    afstanden_map.insert("km", KM);
    afstanden_map.insert("dam", DAM);
    afstanden_map.insert("hm", HM);
    afstanden_map.insert("m", M);
    afstanden_map.insert("dm", DM);
    afstanden_map.insert("cm", CM);
    afstanden_map.insert("mm", MM);

    let distance = input_query("geef de afstand:")
        .trim()
        .replace(" ", "")
        .to_lowercase();

    println!("distance: {}", distance);

    let split_index = distance
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(distance.len());

    let (distance_str, distance_unit) = distance.split_at(split_index);
    let distance_num = distance_str
        .parse::<f64>()
        .expect(&format!("Couldn't convert {} to f64.", distance_str));

    let target_unit = input_query("Converteer naar eenheid:")
        .trim()
        .replace(" ", "")
        .to_lowercase();

    println!("target_unit: {}", target_unit);

    let Some(distance_map) = afstanden_map.get(distance_unit) else {
        println!("No distance mapped for '{}'.", distance_unit);
        return;
    };

    let target_str: &str = target_unit.as_ref();
    let Some(target_map) = afstanden_map.get(target_str) else {
        println!("No distance mapped for '{}'.", target_unit);
        return;
    };

    println!(
        "distance: {}\ndistance map: {}\ntarget map: {}\nexpression: {} < {}",
        distance_num, distance_map, target_map, distance_map, target_map
    );

    let result = distance_num / target_map * distance_map;
    println!(
        "{} {} -> {} {}",
        distance_str, distance_unit, result, target_unit
    );
}
