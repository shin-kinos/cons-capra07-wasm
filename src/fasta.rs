
use wasm_bindgen::prelude::*;
//use std::fs::read_to_string;
//use crate::error;
use std::process;

#[wasm_bindgen(module="/js/error-bomb.js")]
extern "C" {
	pub fn console_log( msg : &str );
}

#[wasm_bindgen(module="/js/error-bomb.js")]
extern "C" {
	pub fn error_bomb( msg : &str );
}

pub struct Fasta {
	pub title_list : Vec<String>,
	pub seq_list   : Vec<String>,
	pub site_list  : Vec<String>,
}

impl Fasta {
	pub fn new() -> Fasta {

		let title_list : Vec<String> = Vec::new();
		let seq_list   : Vec<String> = Vec::new();
		let site_list  : Vec<String> = Vec::new();

		Fasta {
			title_list : title_list,
			seq_list   : seq_list,
			site_list  : site_list,
		}
	}

	pub fn read_fasta_info( &mut self, arg_i : &Vec<&str> /* &String */ ) {

		//let fin = read_to_string( ( *arg_i ).as_str() ).expect( "FAILED to open input file" );

		/* Temporary String to conbine a sequence line separated by "\n" */
		let mut segment : Vec<String> = Vec::new();

		for line in ( *arg_i ).iter() /* fin.lines() */ {
			if line.starts_with( ">" ) && segment.is_empty() {
				( self.title_list ).push( line.to_string() );
			} else if line.starts_with( ">" ) && !segment.is_empty() {
				( self.title_list ).push( line.to_string() );
				( self.seq_list ).push( segment.concat() );
				segment.clear();
			} else {
				segment.push( line.to_string() );
			}
		}
		( self.seq_list ).push( segment.concat() );
		segment.clear();
		segment.shrink_to_fit();
		
		( self.title_list ).shrink_to_fit();
		( self.seq_list   ).shrink_to_fit();
		( self.site_list  ).shrink_to_fit();
	}

	pub fn check_fasta_info( &mut self, arg_t : &String ) {

		let num_title : usize = ( self.title_list ).len();
		let num_seq   : usize = ( self.seq_list   ).len();

		/**/
		for i in 0 .. num_seq {
			let sequence : &String = &( self.seq_list[ i ] );
			if      *arg_t == "yes" { self.seq_list[ i ] = convert_to_gap( sequence, i + 1 ); }
			else if *arg_t == "no"  { check_symbol( sequence /*, i + 1 */ ); }
		}

		/**/
		if num_seq != num_title {
			/* error:: */ error_bomb( "ERROR !!! 😱\nThe Input file format must be aligned Multi-FASTA format.\nProgram halted !!!💥🔨" );
			process::exit( 1 );
		}

		/**/
		for i in 1 .. num_seq {
			if ( self.seq_list[ 0 ] ).len() != ( self.seq_list[ i ] ).len() {
				/* error:: */ error_bomb( "ERROR !!! 😱\nThe Input file format must be aligned Multi-FASTA format, means all of the sequences must be the same length.\nProgram halted !!!💥🔨" );
				process::exit( 1 );
			}
		}

	}

	pub fn get_site_list( &mut self ) {

		let num_seq  : usize = ( self.seq_list ).len();
		let num_site : usize = ( self.seq_list[ 0 ] ).to_string().len();

		//println!( "Number of the sequences : {}", num_seq );
		//println!( "Number of the sites     : {}", num_site );
		let num_seq_msg : String = "Number of the sequences   = ".to_string() + &( num_seq ).to_string();
		console_log( &num_seq_msg );
		let num_site_msg : String = "Number of the sites       = ".to_string() + &( num_site ).to_string();
		console_log( &num_site_msg );

		let mut site : Vec<String> = Vec::new();

		for i in 0 .. num_site {
			for j in 0 .. num_seq {
				let segment : Vec<char> = ( self.seq_list[ j ] ).chars().collect();
				site.push( segment[ i ].to_string() );
			}
			( self.site_list ).push( site.concat() );
			site.clear();
		}
		//site.shrink_to_fit();
	}

}

fn convert_to_gap( sequence : &String, seq_order : usize ) -> String {

	let mut aa_list : Vec<char> = ( *sequence ).chars().collect();

	for i in 0 .. aa_list.len() {
		let aa : char = aa_list[ i ];
		match aa {
			'A'|'R'|'N'|'D'|'C'|'Q'|'E'|'G'|'H'|'I'|'L'|'K'|'M'|'F'|'P'|'S'|'T'|'W'|'Y'|'V'|'-' => (),
			'B'|'Z'|'X'|'U'|'O' => {
				//println!( "\nNOTE :");
				//println!( "Non-standard residue was observed in sequence {} : '{}'", seq_order, aa );
				//println!( "'{}' was converted into gap.", aa );
				//println!( "" );
				let mut non_std_msg : String = "Non-standard residue was observed in sequence ".to_string() + &( seq_order ).to_string();
				non_std_msg = non_std_msg + " -> ";
				non_std_msg = non_std_msg + &aa.to_string();
				console_log( &non_std_msg );
				aa_list[ i ] = '-';
			},
			_ => {
				//println!( "\nNOTE :" );
				//println!( "Unexpected symbol was observed in sequence {} : '{}'", seq_order, aa );
				//println!( "'{}' was converted into gap.", aa );
				//println!( "" );
				let mut un_exp_msg : String = "Unexpected symbol was observed in sequence ".to_string() + &( seq_order ).to_string();
				un_exp_msg = un_exp_msg + " -> ";
				un_exp_msg = un_exp_msg + &aa.to_string();
				console_log( &un_exp_msg );
				aa_list[ i ] = '-';
			},
		}
	}

	/* Convert Vec<char> into String. */
	aa_list.iter().collect()
}

fn check_symbol( sequence : &String /* , seq_order : usize */ ) {

	let aa_list : Vec<char> = ( *sequence ).chars().collect();

	for i in 0 .. aa_list.len() {
		let aa : char = aa_list[ i ];
		match aa {
			'A'|'R'|'N'|'D'|'C'|'Q'|'E'|'G'|'H'|'I'|'L'|'K'|'M'|'F'|'P'|'S'|'T'|'W'|'Y'|'V'|'-' => (),
			'B'|'Z'|'X'|'U'|'O' => {
				//println!( "\nFATAL :" );
				//println!( "Non-standard residue was observed in sequence {} : '{}'", seq_order, aa );
				//println!( "" );
				/* error:: */ error_bomb( "ERROR !!! 😱\nNon-standard residue was observed in a sequence.\nProgram halted !!!💥🔨" );
				process::exit( 1 );
			},
			_ => {
				//println!( "\nFATAL :" );
				//println!( "Unexpected symbol was observed in sequence {} : '{}'", seq_order, aa );
				//println!( "" );
				/* error:: */ error_bomb( "ERROR !!! 😱\nUnexpected symbol was observed in a sequence.\nProgram halted !!!💥🔨" );
				process::exit( 1 );
			},
		}
	}

}