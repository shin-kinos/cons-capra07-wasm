
/* These functions are used in Rust program. */

export function console_log( msg )
{
	console.log( "[Rust] console_log() : " + msg );
}

export function error_bomb( msg ) 
{
	let i = true;
	try {
		if( i == true ) {
			throw new Error( msg );
		}
	} catch( e ) {
			//alert( e.message );
			document.getElementById( "outputFile" ).innerHTML
			= e.message;
	}
}
