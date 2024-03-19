fn main() {
    // line count
    let file_name = std::env::args().nth(1).expect("the file name to be passed in");
    let file = std::fs::read_to_string(file_name).expect("unable to read the file to string");
    let mut count: usize = 1;
    let mut temp: Vec<usize> = vec![];
    file.lines().for_each(|line| {
            count += 1;
            temp.push(line.len());
            println!("{}", line);
        println!("{:?}",temp);
        });
    println!("{} lines in file ", count);
    let wc: usize = temp.into_iter().sum();
    println!("{} wc ", wc);
}
