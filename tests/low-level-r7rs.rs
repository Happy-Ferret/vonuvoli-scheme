

#![ feature (test) ]
#![ no_implicit_prelude ]
include! ("prelude.in");




def_test! (test__0, {
	
	
	
	
	let print_all_forced = true;
	let print_all_missing = true;
	
	let print_headers = true;
	let print_definitions_table = true;
	let print_definitions_symbols = true;
	let print_definitions_symbols_list = print_definitions_symbols && print_all_missing;
	
	let print_procedures = print_all_forced || print_all_missing;
	let print_procedure_alternatives = print_all_forced || print_all_missing;
	let print_syntaxes = print_all_forced || print_all_missing;
	let print_values = print_all_forced || print_all_missing;
	
	let print_library_base = print_all_forced || print_all_missing;
	let print_library_ports = print_all_forced || print_all_missing;
	let print_library_miscellaneous = print_all_forced || print_all_missing;
	
	let print_implemented = print_all_forced;
	let print_unimplemented = print_all_forced || print_all_missing;
	let print_unsupported = print_all_forced || print_all_missing;
	let print_reserved = print_all_forced;
	
	let print_implemented_symbols = false;
	let print_reserved_symbols = false;
	
	
	
	
	let definitions = language_r7rs_generate_definitions () .expect ("3bd1d93c");
	
	language_r7rs_verify_definitions (&definitions) .expect ("11e64ae7");
	
	
	
	
	let mut implemented_symbols = StdSet::new ();
	let mut unimplemented_symbols = StdSet::new ();
	let mut unsupported_symbols = StdSet::new ();
	let mut reserved_symbols = StdSet::new ();
	
	
	
	
	if print_definitions_table && print_headers {
		eprintln! ();
		eprintln! ("## Scheme R7RS definitions");
		eprintln! ();
	}
	
	macro_rules! print_definition {
		($library : expr, $category : expr, $type : expr, $identifier : expr, $value : expr) => (
			if print_definitions_table {
				eprintln! ("|  {:^16}  |  {:^12}  |  {:^16}  | `{:<32}` | `{:?}`", $library.string_as_str (), $category.string_as_str (), $type, $identifier.string_as_str (), $value);
			}
		);
	}
	
	if print_definitions_table {
		eprintln! ("|  {:^16}  |  {:^12}  |  {:^16}  |  {:<32}  |  {:}", "Library", "Category", "Type", "Scheme identifier", "Rust value");
		eprintln! ("|  {:^16}  |  {:^12}  |  {:^16}  |  {:<32}  |  {:}", ":---:", ":---:", ":---:", ":---", ":---");
	}
	
	for (library, category, identifier, value) in definitions.into_iter () {
		
		let library_is_ports = category.string_eq ("ports");
		let library_is_base = library.string_eq ("base") && !library_is_ports;
		let library_is_miscellaneous = !library_is_base && !library_is_ports;
		if !(
				(print_library_base && library_is_base) ||
				(print_library_ports && library_is_ports) ||
				(print_library_miscellaneous && library_is_miscellaneous)
		) {
			continue;
		}
		
		match value.kind_match_as_ref () {
			
			ValueKindMatchAsRef::ProcedurePrimitive (primitive) => {
				let primitive = *primitive;
				match primitive {
					
					ProcedurePrimitive::Primitive0 (_) |
					ProcedurePrimitive::Primitive1 (_) |
					ProcedurePrimitive::Primitive2 (_) |
					ProcedurePrimitive::Primitive3 (_) |
					ProcedurePrimitive::Primitive4 (_) |
					ProcedurePrimitive::Primitive5 (_) |
					ProcedurePrimitive::PrimitiveN (_) => {
						let arity = match primitive {
							ProcedurePrimitive::Primitive0 (_) => '0',
							ProcedurePrimitive::Primitive1 (_) => '1',
							ProcedurePrimitive::Primitive2 (_) => '2',
							ProcedurePrimitive::Primitive3 (_) => '3',
							ProcedurePrimitive::Primitive4 (_) => '4',
							ProcedurePrimitive::Primitive5 (_) => '5',
							ProcedurePrimitive::PrimitiveN (_) => 'n',
							_ => panic_0! (0x7e5d3d15),
						};
						if print_procedures && print_implemented {
							print_definition! (library, category, format! ("procedure-{}", arity), identifier, primitive);
						}
						implemented_symbols.insert (identifier);
					},
					
					ProcedurePrimitive::PrimitiveV (primitive_v) => {
						if print_procedures && print_implemented && print_procedure_alternatives {
							let mut has_alternatives = false;
							if let Some (primitive) = procedure_primitive_v_alternative_0 (primitive_v) {
								print_definition! (library, category, "procedure-0*", identifier, ProcedurePrimitive::Primitive0 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_1 (primitive_v) {
								print_definition! (library, category, "procedure-1*", identifier, ProcedurePrimitive::Primitive1 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_2 (primitive_v) {
								print_definition! (library, category, "procedure-2*", identifier, ProcedurePrimitive::Primitive2 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_3 (primitive_v) {
								print_definition! (library, category, "procedure-3*", identifier, ProcedurePrimitive::Primitive3 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_4 (primitive_v) {
								print_definition! (library, category, "procedure-4*", identifier, ProcedurePrimitive::Primitive4 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_5 (primitive_v) {
								print_definition! (library, category, "procedure-5*", identifier, ProcedurePrimitive::Primitive5 (primitive));
								has_alternatives = true;
							}
							if let Some (primitive) = procedure_primitive_v_alternative_n (primitive_v) {
								print_definition! (library, category, "procedure-n*", identifier, ProcedurePrimitive::PrimitiveN (primitive));
								has_alternatives = true;
							}
							if has_alternatives {
								print_definition! (library, category, "procedure-v*", identifier, primitive);
							} else {
								panic_0! (0xed9c74b7);
							}
						} else {
							if print_procedures && print_implemented {
								print_definition! (library, category, "procedure-v", identifier, primitive);
							}
						}
						implemented_symbols.insert (identifier);
					}
					
					ProcedurePrimitive::Unimplemented => {
						if print_procedures && print_unimplemented {
							print_definition! (library, category, "procedure", identifier, primitive);
						}
						unimplemented_symbols.insert (identifier);
					},
					
					ProcedurePrimitive::Unsupported => {
						if print_procedures && print_unsupported {
							print_definition! (library, category, "procedure", identifier, primitive);
						}
						unsupported_symbols.insert (identifier);
					},
					
					ProcedurePrimitive::Reserved => {
						if print_procedures && print_reserved {
							print_definition! (library, category, "procedure", identifier, primitive);
						}
						reserved_symbols.insert (identifier);
					}
					
				}
			},
			
			ValueKindMatchAsRef::SyntaxPrimitive (primitive) => {
				let primitive = *primitive;
				match primitive {
					
					SyntaxPrimitive::PrimitiveV (_) => {
						if print_syntaxes && print_implemented {
							print_definition! (library, category, "syntax", identifier, primitive);
						}
						implemented_symbols.insert (identifier);
					}
					
					SyntaxPrimitive::Auxiliary  => {
						if print_syntaxes && print_implemented {
							print_definition! (library, category, "auxiliary-syntax", identifier, primitive);
						}
						implemented_symbols.insert (identifier);
					}
					
					SyntaxPrimitive::Unimplemented => {
						if print_syntaxes && print_unimplemented {
							print_definition! (library, category, "syntax", identifier, primitive);
						}
						unimplemented_symbols.insert (identifier);
					}
					
					SyntaxPrimitive::Unsupported => {
						if print_syntaxes && print_unsupported {
							print_definition! (library, category, "syntax", identifier, primitive);
						}
						unsupported_symbols.insert (identifier);
					}
					
					SyntaxPrimitive::Reserved => {
						if print_syntaxes && print_reserved {
							print_definition! (library, category, "reserved", identifier, primitive);
						}
						reserved_symbols.insert (identifier);
					}
					
				}
			},
			
			_ =>
				if print_values {
					print_definition! (library, category, "value", identifier, value);
					implemented_symbols.insert (identifier);
				},
			
		}
	}
	
	if print_definitions_table && print_headers {
		eprintln! ();
		eprintln! ("****");
		eprintln! ();
	}
	
	
	
	
	if print_definitions_symbols && print_headers {
		eprintln! ();
		eprintln! ("## Scheme R7RS definitions -- summary");
		eprintln! ();
	}
	
	macro_rules! print_symbols {
		($label : expr, $symbols : expr, $print : expr, $total_available_symbols : expr, $total_specified_symbols : expr) => (
			if print_definitions_symbols {
				let symbols_count = $symbols.len ();
				let symbols_ratio_vs_available = (symbols_count as f64) / ($total_available_symbols as f64);
				let symbols_ratio_vs_specified = (symbols_count as f64) / ($total_specified_symbols as f64);
				eprintln! ("* {:16} {:4} ({:05.2}% / {:05.2}%)", $label, symbols_count, symbols_ratio_vs_available * 100.0, symbols_ratio_vs_specified * 100.0);
				if $print && print_definitions_symbols_list {
					let mut symbols = $symbols.into_iter () .collect::<StdVec<_>> ();
					symbols.sort ();
					if ! symbols.is_empty () {
						eprintln! ("  ```");
						for symbol in symbols.into_iter () {
							eprintln! ("    {}", symbol.string_as_str ());
						}
						eprintln! ("  ```");
					}
				}
				eprintln! ();
			}
		);
	}
	
	if print_definitions_symbols {
		let total_available_symbols = implemented_symbols.len () + unimplemented_symbols.len ();
		let total_specified_symbols = total_available_symbols + unsupported_symbols.len ();
		print_symbols! ("implemented", implemented_symbols, print_implemented && print_implemented_symbols, total_available_symbols, total_specified_symbols);
		print_symbols! ("unimplemented", unimplemented_symbols, print_unimplemented, total_available_symbols, total_specified_symbols);
		print_symbols! ("unsupported", unsupported_symbols, print_unsupported, total_available_symbols, total_specified_symbols);
		print_symbols! ("reserved", reserved_symbols, print_reserved && print_reserved_symbols, total_available_symbols, total_specified_symbols);
	}
	
	if print_definitions_symbols && print_headers {
		eprintln! ("****");
		eprintln! ();
	}
	
});

