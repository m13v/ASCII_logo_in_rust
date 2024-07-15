use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <TEXT>", args[0]);
        return;
    }

    let input = &args[1];

    let a = r#"
     __    
    /  \   
   / /\ \  
  / ____ \ 
 /_/    \_\
"#;

    let b = r#"
  ____  
 |  _ \ 
 | |_) |
 |  _ < 
 | |_) |
 |____/ 
"#;

    let c = r#"
  _____ 
 / ____|
| |     
| |     
| |____ 
 \_____|
"#;

    let d = r#"
  ____  
 |  _ \ 
 | | | |
 | | | |
 | |_| |
 |____/ 
"#;

    let e = r#"
  ______ 
 |  ____|
 | |__   
 |  __|  
 | |____ 
 |______|
"#;

    let f = r#"
  ______ 
 |  ____|
 | |__   
 |  __|  
 | |     
 |_|     
"#;

    let g = r#"
  _____ 
 / ____|
| |  __ 
| | |_ |
| |__| |
 \_____|
"#;

    let h = r#"
  _    _ 
 | |  | |
 | |__| |
 |  __  |
 | |  | |
 |_|  |_|
"#;

    let i = r#"
  _____ 
 |_   _|
   | |  
   | |  
  _| |_ 
 |_____|
"#;

    let j = r#"
       _ 
      | |
      | |
  _   | |
 | |__| |
  \____/ 
"#;

    let k = r#"
  _  __
 | |/ /
 | ' / 
 |  <  
 | . \ 
 |_|\_\
"#;

    let l = r#"
  _      
 | |     
 | |     
 | |     
 | |____ 
 |______|
"#;

    let m = r#"
  __  __ 
 |  \/  |
 | \  / |
 | |\/| |
 | |  | |
 |_|  |_|
"#;

    let n = r#"
  _   _ 
 | \ | |
 |  \| |
 | . ` |
 | |\  |
 |_| \_|
"#;

    let o = r#"
  _____ 
 / ____|
| |     
| |     
| |____ 
 \_____|
"#;

    let p = r#"
  _____  
 |  __ \ 
 | |__) |
 |  ___/ 
 | |     
 |_|     
"#;

    let q = r#"
  _____ 
 / ____|
| |  __ 
| | |_ |
| |__| |
 \_____\ 
"#;

    let r = r#"
  _____  
 |  __ \ 
 | |__) |
 |  _  / 
 | | \ \ 
 |_|  \_\
"#;

    let s = r#"
  _____ 
 / ____|
| (___  
 \___ \ 
 ____) |
|_____/ 
"#;

    let t = r#"
  _______ 
 |__   __|
    | |   
    | |   
    | |   
    |_|   
"#;

    let u = r#"
  _    _ 
 | |  | |
 | |  | |
 | |  | |
 | |__| |
  \____/ 
"#;

    let v = r#"
 __      __
 \ \    / /
  \ \  / / 
   \ \/ /  
    \  /   
     \/    
"#;

    let w = r#"
 __          __
 \ \        / /
  \ \  /\  / / 
   \ \/  \/ /  
    \  /\  /   
     \/  \/    
"#;

    let x = r#"
 __   __
 \ \ / /
  \ V / 
   > <  
  / . \ 
 /_/ \_\
"#;

    let y = r#"
 __     __
 \ \   / /
  \ \_/ / 
   \   /  
    | |   
    |_|   
"#;

    let z = r#"
  ______
 |___  /
    / / 
   / /  
  / /__ 
 /_____|
"#;

    let dot = r#"
        
         
         
         
  _     
 (_)
"#;

    let letters = [
        ('A', a), ('B', b), ('C', c), ('D', d), ('E', e), ('F', f), ('G', g), ('H', h), ('I', i), ('J', j), ('K', k), ('L', l), 
        ('M', m), ('N', n), ('O', o), ('P', p), ('Q', q), ('R', r), ('S', s), ('T', t), ('U', u), ('V', v), ('W', w), ('X', x), 
        ('Y', y), ('Z', z), ('.', dot)
    ];

    // Split each letter into lines
    let letter_lines: Vec<Vec<&str>> = input.chars().map(|c| {
        letters.iter().find(|&&(ch, _)| ch == c).map(|&(_, art)| art.split('\n').collect()).unwrap_or_else(|| vec![""])
    }).collect();

    // Find the maximum width of any line
    let max_width = letter_lines.iter().flat_map(|lines| lines.iter()).map(|line| line.len()).max().unwrap_or(0);

    // Print each line of the ASCII art horizontally
    for i in 0..letter_lines[0].len() {
        for letter in &letter_lines {
            if i < letter.len() {
                print!("{:<width$}", letter[i], width = max_width);
            } else {
                print!("{:<width$}", "", width = max_width);
            }
        }
        println!();
    }
}