
pub(crate) trait SequenceAnalyzer {
    fn process_line(&mut self, line: &str);
    fn get_final_data(&mut self) -> String;
}