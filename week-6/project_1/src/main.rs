use std::io;

fn main() {
    let f1 = "            Main                                      Price ".to_string();
    let f2 = " P = Poundo Yam/Edinkaiko Soup                      - N3,200".to_string();
    let f3 = " F = Fried Rice & Chicken                           - N3,000".to_string();
    let f4 = " A = Amala & Ewedu Soup                             - N2,500".to_string();
    let f5 = " E = Eba & Egusi Soup                               - N2,000".to_string();
    let f6 = " W = White Rice & Stew                              - N2,500".to_string();

    let text = format!("{} \n{} \n{} \n{} \n{} \n{}",f1, f2, f3, f4, f5, f6);
    println!("{}", text);

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What type of food do you want: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let food = input1.trim().to_uppercase();

    println!("Enter the quantity you want: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:f32 = input2.trim().parse().expect("Not a valid number");

    if food == "P"{
        let price:f32 = 3200.0 * quantity;
        if price > 10000.0 {
            let final_price:f32 = price - (price * 5.0 )/100.0;
            println!("Your discount total is: N{}", final_price);
        } else {
            println!("Your total is: N{}", price);
        }
    } else if food == "F"{
        let price:f32 = 3000.0 * quantity;
        if price > 10000.0 {
            let final_price:f32 = price - (price * 5.0 )/100.0;
            println!("Your discount total is: N{}", final_price);
    } else {
        println!("Your total is: N{}", price); 
    }
} else if food == "A"{
    let price:f32 = 2500.0 * quantity;
    if price > 10000.0 {
        let final_price:f32 = price - (price * 5.0 )/100.0;
        println!("Your discount total is: N{}", final_price);
} else {
    println!("Your total is: N{}", price); 
}
} else if food == "E"{
    let price:f32 = 2000.0 * quantity;
    if price > 10000.0 {
        let final_price:f32 = price - (price * 5.0 )/100.0;
        println!("Your discount total is: N{}", final_price);
} else {
    println!("Your total is: N{}", price); 
}
} else if food == "W"{
    let price:f32 = 2500.0 * quantity;
    if price > 10000.0 {
        let final_price:f32 = price - (price * 5.0 )/100.0;
        println!("Your discount total is: N{}", final_price);
} else {
    println!("Your total is: N{}", price); 
}
} else {
    println!("Invalid food item");
}
}