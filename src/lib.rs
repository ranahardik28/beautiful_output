/// # Use print function get proper output
/// 
/// ### the print function accepts four arguments
/// 1 . Input  
/// 2 . padding left  
/// 3 . padding rigth   
/// 4 . message      
/// 5 . border vertical style   
/// 6 . border horizontal style      
/// 7 . border corner style 
/// 
/// ### Example
/// 
/// ```
/// 
/// print("Hello World!",3,7,"Output :-","-","|","+");
/// 
/// Output :-
/// +----------------------+
/// |   Hello World!       |
/// +----------------------+
/// 
/// ```


pub fn print(input: &str,padding_left:usize,padding_rigth:usize,message:&str,border_vertical_style:&str,border_horizontal_style:&str,border_corner_style:&str) {
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
            "{border_corner_style}{}{border_corner_style}",
            String::from(border_vertical_style).repeat(lines.last().unwrap().len() + padding_left + padding_rigth)
        );

        for line in input
            .split("\n")
            .filter(|a| !a.is_empty())
            .map(|a| a.trim())
        {
            println!(
                "{border_horizontal_style}{}{}{}{border_horizontal_style}",
                String::from(" ").repeat(padding_left),
                line,
                String::from(" ").repeat(lines.last().unwrap().len() - line.len() + padding_rigth)
            );
        }
        println!(
            "{border_corner_style}{}{border_corner_style}",
            String::from(border_vertical_style).repeat(lines.last().unwrap().len() + padding_left + padding_rigth)
        );
    } else {
        println!("{border_corner_style}{}{border_corner_style}", String::from(border_vertical_style).repeat(input.len() + padding_left + padding_rigth));
        println!(
            "{border_horizontal_style}{}{}{}{border_horizontal_style}",
            String::from(" ").repeat(padding_left),
            input,
            String::from(" ").repeat(padding_rigth)
        );
        println!("{border_corner_style}{}{border_corner_style}", String::from(border_vertical_style).repeat(input.len() + padding_left +padding_rigth));
    }
}



