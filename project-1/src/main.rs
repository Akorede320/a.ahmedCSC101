use std::io;


fn main() {
    println!("Select formula for calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    if input == "1" {
        
        let mut height = String::new();
        let mut base1 = String::new();
        let mut base2 = String::new();

        println!("Enter the height of the trapezium:");
        io::stdin().read_line(&mut height).expect("Failed to read line");
        let height: f32 = height.trim().parse().expect("Please enter a valid number");

        println!("Enter the length of the first base:");
        io::stdin().read_line(&mut base1).expect("Failed to read line");
        let base1: f32 = base1.trim().parse().expect("Please enter a valid number");

        println!("Enter the length of the second base:");
        io::stdin().read_line(&mut base2).expect("Failed to read line");
        let base2: f32 = base2.trim().parse().expect("Please enter a valid number");

        let area = (height / 2.0) * (base1 + base2);
        println!("The area of the trapezium is: {}", area);
    } else if input == "2" {
        
        let mut diagonal1 = String::new();
        let mut diagonal2 = String::new();

        println!("Enter the length of the first diagonal:");
        io::stdin().read_line(&mut diagonal1).expect("Failed to read line");
        let diagonal1: f32 = diagonal1.trim().parse().expect("Please enter a valid number");

        println!("Enter the length of the second diagonal:");
        io::stdin().read_line(&mut diagonal2).expect("Failed to read line");
        let diagonal2: f32 = diagonal2.trim().parse().expect("Please enter a valid number");

        let area = 0.5 * diagonal1 * diagonal2;
        println!("The area of the rhombus is: {}", area);
    } else if input == "3" {
        
        let mut base = String::new();
        let mut altitude = String::new();

        println!("Enter the length of the base:");
        io::stdin().read_line(&mut base).expect("Failed to read line");
        let base: f32 = base.trim().parse().expect("Please enter a valid number");

        println!("Enter the altitude :");
        io::stdin().read_line(&mut altitude).expect("Failed to read line");
        let altitude: f32 = altitude.trim().parse().expect("Please enter a valid number");

        let area = base * altitude;
        println!("The area of the parallelogram is: {}", area);
    } else if input == "4" {
        
        let mut side = String::new();

        println!("Enter the length of one side of the cube:");
        io::stdin().read_line(&mut side).expect("Failed to read line");
        let side: f32 = side.trim().parse().expect("Please enter a valid number");

        let area = 6.0 * side.powi(2);
        println!("The area of the cube is: {}", area);
    } else if input == "5" {
       
        let mut radius = String::new();
        let mut height = String::new();

        println!("Enter the radius of the cylinder:");
        io::stdin().read_line(&mut radius).expect("Failed to read line");
        let radius: f32 = radius.trim().parse().expect("Please enter a valid number");

        println!("Enter the height of the cylinder:");
        io::stdin().read_line(&mut height).expect("Failed to read line");
        let height: f32 = height.trim().parse().expect("Please enter a valid number");

        let volume = std::f32::consts::PI * radius.powi(2) * height;
        println!("The volume of the cylinder is: {}", volume);
    } else {
        println!("Invalid number");
    }
}
