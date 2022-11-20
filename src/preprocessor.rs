pub fn first_read(raw_lines: Vec<&str>) -> Vec<&str> {
    // let mut raw_lines: Vec<&str> = file_contents.lines().collect();

    println!("lines: ");
    
    for i in 0..raw_lines.len() {
        println!("{}", raw_lines[i]);
    }

    return raw_lines;
}