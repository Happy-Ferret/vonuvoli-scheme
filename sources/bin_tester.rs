

#![ no_implicit_prelude ]

#[ macro_use ]
extern crate vonuvoli_scheme;

use vonuvoli_scheme::exports::*;
use vonuvoli_scheme::prelude::*;

def_transcript_root! (transcript);




fn main () -> () {
	execute_main (main_0, &transcript);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn main_0 () -> (Outcome<u32>) {
	
	let arguments = env::args () .collect::<StdVec<_>> ();
	let (identifier, source_path) = match arguments.len () {
		0 =>
			("<stdin>", None),
		1 =>
			("<stdin>", None),
		2 =>
			(arguments[1].as_str (), Some (&arguments[1])),
		_ =>
			fail! (0x9f085526),
	};
	
	let context = Context::new (None);
	try! (context.define_all (try! (language_r7rs_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all (try! (language_builtins_generate_binding_templates ()) .as_ref ()));
	try! (context.define_all_with_prefix (try! (language_builtins_generate_binding_templates ()) .as_ref (), Some ("~")));
	
	let mut source = StdString::new ();
	match
		if let Some (source_path) = source_path {
			let mut source_stream = try_or_fail! (fs::File::open (source_path), 0xa9216a08);
			source_stream.read_to_string (&mut source)
		} else {
			let mut source_stream = io::stdin ();
			source_stream.read_to_string (&mut source)
		}
	{
		Ok (_) =>
			(),
		Err (error) => {
			trace_error! (transcript, 0x367496ef => "failed loading script!" => (), error = &error);
			succeed! (1);
		},
	}
	
	match execute_tests_main (identifier, &source, Some (&context), Some (transcript.backend ()), None) {
		Ok (()) =>
			succeed! (0),
		Err (error) => {
			trace_error! (transcript, 0x367496ef => "failed testing script!" => (), error = &error);
			error.backtrace_report (tracer_error! (transcript, 0x5681c4ca));
			succeed! (1);
		},
	}
}

