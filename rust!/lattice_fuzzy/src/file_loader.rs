use std::fs::read_to_string;
use std::io::Write;
use std::fs::File;
use std::io;

fn parse_vector(input: &str) -> Vec<f64> {
    input
        .split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect()
}

pub fn get_vectors_from_file(filename: &str) -> Vec<Vec<f64>>{
    let mut out: Vec<Vec<f64>> = vec![];
    let binding = match read_to_string(filename) {
        Ok(content) => content,
        Err(_e) => {
            return out;
        }
    };

    let file = binding.lines();
    for line in file {
        let x = parse_vector(line);
        out.push(x);
    }

    return out;
}

pub fn make_file_from_i32_vec(data: Vec<i32>, path: &str) -> io::Result<()>  {
    let mut file = File::create(path)?;
    for n in data {
        writeln!(file, "{}", n)?;
    }

    Ok(())
}

pub fn write_tuples_to_file(tuples: Vec<(f64, f64, f64)>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    for (x, y, z) in tuples {
        writeln!(file, "{:.6} {:.6} {:.6}", x, y, z)?;
    }
    Ok(())
}
