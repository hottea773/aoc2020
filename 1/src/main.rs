use std::fs;

fn main() {
    let filename = "src/input.txt";

    println!("In file {}", filename);

    let contents: Vec<u32> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    for ii in contents.iter() {
        for jj in contents.iter() {
            if ii + jj == 2020 {
                println!("ii: {}", ii);
                println!("jj: {}", jj);
                println!("(ii * jj): {}", (ii * jj));
            }
        }
    }

    for ii in contents.iter() {
        for jj in contents.iter() {
            for kk in contents.iter() {
                if ii + jj + kk == 2020 {
                    println!(
                        "ii: {}\njj: {}\nkk: {}\n ii * jj * kk: {}",
                        ii,
                        jj,
                        kk,
                        (ii * jj * kk)
                    );
                }
            }
        }
    }
}
