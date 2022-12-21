use std::env;
use std::fs;

fn getFile() -> String {
    let contents = fs::read_to_string("src/2.txt")
    .expect("Should have been able to read the file");

    return contents;
}

fn p1() {
    let contents = getFile();

    let mut sum = 0;
    for line in contents.lines() {
        let linex = line.split(" ").collect::<Vec<&str>>();
        let opponent = linex[0];
        let me = linex[1];

        match me {
            "X" => //if opponent is X, then I win
                if opponent == "C" {
                    sum += 7;
                    
                } else if opponent == "A" {
                    sum += 4;
                } else if opponent == "B" {
                    sum += 1;
                }

                "Y" => 
                if opponent == "A" {
                    sum += 8;
                } else if opponent == "C" {
                    sum += 2;
                } else if opponent == "B" {
                    sum += 5;
                }

                "Z" =>
                if opponent == "B" {
                    sum += 9;
                } else if opponent == "A" {
                    sum += 3;
                } else if opponent == "C" {
                    sum += 6;
                }

            _ => println!("Error"),
            }
        }
        println!("{}", sum);        
    }


    fn p2() {
        let contents = getFile();
    
        let mut sum = 0;
        for line in contents.lines() {
            let linex = line.split(" ").collect::<Vec<&str>>();
            let opponent = linex[0];
            let me = linex[1];
    
            match opponent {
                "A" => //if im x = lose, y= draw, z = win
                    if me == "X" {
                        //need to lose so im scissors 
                        sum += 3 + 0;

                    } else if me == "Y" { //need to draw so im rock
                        sum += 1 + 3;
                    } else if me == "Z" { //need to win so im Paper
                        sum += 2 + 6;

                    }
                    
                    "B" =>
                    if me == "X" {
                        sum += 1 + 0;
                    } else if me == "Y" {
                        sum += 2 + 3;
                    } else if me == "Z" {
                        sum += 3 + 6;
                    }

                    "C" =>
                    if me == "X" {
                        sum += 2 + 0;
                    } else if me == "Y" {
                        sum += 3 + 3;
                    } else if me == "Z" {
                        sum += 1 + 6;
                    }

                _ => println!("Error"),
            }
               
        }

        println!("{}", sum);     

    }

    fn main() {
        p1();
        p2();
    }


