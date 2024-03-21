

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

pub(crate) fn check_if_sequence_is_harpin(sequence: &String) -> bool{
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