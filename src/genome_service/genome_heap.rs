use std::collections::HashMap;
use crate::genome::HashGenome;
use crate::genome_service::structure::SubsequencePair;

pub(crate) struct GenomeHeap<'a> {
    unique_count: HashMap<String, Vec<usize>>,
    genome_names: &'a Vec<String>
}

impl<'a> GenomeHeap<'a> {
    pub(crate) fn new(genome_names: &'a Vec<std::string::String>) -> GenomeHeap<'a> {
        GenomeHeap {
            unique_count: HashMap::new(),
            genome_names
        }
    }
}

fn maker_inverse_comp_sequence(seq: &str) -> String{
    let comp_seq = seq.chars().map(|c| {
        match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => c,
        }
    }).collect::<String>();
    let rev_comp_seq = comp_seq.chars().rev().collect::<String>();
    rev_comp_seq
}

fn check_if_sequence_is_harpin(sequence: &String) -> bool{
    let mut current_index = 8;
    while current_index < sequence.len() - 8{
        let mut distance = 0;
        if current_index < sequence.len() / 2{
            distance = current_index;
        } else{
            distance = sequence.len() - current_index - 1;
        }
        let comp_sequence = maker_inverse_comp_sequence(&sequence[current_index - distance..current_index]);
        let count_equal_chars = comp_sequence.chars()
            .zip(sequence[current_index..current_index + distance].chars());
        let mut max_consecutive_count = 0;
        let mut current_consecutive_count = 0;
        for (c1, c2) in count_equal_chars{
            if c1 == c2 {
                current_consecutive_count += 1;
            } else {
                current_consecutive_count = 0;
            }
            if current_consecutive_count > max_consecutive_count {
                max_consecutive_count = current_consecutive_count;
            }
        }
        if max_consecutive_count >= 8 {
            return true
        }
        current_index += 1;
    }
    return false
}

impl<'a> GenomeHeap<'a>{
    pub(crate) fn add_sequence(&mut self, pair: (String, usize), genome_number: usize){
        let sequence_is_harpin = check_if_sequence_is_harpin(&pair.0);
        if sequence_is_harpin{
            // print!("{:?}\n", pair.0);
            return
        }
        self.unique_count.entry(pair.0).
            or_insert(vec![0; self.genome_names.len()])
            [genome_number] = pair.1;
    }

    pub fn get_data(&mut self) -> String{
        let mut output = String::new();
        let mut sorted_values: Vec<_> = self.unique_count.iter().
            filter(|&(_, values)| {
                let mut count = 0;
                for &elem in values{
                    if elem != 0{
                        count+= 1;
                        if count >= 2{
                            return true;
                        }
                    }
                }
                false
            }).
            collect();
        sorted_values.sort_by(|a, b| (b.1.iter().sum::<usize>()).cmp(&a.1.iter().sum()));
        for current_genome_name in self.genome_names{
            output.push_str(&*format!("{},", current_genome_name));
        }
        output.push('\n');
        for value in sorted_values {
            output.push_str(value.0);
            output.push_str(",");
            for current_genome_index in 0..self.genome_names.len(){
                output.push_str(&*format!("{},", value.1[current_genome_index]));
                // output.push_str(&*format!(" {}, ",
                //                           self.genome_names[current_genome_index], value.1[current_genome_index]));
            }
            output.push('\n');
        }
        return output;
    }
}