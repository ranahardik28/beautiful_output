/// # Use print function get proper output
/// 
/// ### the print function accepts four arguments
/// 1 . Input  
/// 2 . padding left  
/// 3 . padding rigth  
/// 4 . message  
/// 
/// ### Example
/// 
/// ```
/// 
/// print("Hello World!",3,7,"Output :-");
/// 
/// Output :-
/// +----------------------+
/// |   Hello World!       |
/// +----------------------+
/// 
/// ```


pub fn print(input: &str,padding_left:usize,padding_rigth:usize,message:&str) {
    if !message.is_empty() {
        println!("{}",message);
    }

    if input.contains("\n") {
        let mut lines: Vec<&str> = input
            .split("\n")
            .filter(|a| !a.is_empty())
            .map(|a| a.trim())
            .collect();

        lines.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        println!(
            "+{}+",
            String::from("-").repeat(lines.last().unwrap().len() + padding_left + padding_rigth)
        );

        for line in input
            .split("\n")
            .filter(|a| !a.is_empty())
            .map(|a| a.trim())
        {
            println!(
                "|{}{}{}|",
                String::from(" ").repeat(padding_left),
                line,
                String::from(" ").repeat(lines.last().unwrap().len() - line.len() + padding_rigth)
            );
        }
        println!(
            "+{}+",
            String::from("-").repeat(lines.last().unwrap().len() + padding_left + padding_rigth)
        );
    } else {
        println!("+{}+", String::from("-").repeat(input.len() + padding_left + padding_rigth));
        println!(
            "|{}{}{}|",
            String::from(" ").repeat(padding_left),
            input,
            String::from(" ").repeat(padding_rigth)
        );
        println!("+{}+", String::from("-").repeat(input.len() + padding_left +padding_rigth));
    }
}


