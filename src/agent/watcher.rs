use std::io;
use std::process::Command;

#[allow(unused)]
fn watch(total_iterations: u8) -> Result<(), Box<dyn std::error::Error>> {
    let out_path = "pane.txt";
    let mut line_count = 0;
    let mut i = 0;

    loop {
        if i == total_iterations {
            break;
        }
        // Read input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Command::new(input);

        line_count += 1;

        println!("Line count: {}", line_count);

        if line_count == 25 {
            let pane = crate::agent::panes::Pane::capture();
            println!("{}", pane.content);
            pane.write_to(out_path).unwrap();
            line_count = 0;
            i += 1;
        }
    }

    Ok(())
}
