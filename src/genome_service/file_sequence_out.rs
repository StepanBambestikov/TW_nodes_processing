use std::fs;
use std::io::Write;
use crate::genome_service::structure::SubsequenceRelation;


pub(crate) struct FileSequenceOut{
    file: fs::File,
    values: Vec<SubsequenceRelation>,
}

impl FileSequenceOut{
    pub(crate) fn new(file_name: String) -> FileSequenceOut{
        let mut file = fs::File::create(file_name).expect("Unable to create file for saving");
        FileSequenceOut{
            file: file,
            values: Vec::new()
        }
    }
}

impl FileSequenceOut{
    pub(crate) fn add_answer(&mut self, answer: SubsequenceRelation, genome_name: &str) {
        self.values.push(answer);
        // let str = format!("name: {}, sequence, {} first_number: {}, second_number: {}\n",
        //                   genome_name, answer.sequence, answer.first_repetition_number,
        //                   answer.second_repetition_number);
        // self.file.write_all(str.as_ref()).expect("Unable to write data");
    }
}

impl FileSequenceOut{
    pub fn write_all(&mut self){
        let mut output = String::new();
        self.values.sort_by(|a, b|
            (b.second_repetition_number).
                cmp(&a.second_repetition_number));
        for value in self.values.iter() {
            let mut ratio = -1.0;
            if value.second_repetition_number != 0 {
                ratio = value.first_repetition_number as f64 / value.second_repetition_number as f64;
            }
            output.push_str(&*format!("sequence: {}, first: {}, second: {}, ratio: {}\n",
                                      value.sequence, value.first_repetition_number,
                                      value.second_repetition_number, ratio));
        }
        self.file.write_all(output.as_ref()).expect("Unable to write data");
    }
}


// impl SequenceOut for FileSequenceOut{
//     fn add_answer(&mut self, answer: SubsequenceRelation, genome_name: &str) {
//         self.values.push(answer);
//         // let str = format!("name: {}, sequence, {} first_number: {}, second_number: {}\n",
//         //                   genome_name, answer.sequence, answer.first_repetition_number,
//         //                   answer.second_repetition_number);
//         // self.file.write_all(str.as_ref()).expect("Unable to write data");
//     }
// }