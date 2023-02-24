/// Use print function get proper output
/// 
/// # Example
/// 
/// ```
/// 
/// print("Hello World!")
/// 
/// Output
/// 
/// +----------------------+
/// |   Hello World!       |
/// +----------------------+
/// 
/// ```


pub fn print(input: &str) {
    if input.contains("\n") {
        let mut lines: Vec<&str> = input
            .split("\n")
            .filter(|a| !a.is_empty())
            .map(|a| a.trim())
            .collect();

        lines.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        println!(
            "+{}+",
            String::from("-").repeat(lines.last().unwrap().len() + 10)
        );

        for line in input
            .split("\n")
            .filter(|a| !a.is_empty())
            .map(|a| a.trim())
        {
            println!(
                "|{}{}{}|",
                String::from(" ").repeat(5),
                line,
                String::from(" ").repeat(lines.last().unwrap().len() - line.len() + 5)
            );
        }
        println!(
            "+{}+",
            String::from("-").repeat(lines.last().unwrap().len() + 10)
        );
    } else {
        println!("+{}+", String::from("-").repeat(input.len() + 10));
        println!(
            "|{}{}{}|",
            String::from(" ").repeat(5),
            input,
            String::from(" ").repeat(5)
        );
        println!("+{}+", String::from("-").repeat(input.len() + 10));
    }
}


