use std::fs::read_to_string;

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