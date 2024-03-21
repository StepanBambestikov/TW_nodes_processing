use std::collections::hash_map::{IntoIter, Iter};
use std::fs;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
use crate::genome_service::structure::{SubsequencePair, SubsequenceRelation};

pub(crate) trait SequenceOut {
    fn add_answer(&mut self, answer: SubsequenceRelation, genome_name: &str);
}

/*------------------------------------------------------------------------------------------------*/
pub(crate) trait Genome {
    fn find(&self, sequence: &str) -> SubsequencePair;
    // fn iter(&self) -> Box<dyn Iterator<Item=(&String, &usize)>>;

    fn into_iter(self) -> Box<dyn Iterator<Item=(&'static String, &'static usize)>>;
}

pub(crate) struct HashGenome{
    unique_count: HashMap<String, usize>,
    sequence_count: usize,
    print_values_count: usize,
    file_name: String
}

pub(crate) fn new(file_name: &str) -> HashGenome{
    let file_content = fs::read_to_string(file_name).expect("Unable to read file");
    let mut genome = HashGenome{
        unique_count: HashMap::new(),
        sequence_count: 0,
        print_values_count: 1000,
        file_name: file_name.parse().unwrap(),
    };
    for line in file_content.lines() {
        if line.contains('>') {
            continue;
        }
        genome.process_line(line);
    }
    return genome
}

// impl Genome for HashGenome{
//     fn find(&self, sequence: &str) -> SubsequencePair {
//         let mut output = SubsequencePair{
//             sequence: sequence.parse().unwrap(),
//             repetition_number: 0,
//         };
//         let answer = self.unique_count.get(&*sequence);
//         if let Some(x) = answer {
//             output.repetition_number = *x;
//         }
//         return output
//     }
//
//     // fn iter(&self, &HashGenome) -> Box<dyn Iterator<Item=(&String, &usize)> + 'static> {
//     //     Box::new(HashGenomeIter{
//     //         iter: self.unique_count.iter()
//     //     })
//     // }
//     fn into_iter(self) -> IntoIter<String, usize> {
//         Box::new(HashGenomeIter{
//             iter: self.unique_count.into_iter()
//         })
//     }
// }

impl HashGenome{
    pub fn find(&self, sequence: &str) -> SubsequencePair {
        let mut output = SubsequencePair{
            sequence: sequence.parse().unwrap(),
            repetition_number: 0,
        };
        let answer = self.unique_count.get(&*sequence);
        if let Some(x) = answer {
            output.repetition_number = *x;
        }
        return output
    }

    pub fn save_to_file(&self){
        let mut output = String::new();
        output.push_str(&*format!("Unique number: {}\n", self.unique_count.keys().len()));
        let mut sorted_values: Vec<_> = self.unique_count.iter().collect();
        output.push_str(&*format!("All sequence number: {}\n", self.sequence_count));
        // output.push_str(&*format!("Ratio: {}\n", self.unique_count.keys().len() * 100 / self.sequence_count));
        sorted_values.sort_by(|a, b| b.1.cmp(&a.1));
        // for (i, (key, value)) in sorted_values.iter().take(self.print_values_count).enumerate() {
        //     output.push_str(&*format!("{}: {} - {}\n", i+1, key, value));
        // }
        for (i, (key, value)) in sorted_values.iter().enumerate() {
            output.push_str(&*format!("{}: {} - {}\n", i+1, key, value));
        }
        let mut file = fs::File::create(format!("{}_unique.txt", self.file_name)).expect("Unable to create file for saving");
        file.write_all(output.as_ref()).expect("Unable to write data");
    }
    pub fn into_iter(self) -> IntoIter<String, usize> {
        self.unique_count.into_iter()
    }
}

struct HashGenomeIter<'a>{
    iter: Iter<'a, String, usize>,
}

impl<'a> Iterator for HashGenomeIter<'a>{
    type Item = (&'a String, &'a usize);
    fn next(&mut self) -> Option<Self::Item>{
        self.iter.next()
    }
}

impl IntoIterator for HashGenome {
    type Item = (String, usize);
    type IntoIter = <HashMap<String, usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.unique_count.into_iter()
    }
}

impl HashGenome{
    fn process_line(&mut self, line: &str){
        let copy = String::from(line);
        *self.unique_count.entry(copy).or_insert(0) += 1;
        self.sequence_count += 1;
    }
}
