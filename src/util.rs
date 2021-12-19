use std::io;

// TODO: Can't get this function to properly work
// Currently, I'm grabbing input and have the server tell me whether or
// not input was validated. What I want to have happen is if there's
// a failure in input validation, I want the program to tell right away
// not have the server tell me when I'm doen inputting values
pub fn get_input(question: String) -> String {
    let result: String;
    loop {
        let mut input = String::new();
        println!("{}", &question);
        io::stdin().read_line(&mut input).unwrap();

        match input.len() {
            0 => println!("{}", "Sorry, that wasn't valid input. Please try again.\n"),
            _ => {
                result = input.trim().to_string();
                break;
            }
        }
    }   
    result
}