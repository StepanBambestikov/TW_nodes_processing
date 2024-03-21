use std::collections::HashMap;

// struct GenomeNode {
//     repetition_number: usize,
//     A_branch: Option<Box<GenomeNode>>,
//     T_branch: Option<Box<GenomeNode>>,
//     G_branch: Option<Box<GenomeNode>>,
//     C_branch: Option<Box<GenomeNode>>
// }
//
// fn default_genome_node() -> GenomeNode{
//     GenomeNode {
//         repetition_number: 0,
//         A_branch: None,
//         T_branch: None,
//         G_branch: None,
//         C_branch: None
//     }
// }
//
// pub(crate) struct GenomeTree {
//     tree_root: Option<Box<GenomeNode>>,
//     mismatch_number: usize
// }
//
// fn new(mismatch_number: usize) -> GenomeTree{
//     GenomeTree{
//         tree_root: Some(Box::from(default_genome_node())),
//         mismatch_number
//     }
// }
//
// fn check_node_for_adding(mut val: &Option<Box<GenomeNode>>) -> &Option<Box<GenomeNode>>{
//     if !val.is_some(){
//         let new_node = Some(Box::from(default_genome_node()));
//         val = &new_node;
//     }
//     return val
// }
//
// impl GenomeNode{
//     fn process_letter_for_adding(&mut self, char: char, repetition_number: usize) -> &Option<Box<GenomeNode>>{
//         self.repetition_number += repetition_number;
//         match char{
//             'A' => {return check_node_for_adding(&self.A_branch);},
//             'T' => {return check_node_for_adding(&self.T_branch);},
//             'G' => {return check_node_for_adding(&self.G_branch);},
//             'C' => {return check_node_for_adding(&self.C_branch);},
//             _ => return &None
//         }
//     }
//
//     fn process_letter_for_finding(&self, char: char, mut mismatch: usize) -> Vec<(usize, usize, &Option<Box<GenomeNode>>)> {
//         if mismatch == 0{
//             match char{
//                 'A' => return vec![(0, 0, &self.A_branch)],
//                 'T' => return vec![(0, 0, &self.T_branch)],
//                 'G' => return vec![(0, 0, &self.G_branch)],
//                 'C' => return vec![(0, 0, &self.C_branch)],
//                 _ => return vec![(0, 0, &None)]
//             }
//         }
//         mismatch -= 1;
//         let answer =  vec![
//             (mismatch, 0, &self.A_branch),
//             (mismatch, 0, &self.T_branch),
//             (mismatch, 0, &self.G_branch),
//             (mismatch, 0, &self.C_branch),
//         ];
//         match char{
//             'A' => answer[0].0 += 1,
//             'T' => answer[0].0 += 1,
//             'G' => answer[0].0 += 1,
//             'C' => answer[0].0 += 1,
//             _ => return vec![]
//         }
//         return answer
//     }
// }
//
// impl GenomeTree{
//     fn add_sequence(&mut self, sequence: String, repetition_number: usize){
//         let mut _current_node = &self.tree_root;
//         for current_char in sequence.chars(){
//             _current_node = _current_node.unwrap().process_letter_for_adding(current_char, repetition_number);
//         }
//     }
//
//     fn find_sequence(&mut self, sequence: String){
//         let sequence_index = 0;
//         let mut working_nodes = vec![(self.mismatch_number, sequence_index, &self.tree_root)];
//         let mut current_node_index = 0;
//         let byte_string = sequence.as_bytes();
//         while !working_nodes.is_empty(){
//             //work with current node
//             let (mismatch, index, node) = working_nodes.remove(0);
//             let val = node.unwrap();
//             let new_nodes = val.process_letter_for_finding(byte_string[index] as char, mismatch);
//             if index < sequence.len() - 1{
//                 working_nodes.extend(new_nodes);
//             }
//             current_node_index += 1;
//         }
//     }
//
// }