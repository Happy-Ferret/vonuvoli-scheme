

use super::constants::exports::*;
use super::errors::exports::*;
use super::extended_procedures::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			
			record_kind_build,
			
			record_kind_identifier,
			record_kind_size,
			
		};
	
	pub use super::{
			
			record_kind_get, record_kind_is,
			
			record_build_0, record_build_1, record_build_2, record_build_3, record_build_4, record_build_n,
			
			record_get, record_set,
			record_get_x, record_set_x,
			
		};
	
	pub use super::{
			
			record_to_array, record_from_array,
			record_to_values, record_from_values,
			record_to_list, record_from_list,
			
		};
	
	pub use super::{
			
			record_kind_is_fn,
			
			record_build_fn,
			
			record_get_fn, record_set_fn,
			record_get_x_fn, record_set_x_fn,
			
		};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_kind_build (identifier : Option<&Value>, fields : &Value) -> (Outcome<RecordKind>) {
	let (fields, size) = match fields.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (fields) =>
			(None, try! (fields.try_to_usize ())),
		_ =>
			fail_unimplemented! (0xefef1c6f), // deferred
	};
	let identifier = if let Some (identifier) = identifier {
		match identifier.kind_match_as_ref () {
			ValueKindMatchAsRef::Boolean (identifier) =>
				if ! identifier.value () {
					None
				} else {
					fail! (0xddeb44fd);
				},
			ValueKindMatchAsRef::Symbol (identifier) =>
				Some (identifier.string_rc_clone ()),
			_ =>
				fail! (0xbd53861b),
		}
	} else {
		None
	};
	return RecordKind::new (identifier, fields, size);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_kind_identifier (kind : &Value) -> (Outcome<Value>) {
	let kind = try_as_record_kind_ref! (kind);
	if let Some (identifier) = kind.identifier_rc_clone () {
		succeed! (Symbol::from_rc (identifier) .into ());
	} else {
		succeed! (FALSE_VALUE);
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_kind_size (kind : &Value) -> (Outcome<usize>) {
	let kind = try_as_record_kind_ref! (kind);
	succeed! (kind.values_count ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_kind_get (value : &Value) -> (Outcome<RecordKind>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::RecordImmutable (value) =>
			succeed! (value.kind () .clone () .into ()),
		ValueKindMatchAsRef::RecordMutable (value) =>
			succeed! (value.kind () .clone () .into ()),
		_ =>
			fail! (0xc4f39aeb),
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_kind_is (kind : &RecordKind, value : &Value, immutable : Option<bool>) -> (bool) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::RecordImmutable (value) =>
			return RecordKind::is_self (value.kind (), kind) && immutable.unwrap_or (true),
		ValueKindMatchAsRef::RecordMutable (value) =>
			return RecordKind::is_self (value.kind (), kind) && ! immutable.unwrap_or (false),
		_ =>
			return false,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_0 (kind : &RecordKind, fields : Option<&[usize]>, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 0 {
			fail! (0x6007924a);
		}
		vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ())
	} else {
		vec! []
	};
	return record_new (kind, values, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_1 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 1 {
			fail! (0x03ce30be);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		try! (vec_set (&mut values, fields[0], value_1));
		values
	} else {
		vec! [value_1.clone ()]
	};
	return record_new (kind, values, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_2 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, value_2 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 2 {
			fail! (0x34c1df40);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		try! (vec_set (&mut values, fields[0], value_1));
		try! (vec_set (&mut values, fields[1], value_2));
		values
	} else {
		vec! [value_1.clone (), value_2.clone ()]
	};
	return record_new (kind, values, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_3 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, value_2 : &Value, value_3 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 3 {
			fail! (0xa678b9a9);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		try! (vec_set (&mut values, fields[0], value_1));
		try! (vec_set (&mut values, fields[1], value_2));
		try! (vec_set (&mut values, fields[2], value_3));
		values
	} else {
		vec! [value_1.clone (), value_2.clone (), value_3.clone ()]
	};
	return record_new (kind, values, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_4 (kind : &RecordKind, fields : Option<&[usize]>, value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value, immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != 4 {
			fail! (0xef4b85b7);
		}
		let mut values = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		try! (vec_set (&mut values, fields[0], value_1));
		try! (vec_set (&mut values, fields[1], value_2));
		try! (vec_set (&mut values, fields[2], value_3));
		try! (vec_set (&mut values, fields[3], value_4));
		values
	} else {
		vec! [value_1.clone (), value_2.clone (), value_3.clone (), value_4.clone ()]
	};
	return record_new (kind, values, immutable);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_n <ValueRef : StdAsRef<Value>> (kind : &RecordKind, fields : Option<&[usize]>, values : &[ValueRef], immutable : Option<bool>) -> (Outcome<Value>) {
	let values = if let Some (fields) = fields {
		if fields.len () != values.len () {
			fail! (0x9c52a9f1);
		}
		let mut values_ = vec_clone_fill (&UNDEFINED_VALUE, kind.values_count ());
		for index in 0 .. fields.len () {
			try! (vec_set_ref (&mut values_, fields[index], &values[index]));
		}
		values_
	} else {
		vec_clone_slice_ref (values)
	};
	return record_new (kind, values, immutable);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_get (kind : Option<&RecordKind>, field : usize, record : &Value) -> (Outcome<Value>) {
	let record = try_as_record_ref! (record);
	if let Some (kind) = kind {
		if ! record.is_kind (kind) {
			fail! (0xe5012bde);
		}
	}
	let record = record.values_as_slice ();
	let value = try_some! (record.get (field), 0xcce25bab);
	let value = value.clone ();
	succeed! (value);
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_set (kind : Option<&RecordKind>, field : usize, record : &Value, value : &Value) -> (Outcome<Value>) {
	let record = try_as_record_mutable_ref! (record);
	if let Some (kind) = kind {
		if ! record.is_kind (kind) {
			fail! (0x64c0a2cd);
		}
	}
	let mut record = try! (record.values_ref_mut ());
	let value_ref = try_some! (record.get_mut (field), 0x8b20ee6e);
	let mut value_swap = value.clone ();
	mem::swap (&mut value_swap, value_ref);
	succeed! (value_swap);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_get_x (kind : Option<&RecordKind>, field : &Value, record : &Value) -> (Outcome<Value>) {
	match field.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (field) =>
			return record_get (kind, try! (field.try_to_usize ()), record),
		ValueKindMatchAsRef::Symbol (_) =>
			fail_unimplemented! (0x8424a427), // deferred
		_ =>
			fail! (0x8dbc8031),
	}
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_set_x (kind : Option<&RecordKind>, field : &Value, record : &Value, value : &Value) -> (Outcome<Value>) {
	match field.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (field) =>
			return record_set (kind, try! (field.try_to_usize ()), record, value),
		ValueKindMatchAsRef::Symbol (_) =>
			fail_unimplemented! (0xd2d2f80a), // deferred
		_ =>
			fail! (0x194d0fbf),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_to_array (_kind : Option<&RecordKind>, _record : &Value, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0x2bb3bd43); // deferred
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_to_values (_kind : Option<&RecordKind>, _record : &Value, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0xb9e5c4ce); // deferred
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_to_list (_kind : Option<&RecordKind>, _record : &Value, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0x18314e71); // deferred
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_from_array (_kind : Option<&RecordKind>, _values : &Value, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0xd1a160d3); // deferred
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_from_values (_kind : Option<&RecordKind>, _values : &Value, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0x6f32a452); // deferred
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_from_list (_kind : Option<&RecordKind>, _values : &Value, _immutable : Option<bool>) -> (Outcome<Value>) {
	fail_unimplemented! (0xdd729ef6); // deferred
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_kind_is_fn (kind : &RecordKind, immutable : Option<bool>) -> (ProcedureExtended) {
	return ProcedureExtendedInternals::RecordKindIs (kind.clone (), immutable) .into ();
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_build_fn (kind : &RecordKind, fields : Option<&Value>, immutable : Option<bool>) -> (Outcome<ProcedureExtended>) {
	let fields = if let Some (fields) = fields {
		match fields.class_match_as_ref () {
			ValueClassMatchAsRef::Boolean (fields) =>
				if ! fields.value () {
					None
				} else {
					fail! (0xd31ec4f3);
				},
			ValueClassMatchAsRef::Array (class) => {
				let array = try! (class.array_ref ());
				let fields = try_vec_map! (array.values_iter (), field, try_as_number_integer_ref! (field) .try_to_usize ());
				Some (fields.into_boxed_slice ())
			},
			_ =>
				fail_unimplemented! (0x0b12cf86),
		}
	} else {
		None
	};
	let kind = kind.clone ();
	succeed! (ProcedureExtendedInternals::RecordBuild (kind, fields, immutable) .into ());
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_get_fn (kind : Option<&RecordKind>, field : usize) -> (Outcome<ProcedureExtended>) {
	if let Some (kind) = kind {
		if field >= kind.values_count () {
			fail! (0x56ee989d);
		}
	}
	let kind = option_map! (kind, kind.clone ());
	succeed! (ProcedureExtendedInternals::RecordGet (kind, field) .into ());
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_set_fn (kind : Option<&RecordKind>, field : usize) -> (Outcome<ProcedureExtended>) {
	if let Some (kind) = kind {
		if field >= kind.values_count () {
			fail! (0x4747b115);
		}
	}
	let kind = option_map! (kind, kind.clone ());
	succeed! (ProcedureExtendedInternals::RecordSet (kind, field) .into ());
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_get_x_fn (kind : Option<&RecordKind>, field : &Value) -> (Outcome<ProcedureExtended>) {
	match field.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (field) =>
			return record_get_fn (kind, try! (field.try_to_usize ())),
		_ => {
			let kind = option_map! (kind, kind.clone ());
			succeed! (ProcedureExtendedInternals::RecordGetX (kind, field.clone ()) .into ());
		},
	}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn record_set_x_fn (kind : Option<&RecordKind>, field : &Value) -> (Outcome<ProcedureExtended>) {
	match field.kind_match_as_ref () {
		ValueKindMatchAsRef::NumberInteger (field) =>
			return record_set_fn (kind, try! (field.try_to_usize ())),
		_ => {
			let kind = option_map! (kind, kind.clone ());
			succeed! (ProcedureExtendedInternals::RecordSetX (kind, field.clone ()) .into ());
		},
	}
}

