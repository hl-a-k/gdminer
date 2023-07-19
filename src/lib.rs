use std::fs::File;
use std::io::{BufRead, BufReader};

const CNT: usize = 100;
const MATRIX_CNT: usize = (CNT - 1) * CNT / 2;

fn swap(arr: &mut [u128; MATRIX_CNT], idx: i32, div: i32) {
    let tmp_swap_val: u128;
    tmp_swap_val = arr[idx as usize];
    arr[idx as usize] = arr[(idx + div) as usize];
    arr[(idx + div) as usize] = tmp_swap_val;
}

fn rebuild_matrix(
    pre: &mut [u128; CNT],
    pre_matrix: &mut [u128; MATRIX_CNT],
    idx: usize,
    newval: u128,
) {
    let need_rm = pre[(idx - CNT) % CNT];
    pre[idx % CNT] = newval;

    for i in 0..(CNT - 1) {
        let oldsum = pre[(idx - i - 1) % CNT] + need_rm;
        if let Ok(mut idx) = pre_matrix.binary_search(&oldsum) {
            pre_matrix[idx] = oldsum - need_rm + newval;
            loop {
                if idx <= 0
                    || idx >= MATRIX_CNT - 1
                    || (pre_matrix[idx] >= pre_matrix[idx - 1]
                        && pre_matrix[idx] <= pre_matrix[idx + 1])
                {
                    break;
                } else if pre_matrix[idx] < pre_matrix[idx - 1] {
                    swap(pre_matrix, idx as i32, -1);
                    idx -= 1;
                } else if pre_matrix[idx] > pre_matrix[idx + 1] {
                    swap(pre_matrix, idx as i32, 1);
                    idx += 1;
                }
            }
        }
    }
}

pub fn is_safe(path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut pre: [u128; CNT] = [0; CNT];
    let mut pre_matrix: [u128; MATRIX_CNT] = [0; MATRIX_CNT];
    let mut idx = 0;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let newval: u128 = line.parse::<u128>()?;
            if idx < CNT {
                pre[idx] = newval;
            } else if idx == CNT {
                let mut k = 0;
                for i in 0..(CNT - 1) {
                    for j in (i + 1)..CNT {
                        pre_matrix[k] = pre[i] + pre[j];
                        k += 1;
                    }
                }
                pre_matrix.sort();
                if let Err(_) = pre_matrix.binary_search(&newval) {
                    return Ok(false);
                }
                rebuild_matrix(&mut pre, &mut pre_matrix, idx, newval);
            } else {
                if let Err(_) = pre_matrix.binary_search(&newval) {
                    return Ok(false);
                }
                rebuild_matrix(&mut pre, &mut pre_matrix, idx, newval);
            }
            idx += 1;
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let result = is_safe("challenge_input.txt");
        assert_eq!(result?, false);
        Ok(())
    }
}
