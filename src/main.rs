mod genome;
mod genome_service;
mod sequence_analyzer;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Write};
use std::io::BufReader;
use serde_json::{Value, Map};
use std::string::String;
use std::env;

fn main() {
    //terminal args preparation
    let args: Vec<String> = env::args().collect();
    let json_file_name = args[1].clone();
    let stem_size = args[2].clone().to_owned();
    let file_names: Vec<String> = args.iter().skip(3).cloned().collect();
    let organism_names = get_organism_names(file_names.clone(), json_file_name);
    let genome_number = file_names.len();

    let mut genome_heap = genome_service::genome_heap::GenomeHeap::new(&organism_names);
    print!("processing begin/n");
    for current_genome_index in 0..genome_number{
        print!("{}/n", current_genome_index);
        let current_genome = genome::new(&file_names[current_genome_index]);
        current_genome.save_to_file();
        for (current_sequence, repetition_number) in current_genome{
            genome_heap.add_sequence((current_sequence, repetition_number), current_genome_index);
        }
    }

    //output_nodes general heap writing
    let mut file = File::create("output_nodes/output_nodes_".to_owned() + &stem_size + ".txt").expect("Unable to create file for saving");
    file.write_all(genome_heap.get_data().as_ref()).expect("Unable to write data");
}

fn parse_json_file(json_file_name: String) -> HashMap<String, String>{
    let file = File::open(json_file_name).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut organism_map = HashMap::new();
    for line in reader.lines() {
        let json_str = line.expect("Failed to read line");
        let json_value: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");
        let obj: Map<String, Value> = json_value.as_object().unwrap().clone();
        let organism_name: Map<String, Value> = obj["organism"].as_object().unwrap().clone();
        let accession = obj["accession"].as_str().expect("Failed to get accession").to_string();
        let organism_name = organism_name["organismName"].as_str().expect("Failed to get organismName").to_string();
        organism_map.insert(accession, organism_name);
    }
    return organism_map
}

fn get_organism_names(file_names: Vec<String>, json_file_name: String) -> Vec<String> {
    let mut organism_names = Vec::new();
    let organism_map = parse_json_file(json_file_name);
    for current_file_name in &file_names{
        let parts: Vec<&str> = current_file_name.split("/").collect();
        organism_names.push(organism_map[parts[2]].as_str().to_string());
    }
    return organism_names
}