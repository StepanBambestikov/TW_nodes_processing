

pub struct SubsequenceRelation {
    pub(crate) sequence: String,
    pub(crate) first_repetition_number: usize,
    pub(crate) second_repetition_number: usize,
}

pub struct SubsequencePair {
    pub(crate) sequence: String,
    pub(crate) repetition_number: usize,
}

impl SubsequencePair{
    pub(crate) fn make_subsequence_relation(self, second_repetition_number: usize) -> SubsequenceRelation{
        SubsequenceRelation{
            sequence: self.sequence,
            first_repetition_number: self.repetition_number,
            second_repetition_number,
        }
    }
}