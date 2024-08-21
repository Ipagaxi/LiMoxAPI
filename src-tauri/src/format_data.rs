use serde::{Deserialize, Serialize};
use serde_json::{Value};
use core::time;
use std::{hash::Hash, io::Cursor, vec};
use polars::{io::json, prelude::*}; 
use chrono::{DateTime, Utc};
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Actor {
    pub name: String,
    pub mbox: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FilterRule {
    path: String,
    value: String,
    option: String,
}

#[derive(Clone)]
pub struct Filter {
    pub id: String,
    pub rules: Vec<FilterRule>,
    pub statement_counter: usize
}

fn preprocess_json(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if map.is_empty() {
                *value = Value::Null;
            } else {
                for (_, v) in map.iter_mut() {
                    preprocess_json(v);
                }
            }
        },
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                preprocess_json(v);
            }
        },
        _ => {}
    }
}

fn transform_to_dataframe(mut json_data: Value) -> DataFrame {
    preprocess_json(&mut json_data);

    let cursor = Cursor::new(json_data.to_string());
    let df = JsonReader::new(cursor.clone())
        .with_json_format(JsonFormat::Json)
        .finish()
        .expect("Transforming data to data frame failed!");
   
    return df;
}

// The path parameter are the elements in the json tree e.g: [actor, name] will search in an element "actor" for the value of an element "name"
pub fn get_values_in_json_tree(json_data: Value, path: &[&str]) -> Series {
    let df = transform_to_dataframe(json_data);

    let mut values = df.column(path[0]).unwrap().clone(); 

    for element in path.iter().skip(1) {
        let field = values.struct_().unwrap().field_by_name(element).unwrap();
        values = field;
    }

    values
}

pub fn series_to_vec(series: Series) -> Vec<String> {

    // Convert Series to Vec<Option<String>>
    let mut result = Vec::new();

    for value in series.iter() {
        // Convert each value to a String and push to the result
        result.push(value.to_string().replace("\"", ""));
    }
    result
}

pub fn datetime_series_to_vec(series: Series) -> Vec<String> {
    // Convert Series to Vec<Option<String>>
    let mut result = Vec::new();

    for value in series.iter() {
        let datetime_str = value.to_string().replace("\"", "");
        let datetime = DateTime::parse_from_rfc3339(datetime_str.as_str()).expect("Failed to parse datetime");

        // Convert to UTC
        let datetime_utc = datetime.with_timezone(&Utc);

        // Format the datetime
        let formatted_datetime = datetime_utc.format("%H:%M:%S%.f").to_string();
        // Convert each value to a String and push to the result
        result.push(formatted_datetime);
    }
    result
}

fn parse_name(mut texts: Vec<String>) -> Vec<String> {
    for i in 0..texts.len() {
        if let Some(pos) = texts[i].rfind('/') {
            // Slice the string after the last '/'
            texts[i] = texts[i][pos + 1..].to_string();
        }
    }
    return texts;
}

pub fn get_prettified_statements(json_data: Value) -> Vec<String> {
    let mut prettified_statements = Vec::new();

    // Check if json_data is an array
    if let Value::Array(values) = json_data {
        for value in values {
            // Convert each value to a prettified JSON string
            if let Ok(pretty_str) = serde_json::to_string_pretty(&value) {
                prettified_statements.push(pretty_str.clone());
            }
        }
    }

    prettified_statements
}

pub fn get_num_statements(json_data: Value) -> usize {
    let df = transform_to_dataframe(json_data);
    return df.shape().0;
}

pub fn get_condensed_statements(json_data: Value) -> Vec<String> {
    let timestamps = datetime_series_to_vec(get_values_in_json_tree(json_data.clone(), &["timestamp"]));
    let actor_names = series_to_vec(get_values_in_json_tree(json_data.clone(), &["actor", "name"]));
    let verb_names = parse_name(series_to_vec(get_values_in_json_tree(json_data.clone(), &["verb", "id"])));
    let object_names =parse_name(series_to_vec(get_values_in_json_tree(json_data.clone(), &["object", "id"])));

    let mut condensed_statements: Vec<String> = Vec::new();

    for i in 0..timestamps.len() {
        condensed_statements.push(format!("{}:  {} {} {}", timestamps[i], actor_names[i], verb_names[i], object_names[i]));
    }
    condensed_statements
}

fn value_in_statement(data: Value, path: Vec<String>, value: String) -> bool {
    let mut currentValue = data;
    for element in path {
        match &currentValue[element] {
            Value::Null => println!("Null"),
            new_value => currentValue = new_value.clone()
        }                   
    }
    println!("{} is in statement: {}", value, currentValue == value);
    return currentValue == value;
}

fn parse_path(path: String) -> Vec<String> {
    let normalized_path = path.replace(". ", ".");
    let string_vec: Vec<String> = normalized_path.split(".").map(|s| s.to_string()).collect();
    string_vec
}

fn filter_statements(filter: Filter, json_data: Value) -> Vec<Value> {
    let mut valid_statements: Vec<Value> = vec![];
    if let Some(json_array) = json_data.as_array() {
        for item in json_array {
            let mut is_valid_statement = true;
            for filter_rule in filter.rules.clone() {
                let value_is_in_statements = value_in_statement(item.clone(), parse_path(filter_rule.path), filter_rule.value);
                match filter_rule.option.as_str() {
                    "In" => {
                        if (!value_is_in_statements) {
                            is_valid_statement = false;
                            break;
                        }
                    },
                    "Out" => {
                        if (value_is_in_statements) {
                            is_valid_statement = false;
                            break;
                        }
                    },
                    _ => {

                    }
                    
                }
            }
            if is_valid_statement {
                valid_statements.push(item.clone());
            }
        }
    }
    valid_statements
}


pub fn get_filtered_statements_counters(json_data: Value, mut filters: Vec<Filter>) -> Vec<Filter> {    
        for filter in filters.iter_mut() {
            let statements = filter_statements(filter.clone(), json_data.clone());
            filter.statement_counter += statements.len();
        }
    
    filters
}

pub fn get_actors(json_data: Value, mut actors: HashSet<Actor>) -> HashSet<Actor> {

    if let Some(array) = json_data.as_array() {
        for item in array {
            let name = item["actor"]["name"].to_string();
            let mbox = item["actor"]["mbox"].to_string();
            let actor = Actor { name, mbox };
            actors.insert(actor);
        }
    }
    actors
}