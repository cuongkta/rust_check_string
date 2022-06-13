use std::io;

fn main() {
   let main_str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
   println!("Please input your text.");
   let mut guess = String::new();
   io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
   let input_text: &str = remove_suffix(&guess, "\r\n");
   println!("{:?}", input_text );
   let main_str = main_str.to_lowercase();
   let input_text = input_text.to_lowercase();
   let number_matching : usize = main_str.matches(&input_text).count();
   println!("the numbers of matching {}", number_matching);

}

fn remove_suffix<'a>(mystring: &'a str, suffix: &'a str) -> &'a str { 

    match mystring.strip_suffix(suffix) {
        Some(mystring) => mystring,
        None => mystring
    }
}



