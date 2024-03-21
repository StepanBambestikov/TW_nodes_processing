use std::collections::HashMap;
use crate::sequence_analyzer;

pub(crate) struct UniqueAnalyzer{
    unique_count: HashMap<String, usize>,
    sequence_count: usize,
    print_values_count: usize,
}
impl sequence_analyzer::SequenceAnalyzer for UniqueAnalyzer{

    fn process_line(&mut self, line: &str){
        let copy = String::from(line);
        *self.unique_count.entry(copy).or_insert(0) += 1;
        self.sequence_count += 1;
    }

    fn get_final_data(&mut self) -> String{
        let mut output = String::new();
        output.push_str(&*format!("Unique number: {}\n", self.unique_count.keys().len()));
        let mut sorted_values: Vec<_> = self.unique_count.iter().collect();
        output.push_str(&*format!("All sequence number: {}\n", self.sequence_count));
        output.push_str(&*format!("Ratio: {}\n", self.unique_count.keys().len() * 100 / self.sequence_count));
        sorted_values.sort_by(|a, b| b.1.cmp(&a.1));
        // for (i, (key, value)) in sorted_values.iter().take(self.print_values_count).enumerate() {
        //     output.push_str(&*format!("{}: {} - {}\n", i+1, key, value));
        // }
        for (i, (key, value)) in sorted_values.iter().enumerate() {
            output.push_str(&*format!("{}: {} - {}\n", i+1, key, value));
        }
        return output
    }

}

impl UniqueAnalyzer{
    pub(crate) fn new() -> Box<dyn sequence_analyzer::SequenceAnalyzer>{
        Box::new(UniqueAnalyzer{
            unique_count: HashMap::new(),
            sequence_count: 0,
            print_values_count: 1000,
        })
    }
}