use std::collections::HashMap;
use crate::sequence_analyzer;

pub(crate) struct NucleotideFrequencyAnalyzer{
    char_count: HashMap<char, f64>,
    data_length: f64,
}
impl sequence_analyzer::SequenceAnalyzer for NucleotideFrequencyAnalyzer{

    fn process_line(&mut self, line: &str){
        for current_char in line.chars() {
            *self.char_count.entry(current_char).or_insert(0.) += 1.;
        }
        self.data_length += line.len() as f64;
    }

    fn get_final_data(&mut self) -> String{
        let mut output = String::new();
        for (_, current_value) in self.char_count.iter_mut(){
            *current_value /= self.data_length;
        }
        for (key, value) in &self.char_count {
            output.push_str(&*format!("{nuc}: {frequency}\n", nuc = key, frequency = value));
        }
        return output
    }
}

impl NucleotideFrequencyAnalyzer{
    pub(crate) fn new() -> Box<dyn sequence_analyzer::SequenceAnalyzer>{
        Box::new(NucleotideFrequencyAnalyzer{
            char_count: HashMap::new(),
            data_length: 0.,
        })
    }
}