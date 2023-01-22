use std::{io::{self, Write}, fmt::Display, error::Error};
use std::process;

trait DataType {

}
struct INT ();

impl DataType for INT {

}

impl Display for INT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "INT")
    }
}

struct STRING();

impl DataType for STRING {

}

impl Display for STRING {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "STRING")
    }
}

struct FLOAT();

impl DataType for FLOAT {

}

impl Display for FLOAT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FLOAT")
    }
}

#[derive(PartialEq)]
struct Variable<T, V>
where
    T: DataType
{
    var_type: T,
    var_val: V,
}

impl<T, V> Variable<T, V> 
where
    T: DataType
{
    fn new(var_type: T, var_val: V) -> Result<Variable<T, V>, Box<dyn Error>> {
        Ok(Variable {
            var_type: var_type,
            var_val: var_val,
        })
    }
}

impl Display for Variable<INT, i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type: {}\tValue: {}", self.var_type, self.var_val)
    }
}

impl Display for Variable<FLOAT, f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type: {}\tValue: {}", self.var_type, self.var_val)
    }
}

impl Display for Variable<STRING, String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type: {}\tValue: {}", self.var_type, self.var_val)
    }
}

fn main() -> io::Result<()>{
    println!("vplC - Visual Programming Language");
    println!("==================================");
    println!("==================================");
    
    'main_loop: loop {
        println!("1) VARIABLE");
        println!("2) SEQUENCE");
        println!("3) SELECTION");
        println!("4) ITERATION");
        println!("5) EXIT");

        print!("Enter chosen input: ");
        let _ = io::stdout().flush();
        
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line!");

        let user_input: usize = match user_input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("That is not a number!");
                continue;
            }
        };

        match user_input {
            1 => {
                println!("VARIABLE");
                println!("========");

                // let (t, v) = match get_variable_inputs() {
                //     Ok((t, v)) => (t, v),
                //     Err(e) => {
                //         println!("Error in function, crashing :(\t{}", e);
                //         process::exit(1);
                //     }
                // };
                
            }
            2 => {
                println!("SEQUENCE");
            }
            3 => {
                println!("SELECTION");
            }
            4 => {
                println!("ITERATION");
            }
            5 => {
                println!("EXITING!");
                process::exit(0);
            }
            _ => {
                println!("That is not a correct input!");
                continue;
            }
        }

    }

}

// fn get_variable_inputs() -> (&str, &str) {
//     loop {
//         print!("Enter type: [INT/FLOAT/STRING]");
//         let _ = io::stdout().flush();

//         let mut type_input = String::new();
//         io::stdin().read_line(&mut type_input).expect("Failed to readline!");

//         print!("Enter value: ");
//         let _ = io::stdout().flush();

//         let mut value_input = String::new();
//         io::stdin().read_line(&mut value_input).expect("Failed to readline!");

//         (type_input.trim(), value_input.trim())
//     }

// }

// #[cfg(test)]
// mod tests {
//     use crate::*;

//     #[test]
//     fn test_var() {
//         let integer = Variable::new(INT(), 5);
//         println!("{}", integer);
//     }
// }
