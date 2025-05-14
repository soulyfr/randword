use clap::{Parser};
use rand::seq::IndexedRandom;
use surf;
use tokio;
use colored::*;
use colored::Color;
use rand::rng;

///A simple CLI tool integrating a free random word API
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// number of words to return | MAX 255
    #[arg(short,long)]
    number: Option<u8>,
    /// length of word(s) to return
    #[arg(long)]
    len: Option<u8>,
    /// alphabetical sorting - [ASC] | desc
    #[arg(short,long, default_value_t = String::from("asc"))]
    sort: String,
    
    //styling
    /// returns words sep. by 1 space, no line break
    #[arg(long)]
    batch: bool,
    /// returns word(s) in uppercase
    #[arg(short,long)]
    upper: bool,
    /// returns word(s) in bold
    #[arg(short,long)]
    bold: bool,
    /// red | green | blue | yellow | purple | random
    #[arg(short,long)]
    color: Option<String>,
    /// columns - 1 to 10 works well depending on zoom | DEFAULT 1
    #[arg(long, default_value_t = 1)]
    cols: i8
}

static COLORS : &[Color] = &[
    Color::Red,
    Color::Green,
    Color::Blue,
    Color::Yellow,
    Color::Magenta
];

async fn fetch_data(uri : &str) -> Result<Vec<String>, surf::Error> {

    let mut res: surf::Response  = surf::get(uri).await?;

    let response: Vec<String>  = res.body_json().await?;

    Ok(response)
}

#[tokio::main]
async fn main() {

    let mut base_url = String::from("https://random-word-api.vercel.app/api?");
    let args: Args = Args::parse();
    let mut rng = rng();

    if let Some(amount) = args.number {
        base_url.push_str(&format!("&words={}", amount));
    }

    if let Some(length) = args.len {
        base_url.push_str(&format!("&length={}", length));
    }

    let upper = if args.upper {true} else {false};

    match fetch_data(&base_url).await {
        Ok(mut words) => {
            
            let mut count= 0;

            if words.len() > 0 {

                let mut pivot = 0;

                if words.len() > 1 {
                    words.sort();

                    if let Some(longest_word) = words.iter().max_by_key(|&word| word.len()) {
                        pivot = longest_word.len();
                    }
                }
                if args.sort == String::from("desc") {
                    words.reverse();
                }

                for word in &words {
                    let mut returnedword;
                    if upper {
                        returnedword = word.to_uppercase().normal();
                    } else {
                        returnedword = word.to_string().normal();
                    }



                    if args.bold {
                        returnedword = returnedword.bold();
                    }

                    if let Some(ref color) = args.color {
                        match color.to_lowercase().as_str() {
                            "red" => returnedword = returnedword.red(),
                            "green" => returnedword = returnedword.green(),
                            "blue" => returnedword = returnedword.blue(),
                            "yellow" => returnedword = returnedword.yellow(),
                            "purple" => returnedword = returnedword.magenta(),
                            "random" => {
                                let random_color = COLORS.choose(&mut rng).unwrap();
                                returnedword = returnedword.color(*random_color);
                            }
                            _ => {}
                        }
                    }

                    if args.batch {
                        print!("{} ", returnedword)
                    } else {
                        if words.len() > 1 {
                            // print!("{}", pivot -returnedword.len() + 3);
                            print!("{}{}", returnedword, " ".repeat(pivot - returnedword.len() + 1));
                        }
                        else {
                            println!("{}", returnedword);
                        }
                        count += 1;

                        if count == args.cols {
                            println!();
                            count = 0;
                        }
                    }
                }
                if count != 0 || args.batch {
                    println!();
                }
            }
            // if let Some(word) = words.get(0) {
            //     println!("word: {}", word);
            // }
            
            // dogs_data = data.data
        }
        Err(err) => {
            eprintln!("couldnt fetch data, sorry :P - {}", err)
        }
    }
    // println!("sup {}", args.name);
    
}
