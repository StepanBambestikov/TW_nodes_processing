use std::collections::hash_map::{IntoIter, Iter};
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use crate::genome_service::structure::SubsequencePair;
use std::io::Write;

pub(crate) struct HashGenome{
    unique_count: HashMap<String, usize>,
    sequence_count: usize,
    print_values_count: usize,
    file_name: String,
    filtered_file: File,
    filtered_coords_file: File,
    stem_size: i32
}

pub(crate) fn new(file_name: &str, stem_size: i32) -> HashGenome{
    let file_content = fs::read_to_string(file_name).expect("Unable to read file");
    //output files creation
    let filtered_file = File::create(format!("{file_name}_results/{file_name}_filtered.txt"))
        .expect("Unable to create filtered file for saving");
    let filtered_coords_file = File::create(format!("{file_name}_results/{file_name}_filtered_coords.txt"))
        .expect("Unable to create filtered coords file for saving");

    filtered_file.set_len(0)
        .expect("Unable to truncate filtered file");
    filtered_coords_file.set_len(0)
        .expect("Unable to truncate filtered coords file");

    let mut genome = HashGenome{
        unique_count: HashMap::new(),
        sequence_count: 0,
        print_values_count: 1000,
        file_name: file_name.parse().unwrap(),
        filtered_file,
        filtered_coords_file,
        stem_size
    };

    //file processing
    let mut line_block = (String::new(), String::new());
    for line in file_content.lines() {
        if line.contains('>') {
            line_block.0 = line.parse().unwrap();
        } else{
            line_block.1 = line.parse().unwrap();
        }
        genome.process_line_block(line_block.clone());
    }
    return genome
}

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

    fn process_line_block(&mut self, line_block: (String, String)){
        //check harping filter
        let sequence_is_harpin = crate::genome_service::harpin_filter::check_if_sequence_is_harpin(&line_block.1, self.stem_size);
        if sequence_is_harpin{
            return
        }

        //processing for filtered values
        write!(self.filtered_file, "{}\n {}\n", line_block.0, line_block.1)
            .expect("cant write to file");

        //processing for coords file
        let node_info: Vec<&str> = line_block.0.split_whitespace().collect();
        let node_len = node_info.len();
        write!(self.filtered_coords_file, "{} {}\n", node_info[node_len - 17], node_info[node_len - 10])
            .expect("cant write to file");

        //processing for unique heap
        *self.unique_count.entry(line_block.1).or_insert(0) += 1;
        self.sequence_count += 1;
    }

    pub fn into_iter(self) -> IntoIter<String, usize> {
        self.unique_count.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::genome;

    #[test]
    fn test_one_file_processing() {
        let file_name = "GCF_000001405.40_GRCh38.p14_genomic.fna_12_0_nodes_new.txt";
        let current_genome = genome::new(&file_name, 5);
        current_genome.save_to_file();
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