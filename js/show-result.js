
function color_palette( opt_C )
{
	var color_hex_list = "";
	for ( var i = 0; i < 6; i++ ) {
		if      ( opt_C == "thermo"      ) { color_hex_list = [ '#88B840', '#C3D323', '#F9EC35', '#ED8D1F', '#E86523', '#E34017' ]; }
		else if ( opt_C == "ocean"       ) { color_hex_list = [ '#90EE90', '#67D89A', '#34BEA5', '#1EA9AC', '#1D80AF', '#1C61B1' ]; }
		else if ( opt_C == "flame"       ) { color_hex_list = [ '#FEF001', '#FFCE03', '#FD9A01', '#FD6104', '#FF2C05', '#F00505' ]; }
		else if ( opt_C == "neon"        ) { color_hex_list = [ '#02F2F5', '#25CFF3', '#4895FF', '#8357EB', '#D63BFE', '#FC2CFF' ]; }
		else if ( opt_C == "vaporwave"   ) { color_hex_list = [ '#FFFF02', '#FED825', '#FFA15A', '#FF6496', '#FE2BCC', '#EC02E2' ]; }
	}

	for ( var i = 0; i < 6; i++ ) {
		color_hex_list[ i ] = '<span style="color:#000000; background-color:' + color_hex_list[ i ] + '";>';
	}

	return color_hex_list;
}

function colorize_score( score_str, opt_C )
{
	let score = Number( score_str );
	//console.log( typeof( score ) );

	let color_hex_list = color_palette( opt_C );
	//console.log( color_hex_list );

	var colorized_score = "";
	if      ( 0.000 <= score && score < 0.167 ) { colorized_score += color_hex_list[ 0 ] + score_str + '</span>'; }
	else if ( 0.167 <= score && score < 0.334 ) { colorized_score += color_hex_list[ 1 ] + score_str + '</span>'; }
	else if ( 0.334 <= score && score < 0.501 ) { colorized_score += color_hex_list[ 2 ] + score_str + '</span>'; }
	else if ( 0.501 <= score && score < 0.668 ) { colorized_score += color_hex_list[ 3 ] + score_str + '</span>'; }
	else if ( 0.668 <= score && score < 0.835 ) { colorized_score += color_hex_list[ 4 ] + score_str + '</span>'; }
	else if ( 0.835 <= score                  ) { colorized_score += color_hex_list[ 5 ] + score_str + '</span>'; }
	else                                        { colorized_score += score_str;                                   }


	return colorized_score;

}

function colorize_site( site )
{
	let site_vec = Array.from( site );

	var colorized_site = "";
	for ( var i = 0; i < site_vec.length; i++ ) {
		if      ( site_vec[ i ] == 'A' ) { colorized_site += '<span style="background-color:#FFD615;">' + site_vec[ i ] + '</span>'; } // Aliphatic ( yellow )
		else if ( site_vec[ i ] == 'V' ) { colorized_site += '<span style="background-color:#FFD615;">' + site_vec[ i ] + '</span>'; } // Aliphatic ( yellow )
		else if ( site_vec[ i ] == 'L' ) { colorized_site += '<span style="background-color:#FFD615;">' + site_vec[ i ] + '</span>'; } // Aliphatic ( yellow )
		else if ( site_vec[ i ] == 'I' ) { colorized_site += '<span style="background-color:#FFD615;">' + site_vec[ i ] + '</span>'; } // Aliphatic ( yellow )
		else if ( site_vec[ i ] == 'M' ) { colorized_site += '<span style="background-color:#FFD615;">' + site_vec[ i ] + '</span>'; } // Aliphatic ( yellow )
		else if ( site_vec[ i ] == 'C' ) { colorized_site += '<span style="background-color:#FFD615;">' + site_vec[ i ] + '</span>'; } // Aliphatic ( yellow )
		else if ( site_vec[ i ] == 'F' ) { colorized_site += '<span style="background-color:#05DFD7;">' + site_vec[ i ] + '</span>'; } // Aromatic  ( cyan   )
		else if ( site_vec[ i ] == 'W' ) { colorized_site += '<span style="background-color:#05DFD7;">' + site_vec[ i ] + '</span>'; } // Aromatic  ( cyan   )
		else if ( site_vec[ i ] == 'Y' ) { colorized_site += '<span style="background-color:#05DFD7;">' + site_vec[ i ] + '</span>'; } // Aromatic  ( cyan   )
		else if ( site_vec[ i ] == 'H' ) { colorized_site += '<span style="background-color:#05DFD7;">' + site_vec[ i ] + '</span>'; } // Aromatic  ( cyan   )
		else if ( site_vec[ i ] == 'S' ) { colorized_site += '<span style="background-color:#54E346;">' + site_vec[ i ] + '</span>'; } // Polor     ( green  )
		else if ( site_vec[ i ] == 'T' ) { colorized_site += '<span style="background-color:#54E346;">' + site_vec[ i ] + '</span>'; } // Polor     ( green  )
		else if ( site_vec[ i ] == 'N' ) { colorized_site += '<span style="background-color:#54E346;">' + site_vec[ i ] + '</span>'; } // Polor     ( green  )
		else if ( site_vec[ i ] == 'Q' ) { colorized_site += '<span style="background-color:#54E346;">' + site_vec[ i ] + '</span>'; } // Polor     ( green  )
		else if ( site_vec[ i ] == 'K' ) { colorized_site += '<span style="background-color:#4B7BE5;">' + site_vec[ i ] + '</span>'; } // Positive  ( blue   )
		else if ( site_vec[ i ] == 'R' ) { colorized_site += '<span style="background-color:#4B7BE5;">' + site_vec[ i ] + '</span>'; } // Positive  ( blue   )
		else if ( site_vec[ i ] == 'D' ) { colorized_site += '<span style="background-color:#FF6363;">' + site_vec[ i ] + '</span>'; } // Negative  ( red    )
		else if ( site_vec[ i ] == 'E' ) { colorized_site += '<span style="background-color:#FF6363;">' + site_vec[ i ] + '</span>'; } // Negative  ( red    )
		else if ( site_vec[ i ] == 'G' ) { colorized_site += '<span style="background-color:#B983FF;">' + site_vec[ i ] + '</span>'; } // Special conformations ( magenta )
		else if ( site_vec[ i ] == 'P' ) { colorized_site += '<span style="background-color:#B983FF;">' + site_vec[ i ] + '</span>'; } // Special conformations ( magenta )
		else                             { colorized_site += site_vec[ i ]; }
	}

	return colorized_site;
}

export function show_result(
	result_cons_str,
	result_site_str,
	opt_C,
	opt_c )
{
	let result_cons_list = result_cons_str.split(/\r\n|\n/);
	let result_site_list = result_site_str.split(/\r\n|\n/);
	let num_site         = result_site_list.length;

	var result_str = "Number" + '\t' + "Score" + '\t' + "Site" + '<br><br>';

	//document.getElementById( "outputFile" ).innerHTML = "";
	for ( var i = 0; i < num_site - 1; i++ ) {
		let num = i + 1;
		//document.getElementById( "outputFile" ).innerHTML += result_site_list[ i ] + '<br>';
		result_str += num + '\t';

		if ( opt_C == "none" ) { result_str += result_cons_list[ i ];                          }
		else                   { result_str += colorize_score( result_cons_list[ i ], opt_C ); }

		result_str += '\t';

		/* Whether or not colorize the compsition of site. */
		if      ( opt_c == "yes" ) { result_str += colorize_site( result_site_list[ i ] ); }
		else if ( opt_c == "no"  ) { result_str += result_site_list[ i ];                  }
		else                       { result_str += result_site_list[ i ];                  }

		result_str += '<br>';
	}

	document.getElementById( "outputFile" ).innerHTML = result_str;

}

/*
<span style="color:# ; font-size:15px;">■</span>
<span style="color:# ; font-size:15px;">■</span>
<span style="color:# ; font-size:15px;">■</span>
<span style="color:# ; font-size:15px;">■</span>
<span style="color:# ; font-size:15px;">■</span>
<span style="color:# ; font-size:15px;">■</span>
*/