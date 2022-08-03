
use wasm_bindgen::prelude::*;

//use crate::error;
//use crate::error::ErrorType;

use std::process;

#[wasm_bindgen(module="/js/error-bomb.js")]
extern "C" {
	pub fn console_log( msg : &str );
}

#[wasm_bindgen(module="/js/error-bomb.js")]
extern "C" {
	pub fn error_bomb( msg : &str );
}

pub fn check_window_size( window_size : usize, num_site : usize ) {

	if ( window_size * 3 ) > num_site {
		error_bomb( "ERROR !!! 😱\nThe window size is too large.\nProgram halted !!!💥🔨" );
		process::exit( 0 );
	} else if window_size == 0 {
		/* Display window size in a console. */
		let window_size_msg : String = "Window size = ".to_string() + &format!( "{}", window_size );
		console_log( &window_size_msg );
	}

}

pub fn moving_average( cons_capra07_list : &Vec<f64>, window_size : usize ) -> Vec<f64> {

	/* Display window size in a console. */
	let window_size_msg : String = "Window size = ".to_string() + &format!( "{}", window_size );
	console_log( &window_size_msg );

	let mut window_scores : Vec<f64> = ( *cons_capra07_list ).clone();

	for i in ( window_size + 0 ) .. ( ( *cons_capra07_list ).len() - window_size ) {
		let mut window_sum : f64 = 0.0;
		//print!( "Window of {} : ", ( *cons_capra07_list )[ i ] );
		for j in ( i - window_size ) .. ( i + window_size + 1 ) {
			if i != j {
				window_sum += ( *cons_capra07_list )[ j ];
				//print!( "{}, ", cons_capra07_list[ j ] );
			}
		}
		let window_mean : f64 = window_sum / ( window_size * 2 ) as f64;
		window_scores[ i ] = ( window_scores[ i ] * 0.5 ) + ( window_mean * 0.5 );
	}

	window_scores

}