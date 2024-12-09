use std::collections::{BinaryHeap, HashMap, LinkedList};

use core::cmp::Reverse;

pub fn puzzle1() -> Result<usize, Box<dyn std::error::Error>> {
    handle_puzzle1(std::fs::read_to_string("./inputs/dec09.txt")?.as_str())
}

pub fn puzzle2() -> Result<usize, Box<dyn std::error::Error>> {
    handle_puzzle2(std::fs::read_to_string("./inputs/dec09.txt")?.as_str())
}

fn handle_puzzle1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    // Approach: double-pointer through a LinkedList

    // generate files && free blocks in one list
    let mut blocks = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                Block::File(size, i / 2)
            } else {
                Block::Free(size)
            }
        })
        .collect::<LinkedList<_>>();

    // an accumulator for ultracompact disk blocks. blocks_right does not exist because it has no
    // bearing on our computation for checksum
    let mut blocks_left: LinkedList<Block> = LinkedList::new();

    // double pointer is achieved by popping off of either side of the linked list.
    while !blocks.is_empty() {
        // get the next free block from the left ptr, stashing compact file blocks
        let free = loop {
            if let Some(block) = blocks.pop_front() {
                match block {
                    Block::Free(_) => break Some(block),
                    Block::File(_, _) => {
                        blocks_left.push_back(block);
                        continue;
                    }
                }
            } else {
                break None;
            }
        };
        if blocks.is_empty() {
            break;
        }
        // discard unimportant free blocks, until we find a file block
        let file = loop {
            if let Some(block) = blocks.pop_back() {
                match block {
                    Block::Free(_) => {
                        continue;
                    }
                    Block::File(_, _) => break Some(block),
                }
            } else {
                break None;
            }
        };

        // compute the transfer from right to left, discarding anything possible
        if let (Some(mut free), Some(mut file)) = (free, file) {
            let frag = free.frag(&mut file).unwrap();
            blocks_left.push_back(frag);
            if free.size() > 0 {
                blocks.push_front(free);
            }
            if file.size() > 0 {
                blocks.push_back(file);
            }
        }
    }

    Ok(Block::get_checksum(blocks_left.iter()))
}

fn handle_puzzle2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    // Approach: exploit problem constraints to create a constant-sized map of min-heaps, leading to
    // acceptable/optimal(?) leftmost position search

    let mut holes = HashMap::<usize, BinaryHeap<Reverse<usize>>>::new();
    for i in 0..=9 {
        holes.insert(i, BinaryHeap::new());
    }

    let mut files_in = Vec::<(usize, usize, usize)>::new();
    let mut files_out = Vec::<(usize, usize, usize)>::new();
    let mut entity_pos = 0;

    let disk_map = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate();

    // init stuff
    for (i, entity_size) in disk_map {
        if i % 2 == 0 {
            let file_id = i / 2;
            files_in.push((entity_pos, entity_size, file_id))
        } else {
            holes
                .get_mut(&entity_size)
                .unwrap()
                .push(Reverse(entity_pos));
        }

        entity_pos += entity_size;
    }

    // poke Map (sizes) -> MinHeap(positions) to find leftmost position that fits the next file
    for (mut file_pos, file_size, file_id) in files_in.into_iter().rev() {
        let (mut best_pos, mut its_size) = (None, usize::MAX);

        const MAX_ENTITY_SIZE: usize = 9;
        for free_size in file_size..=MAX_ENTITY_SIZE {
            let min_heap = holes.get_mut(&free_size).unwrap();
            let candidate_pos: usize = match min_heap.peek() {
                Some(Reverse(pos)) => *pos,
                None => usize::MAX,
            };

            if candidate_pos < file_pos && (best_pos.is_none() || candidate_pos < best_pos.unwrap())
            {
                best_pos = Some(candidate_pos);
                its_size = free_size;
            }
        }

        if let Some(best_pos) = best_pos {
            holes.get_mut(&its_size).unwrap().pop();
            file_pos = best_pos;
            // add new, smaller hole back to the pile if necessary
            if file_size < its_size {
                let new_free_size = its_size - file_size;
                let new_free_pos = file_pos + file_size;
                holes
                    .get_mut(&new_free_size)
                    .unwrap()
                    .push(Reverse(new_free_pos));
            }
        }

        files_out.push((file_pos, file_size, file_id));
    }

    let mut sum = 0;

    for (pos, size, id) in files_out.into_iter().rev() {
        sum += id * (pos..pos + size).sum::<usize>();
    }

    Ok(sum)
}

enum Block {
    // size
    Free(usize),
    // size, id
    File(usize, usize),
}

impl Block {
    pub fn size(&self) -> usize {
        match self {
            Block::Free(size) => *size,
            Block::File(size, _) => *size,
        }
    }

    /// fragment a Free block with File block `other`.
    /// Modifies this block in place, and produces a fragmented File block as output
    pub fn frag(&mut self, other: &mut Block) -> Option<Block> {
        if let Block::Free(free_size) = self {
            if let Block::File(file_size, file_id) = other {
                let transfer_size = (*free_size).min(*file_size);
                *free_size -= transfer_size;
                *file_size -= transfer_size;

                return Some(Block::File(transfer_size, *file_id));
            }
        }

        None
    }

    /// compute the checksum
    fn get_checksum<'a, I>(iter: I) -> usize
    where
        I: Iterator<Item = &'a Block>,
    {
        let mut i = 0;
        let mut chksm = 0;
        for block in iter {
            match block {
                Block::Free(size) => i += size,
                Block::File(size, id) => {
                    chksm += *id * (i..i + size).sum::<usize>();

                    i += size;
                }
            }
        }

        chksm
    }
}

#[test]
fn test_puzzle1() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"2333133121414131402"#;

    assert_eq!(handle_puzzle1(input)?, 1928);

    Ok(())
}

#[test]
fn test_puzzle2() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"2333133121414131402"#;

    assert_eq!(handle_puzzle2(input)?, 2858);

    Ok(())
}
