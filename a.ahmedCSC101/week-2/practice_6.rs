fn main() {
  let numbers = vec!(450000.0,1500000.0,750000.0,2850000.0,250000.0);

  let sum: f64 = numbers.iter().sum();
    let count = numbers.len();

let average = if count > 0 {
        sum as f64 / count as f64
    } else {
        0.0
    };
    println!("Sum: {}", sum);
    println!("Average: {}", average);
}