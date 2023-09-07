// Rusty_Quiz
// By: Sina Tashakkori, QVLx Labs

use include_crypt::{include_crypt, EncryptedFile};
use zeroize::Zeroize;
use shielded::Shielded;
use dialoguer::{Select, theme::ColorfulTheme};
use console::{style, Style};
use rand::Rng;

fn main() {
  let file: EncryptedFile = include_crypt!("qstn.txt"); // Question file here
  let mut rng = rand::thread_rng(); // Tailor question randomization here

  let mut decrypted_str = match file.decrypt_str(){
	Ok(o) => o,
	Err(e) => { println!("Issue decrypting file: {}",e); return; }
  };

  let mut shielded_str = Shielded::new(decrypted_str.as_bytes().to_vec());
  decrypted_str.zeroize();
  
  // Remove or tailor the titles
  println!("\n\x1b[38;5;154mDojo Security Training\x1b[0m");
  println!("\x1b[38;5;83mAnswer these questions every day, and watch your Security-Fu blossom.\x1b[0m");

  // At this time, four answer options are provided. Can tailor for more/less
  loop{
    let unshielded = shielded_str.unshield();
    let file_str = String::from_utf8_lossy(unshielded.as_ref());
    let file_str_nl = file_str.split("\n").collect::<Vec<&str>>();	
    let qstn_idx: u32 = rng.gen_range(0..60);
    for (idx, _) in file_str_nl.iter().enumerate() {
        if file_str_nl[idx].eq(&qstn_idx.to_string()) {
            let mut crct = 9;
            let qstn = file_str_nl[idx + 1];
            let ans1_spc = file_str_nl[idx + 2].split(" ").collect::<Vec<&str>>();
            let ans2_spc = file_str_nl[idx + 3].split(" ").collect::<Vec<&str>>();
            let ans3_spc = file_str_nl[idx + 4].split(" ").collect::<Vec<&str>>();
            let ans4_spc = file_str_nl[idx + 5].split(" ").collect::<Vec<&str>>();
            if ans1_spc[0].eq("+") { crct = 0; }
            if ans2_spc[0].eq("+") { crct = 1; }
            if ans3_spc[0].eq("+") { crct = 2; }
            if ans4_spc[0].eq("+") { crct = 3; }
            let ans1 = file_str_nl[idx + 2].replace("+ ","").replace("- ","");
            let ans2 = file_str_nl[idx + 3].replace("+ ","").replace("- ","");
            let ans3 = file_str_nl[idx + 4].replace("+ ","").replace("- ","");
            let ans4 = file_str_nl[idx + 5].replace("+ ","").replace("- ","");
            print!("\x1b[38;5;85m{}\x1b[0m\n",qstn);
            let choices = vec![ans1,ans2,ans3,ans4,"Exit".to_string()];
		    let response = match get_selection(choices.clone()) {
                Some(e) => e,
                None => 4,
		    };
            if response.eq(&4) { std::process::exit(0); } // Exit chosen
            if response == crct { println!("\x1b[38;5;14mCORRECT! {}!\x1b[0m\n", &choices[crct]); }
            else { println!("\x1b[38;5;13mIncorrect. Correct answer is {}.\x1b[0m\n", &choices[crct]); }
        }
    }
	file_str.to_string().zeroize();
  }
}

pub fn get_selection<D: std::fmt::Display>(options: Vec<D>) -> std::option::Option<usize> {
  match Select::with_theme(&get_menu_theme_custom("green")).default(0).items(&options).interact_opt() {
    Ok(sel) => return sel, 
    Err(err) => { 
      println!("Issue handling your selection. {}", err);
      return std::option::Option::None;
    }    
  }
}

pub fn get_menu_theme_custom(color: &str) -> ColorfulTheme {
  ColorfulTheme {
    defaults_style: get_style_color_custom(Style::new(), false, color),
    prompt_style: Style::new(),
    prompt_prefix: style("".to_string()),
    prompt_suffix: style("".to_string()),
    success_prefix: style("".to_string()),
    success_suffix: style("".to_string()),
    error_prefix: style("".to_string()),
    error_style: Style::new(),
    hint_style: Style::new(),
    values_style: Style::new(),
    active_item_style: get_style_color_custom(Style::new(), true, color).black(),
    inactive_item_style: get_style_color_custom(Style::new(), false, color),
    active_item_prefix: match color {
      "white" => style(">".to_string()).white().bright().blink(),
      "green" => style(">".to_string()).green().bright().blink(),
             _=> style(">".to_string()).white().bright().blink()
    },
    inactive_item_prefix: style("".to_string()),
    checked_item_prefix: style("".to_string()),
    unchecked_item_prefix: style("".to_string()),
    picked_item_prefix: style("".to_string()),
    unpicked_item_prefix: style("".to_string()),
    inline_selections: true,
  }
}

pub fn get_style_color_custom(style: Style, bg: bool, color: &str) -> Style {
  if bg {
    match color {
      "white" => style.on_white(),
      "green" => style.on_green(),
             _=> style.on_white()
    }
  } else {
    match color {
      "white" => style.white(),
      "green" => style.green(),
             _=> style.white()
    }
  }
}