use std::io::Write;
use std::io::Read;


fn main() {
    let mut file = std::fs::File::create("PAU-SMIS.txt").expect("create failed");

        let stud_name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
        let mat_num = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
        let department = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
        let level = ["300", "100", "200", "200", "100"];

            file.write_all("                                                PAU SMIS\n\n".as_bytes()).expect("write failed");
            let _ = file.write_all(b"STUDENT NAME                   MATRIC. NO                     DEPARTMENT                    LEVEL\n");

            for i in 0..stud_name.len() 
            {
                let index = i as usize;
                file.write_all(stud_name[index].as_bytes()).expect("write failed");
                file.write_all("                    ".as_bytes()).expect("write failed");
                file.write_all(mat_num[index].as_bytes()).expect("write failed");
                file.write_all("                    ".as_bytes()).expect("write failed");
                file.write_all(department[index].as_bytes()).expect("write failed");
                file.write_all("                   ".as_bytes()).expect("write failed");
                file.write_all(level[index].as_bytes()).expect("write failed");
                file.write_all("\n".as_bytes()).expect("write failed");

             }

             let mut contents = String::new();
    let mut files = std::fs::File::open("PAU-SMIS.txt").expect("opening failed");
    files.read_to_string(&mut contents).expect("read failed");
    println!("{}", contents);




}