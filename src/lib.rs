/*
    A very thin wrapper for command line arguments in Rust.
    Author: Sam Saint-Pettersen.
*/

use std::env;

pub struct CliOptions {
	program: String,
}

impl CliOptions {

	/// Create new instance of CliOptions.
	pub fn new(program: &str) -> CliOptions {
		CliOptions {
			program: program.to_string(),
		}
	}

	/// Get any command line arguments.
	pub fn get_args(&self) -> Vec<String> {
        env::args().collect()
	}
    
    	/// Get number fo command line arguments.
    pub fn get_num(&self) -> usize {
        self.get_args().clone().len()
    }

	/// Get next argument.
    pub fn next_argument(&self, i: usize) -> String {
	   let a = env::args().nth(i + 1);
	   let arg = match a {
            Some(a) => a,
            None => String::new()
       };
       arg
	}

    /// Get program name from first argument.
    pub fn get_program(&self) -> String {
    	let a = env::args().nth(0);
    	let arg = match a {
    		Some(a) => a,
    		None => self.program.clone()
    	};
        arg
    }
}
