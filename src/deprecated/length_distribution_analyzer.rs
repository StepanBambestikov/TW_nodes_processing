use std::collections::HashMap;
use crate::sequence_analyzer;

pub(crate) struct LengthDistributionAnalyzer{
    length_distribution: HashMap<usize, u32>,
}
impl sequence_analyzer::SequenceAnalyzer for LengthDistributionAnalyzer{

    fn process_line(&mut self, line: &str){
        *self.length_distribution.entry(line.len()).or_insert(0) += 1;
    }

    fn get_final_data(&mut self) -> String{
        let mut output = String::new();
        for (key, value) in &self.length_distribution {
            output.push_str(&*format!("{length}: {frequency}\n", length = key, frequency = value));
        }
        return output
    }
}

impl LengthDistributionAnalyzer{
    pub(crate) fn new() -> Box<dyn sequence_analyzer::SequenceAnalyzer>{
        Box::new(LengthDistributionAnalyzer{
            length_distribution: HashMap::new(),
        })
    }
}