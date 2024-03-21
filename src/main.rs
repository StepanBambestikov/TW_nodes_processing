mod sequence_analyzer;
mod nucleotide_frequency_analyzer;
mod length_distribution_analyzer;
mod unique_analyzer;
mod genome;
mod genome_service;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, Write};
use crate::genome_service::file_sequence_out::FileSequenceOut;
use crate::genome::{Genome, SequenceOut};
use crate::sequence_analyzer::SequenceAnalyzer;
use std::io::BufReader;
use serde_json::{Value, Map};
// use serde_json::Value::String;
use std::string::String;
use std::env;

// fn main() {
//     let file_name = "nodes.txt";
//     let file_content = fs::read_to_string(file_name).expect("Unable to read file");
//     let mut analyzers: Vec<Box<dyn SequenceAnalyzer>> = vec![
//         UniqueAnalyzer::new(),
//     ];
//     let mut count = 0;
//     for line in file_content.lines() {
//         if line.contains('>'){
//             continue;
//         }
//         for current_analyzer in &mut analyzers{
//             current_analyzer.process_line(line);
//         }
//         count += 1;
//         if count % 100000 == 0{
//             println!("{}\n", count)
//         }
//     }
//     let mut file = fs::File::create(format!("{}_output.txt", file_name)).expect("Unable to create file for saving");
//     for current_analyzer in &mut analyzers{
//         let str = current_analyzer.get_final_data();
//         // println!("{}", str)
//         file.write_all(str.as_ref()).expect("Unable to write data");
//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_file_name = args[1].clone();
    let stem_size = args[2].clone().to_owned();
    let file_names: Vec<String> = args.iter().skip(3).cloned().collect();

    let mut file = File::open(json_file_name).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut organism_map = HashMap::new();
    for line in reader.lines() {
        let json_str = line.expect("Failed to read line");
        let json_value: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");
        let obj: Map<String, Value> = json_value.as_object().unwrap().clone();
        let d = obj["organism"].clone();
        let organism_name: Map<String, Value> = d.as_object().unwrap().clone();
        let accession = obj["accession"].as_str().expect("Failed to get accession").to_string();
        let organism_name = organism_name["organismName"].as_str().expect("Failed to get organismName").to_string();
        organism_map.insert(accession, organism_name);
    }
    // print!("{:?}", file_names.len());
    // let other_genome_files = vec!["nodes.txt", "mouse.txt", "e-cori.txt"];
    // let genome_names = vec!["human", "mouse","e-cori"];
    let mut organism_names = Vec::new();
    for current_file_name in &file_names{
        let parts: Vec<&str> = current_file_name.split("/").collect();
        // print!("{:?}", parts[2]);
        organism_names.push(organism_map[parts[2]].as_str().to_string());
    }
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
    let mut file = File::create("output_nodes/output_nodes_".to_owned() + &stem_size + ".txt").expect("Unable to create file for saving");
    file.write_all(genome_heap.get_data().as_ref()).expect("Unable to write data");
}

// fn main() {
//    let general_genome = genome::new("nodes.txt");
//    let other_genome_files = vec!["e-cori.txt"];
//    let mut out_manager = FileSequenceOut::new(String::from("node_output.txt"));
//    for current_file in other_genome_files{
//       let current_genome = genome::new(current_file);
//       for (current_sequence, repetition_number) in current_genome{
//          let current_answer = general_genome.find(current_sequence.as_str());
//          out_manager.add_answer(current_answer.make_subsequence_relation(repetition_number), current_file)
//       }
//    }
//    out_manager.write_all();
// }

// fn main() {
//    // let general_genome = genome::new("nodes.txt");
//    let other_genome_files = vec!["nodes.txt"];
//    let other_genome_names = vec!["human_genome"];
//    let mut out_manager = PostgresSequenceOut::new("host=localhost user=postgres password=Qqqwwweee12321", &other_genome_names).
//        unwrap_or_else(|error| {
//           panic!("Problem with database: {:?}", error);
//    });
//    for current_index in 0..other_genome_files.len(){
//        let current_genome = genome::new(other_genome_files[current_index]);
//        let mut count = 0;
//        for (current_sequence, repetition_number) in current_genome{
//          out_manager.add_answer(current_sequence, repetition_number, other_genome_names[current_index]).
//              unwrap_or_else(|error| {
//                 panic!("Problem with insert: {:?}", error);
//              });
//        count += 1;
//        if count % 100 == 0{
//            print!("{count}\n")
//        }
//       }
//    }
// }