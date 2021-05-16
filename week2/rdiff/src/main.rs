use grid::Grid; // For lcs()
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;
use std::cmp;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    // Be sure to delete the #[allow(unused)] line above
    let mut str_vec: Vec<String> = Vec::new();

    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        //println!("line_str:{}", line_str);
        str_vec.push(line_str);
    }
    Ok(str_vec)
}

fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.
    // Be sure to delete the #[allow(unused)] line above

    let m = seq1.len();
    let n = seq2.len();
    let mut grid = Grid::new(m+1, n+1);

    for i in 0..m+1 {
        match grid.set(i, 0, 0) {
            Err(msg) => panic!(msg),
            Ok(_x) => _x,
        };
    }

    for i in 0..n+1 {
        match grid.set(0, i, 0) {
            Ok(_x) => _x,
            Err(msg) => panic!(msg),
        }
    }

    for i in 0..m {
        for j in 0..n {
            if seq1[i] == seq2[j] {
                match grid.set(i+1, j+1, grid.get(i, j).take().unwrap() + 1) {
                    Ok(_x) => _x,
                    Err(msg) => panic!(msg),
                };
            } else {
                match grid.set(i+1, j+1, 
                    cmp::max(grid.get(i+1, j).take().unwrap(), grid.get(i, j+1).take().unwrap())) {
                        Ok(_x) => _x,
                        Err(msg) => panic!(msg),
                };
            }
        }
    }
    grid
}

fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    // Be sure to delete the #[allow(unused)] line above
    if i > 0 && j > 0 && lines1[i-1] == lines2[j-1] {
        print_diff(lcs_table, lines1, lines2, i - 1, j - 1);
        println!(" {}", lines1[i - 1]);
    } else if j > 0 && (i == 0 || lcs_table.get(i, j - 1) >= lcs_table.get(i - 1, j)) {
        print_diff(lcs_table, lines1, lines2, i, j - 1);
        println!("> {}", lines2[j - 1]);
    } else if i > 0 && (j == 0 || lcs_table.get(i, j - 1) < lcs_table.get(i - 1, j)) {
        print_diff(lcs_table, lines1, lines2, i - 1, j);
        println!("< {}", lines1[i - 1]);
    } else {
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    // Be sure to delete the #[allow(unused)] line above
    let vec1: Vec<String> = {
        match read_file_lines(filename1) {
            Ok(v) => v,
            Err(e) => panic!(e),
        }
    };
    let vec2: Vec<String> = {
        match read_file_lines(filename2) {
            Ok(v) => v,
            Err(e) => panic!(e),
        }
    };
    let lcs_table: Grid = lcs(&vec1, &vec2);

    print_diff(&lcs_table, &vec1, &vec2, vec1.len(), vec2.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
