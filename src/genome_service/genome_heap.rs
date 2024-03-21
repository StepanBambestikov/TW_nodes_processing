use std::collections::HashMap;

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

impl<'a> GenomeHeap<'a>{
    pub(crate) fn add_sequence(&mut self, pair: (String, usize), genome_number: usize){
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