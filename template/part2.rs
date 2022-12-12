fn main() {
    while let Some(Ok(line)) = std::io::stdin().lines().next() {
        println!("{}", line);
    }
}
