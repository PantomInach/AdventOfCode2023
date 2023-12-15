fn hash(s: &str) -> u64 {
    s.chars().fold(0_u64, |acc, c| (acc + c as u64) * 17 % 256)
}

pub fn process_part1(input: &str) -> u64 {
    input.trim().split(',').map(hash).sum()
}

pub fn process_part2(input: &str) -> u64 {
    let mut boxes: [Vec<(&str, u64)>; 256] = vec![Vec::new(); 256]
        .try_into()
        .expect("Can't create LinkedLists");
    input.trim().split(',').for_each(|s| {
        if let Some(label) = s.strip_suffix('-') {
            let label = label;
            let b_index = hash(label) as usize;
            boxes.get_mut(b_index).unwrap().retain(|x| x.0 != label);
        } else {
            let label = &s[..s.len() - 2];
            let b_index = hash(label) as usize;
            let focal: u64 = s.chars().nth(s.len() - 1).unwrap() as u64 - '0' as u64;
            let b = boxes.get_mut(b_index).unwrap();
            if let Some(p) = b.iter().position(|lens| lens.0 == label) {
                b[p] = (label, focal);
            } else {
                b.push((label, focal));
            }
        }
    });
    boxes
        .iter()
        .enumerate()
        .map(|(b_i, b)| {
            b.iter()
                .enumerate()
                .map(|(l_i, lens)| (b_i as u64 + 1) * (l_i as u64 + 1) * lens.1)
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(1320_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(145_u64, process_part2(input));
    }
}
