struct Laptop {
    brand: String,
    price: u32,
    stock: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }

    fn display_info(&self) {
        println!(
            "Brand: {} | Price: {} Naira | Stock: {} ",
            self.brand,
            self.price,
            self.stock
        );
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
        stock: 10,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
        stock: 6,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
        stock: 10,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
        stock: 4,
    };

    println!("Laptops available:");
    hp.display_info();
    ibm.display_info();
    toshiba.display_info();
    dell.display_info();

    let total_cost = hp.total_cost(3) + ibm.total_cost(3) + toshiba.total_cost(3) + dell.total_cost(3);

    println!("\nThe total cost for purchasing 3 laptops of each brand is: {} Naira", total_cost);
}
