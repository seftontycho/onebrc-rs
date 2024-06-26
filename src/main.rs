use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./measurements.txt").unwrap();
    let mut buff = [0; 1024 * 1024];

    let mut count: usize = 0;

    loop {
        let bytes_read = file.read(&mut buff).unwrap();
        if bytes_read == 0 {
            break;
        }

        for char in &buff[0..bytes_read] {
            if char == &b'\n' {
                count += 1;
            }
        }
    }

    println!("Number of lines: {}", count);
}
