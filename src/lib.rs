
use wasm_bindgen::prelude::*;

mod bgdistribution;
mod example;
mod entropy;
mod fasta;
mod gap;
mod weighting;

#[wasm_bindgen(module="/js/error-bomb.js")]
extern "C" {
	pub fn console_log( msg : &str );
}

#[wasm_bindgen]
pub fn get_site_list_as_string( file_input : &str, opt_t : &str ) -> String {

	let fin_vec : Vec<&str> = file_input.split( '\n' ).collect();

	/* Read an input file and get FASTA information. */
	let mut data = fasta::Fasta::new();
	data.read_fasta_info( &( fin_vec /* opts.input */ ) );

	/* Check whether the input file is correct FASTA format. */
	data.check_fasta_info( /*&( opts.tolerate )*/ &( opt_t.to_string() ) /*&("yes".to_string())*/ );

	/* Get site information as Vec<String>. */
	data.get_site_list();

	//println!( "Number of the sequences : {}", num_seq );
	//println!( "Number of the sites     : {}", num_site );

	let num_site : usize = ( data.site_list ).len();

	let mut site_string : String = String::new();
	for i in 0 .. num_site {
		if i == num_site {
			site_string.push_str( &( data.site_list )[ i ] );
		} else {
			site_string.push_str( &( data.site_list )[ i ] );
			site_string.push_str( "\n" );
		}
	}

	site_string

}

#[wasm_bindgen]
pub fn calc_cons_capra07(
	file_input : &str,
	opt_t      : &str,
	opt_w      : &str,
	opt_b      : &str
) -> String {

	let fin_vec : Vec<&str> = file_input.split( '\n' ).collect();

	/* Read an input file and get FASTA information. */
	let mut data = fasta::Fasta::new();
	data.read_fasta_info( &( fin_vec /* opts.input */ ) );

	/* Check whether the input file is correct FASTA format. */
	data.check_fasta_info( /*&( opts.tolerate )*/ &( opt_t.to_string() ) /*&("yes".to_string())*/ );

	/* Get site information as Vec<String>. */
	data.get_site_list();

	/*
	for i in 0 .. ( data.site_list ).len() {
		let mut site_msg : String = "Site ".to_string() + &( i + 1 ).to_string();
		site_msg += " : ";
		site_msg += &( ( data.site_list )[ i ] );
		console_log( &site_msg );
	}
	*/

	/* Sequence weighting. */
	let weight_list : Vec<f64> = weighting::seq_weight(
		&( data.seq_list     ),
		&( data.site_list    ),
		&( opt_w.to_string() )
	);

	/*
	for i in 0 .. weight_list.len() {
		let mut weight_msg : String = "Weighting factor of sequence ".to_string() + &( i + 1 ).to_string();
		weight_msg += " = ";
		weight_msg += &( weight_list[ i ] ).to_string();
		console_log( &weight_msg );
	}
	*/

	/* Calculate gap penalties taking acconut of sequence weighting. */
	let gap_pen_list : Vec<f64> = gap::weight_gap_penalty( &( data.site_list ), &weight_list );

	/*
	for i in 0 .. gap_pen_list.len() {
		let mut gap_msg : String = "Gap nenalty of site ".to_string() + &( i + 1 ).to_string();
		gap_msg += " = ";
		gap_msg += &( gap_pen_list[ i ] ).to_string();
		console_log( &gap_msg );
	}
	*/

	let cons_capra07_list : Vec<f64> = entropy::js_divergence(
		&( data.site_list ),
		&weight_list,
		&gap_pen_list,
		&( opt_b.to_string() )
	);


	/*
	for i in 0 .. cons_capra07_list.len() {
		let mut cons_msg : String = "Score of site ".to_string() + &( i + 1 ).to_string();
		cons_msg += " = ";
		cons_msg += &( cons_capra07_list[ i ] ).to_string();
		console_log( &cons_msg );
	}
	*/
	
	let score_string : String = get_rsult_as_string( &cons_capra07_list );

	score_string
}

fn get_rsult_as_string( cons_capra07_list : &Vec<f64> ) -> String {

	let num_score : usize = ( *cons_capra07_list ).len();

	let mut score_string : String = String::new();
	for i in 0 .. num_score {
		let score_rounded : String = format!( "{:.3}", ( *cons_capra07_list )[ i ] ); 
		if i == num_score {
			score_string += &score_rounded;
		} else {
			score_string += &score_rounded;
			score_string += "\n";
		}
	}

	score_string

}
