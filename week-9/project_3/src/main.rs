use std::io::Write;
use std::io::Read;


fn main() {
    let mut file = std::fs::File::create("Convicted Ministers.txt").expect("create failed");

        let names = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
        let ministry = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
        let geo_zone = vec!["South West", "North East", "South South", "South West", "South East"];

            file.write_all("                                      CONVICTED MINISTERS BY EFCC\n\n".as_bytes()).expect("write failed");
            let _ = file.write_all(b"NAME OF COMMISSIONER                   MINISTRY                     GEOPOLITICAL ZONE\n");

            for i in 0..names.len() 
            {
                let index = i as usize;
                file.write_all(names[index].as_bytes()).expect("write failed");
                file.write_all("                    ".as_bytes()).expect("write failed");
                file.write_all(ministry[index].as_bytes()).expect("write failed");
                file.write_all("                    ".as_bytes()).expect("write failed");
                file.write_all(geo_zone[index].as_bytes()).expect("write failed");
                file.write_all("\n".as_bytes()).expect("write failed");

             }

    let mut contents = String::new();
    let mut files = std::fs::File::open("Convicted Ministers.txt").expect("opening failed");
    files.read_to_string(&mut contents).expect("read failed");
    println!("{}", contents);

    println!("The above is the merged dataset!");


}
