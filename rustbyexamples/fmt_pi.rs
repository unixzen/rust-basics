

fn main() {
    let pi = 3.141592;
    let formatted_pi = format!("{:.*}", 2, pi);
    println!("Pi number {}", formatted_pi);
}
