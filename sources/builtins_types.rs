

use super::errors::exports::*;
use super::values::exports::*;




pub mod exports {
	
	pub use super::{is_true, is_false, is_not_false, is_true_or_equivalent, is_false_or_equivalent};
	
	pub use super::{is_null, is_void, is_undefined};
	pub use super::{is_null_all_2, is_null_all_3, is_null_all_4};
	pub use super::{is_null_any_2, is_null_any_3, is_null_any_4};
	
	pub use super::{is_number};
	
	pub use super::{is_list, is_list_proper, is_list_proper_or_empty, is_list_dotted, is_list_dotted_or_empty, is_list_cyclic, is_list_cyclic_or_empty};
	
	pub use super::{is_procedure, is_syntax};
	
	pub use super::{number_class, list_class, procedure_class, syntax_class};
	pub use super::{NumberClass, ListClass, ProcedureClass, SyntaxClass};
	
}




pub fn is_true (value : &Value) -> (bool) {
	if let Ok (value) = Boolean::try_as_ref (value) {
		return value.0 == true;
	} else {
		return false;
	}
}

pub fn is_false (value : &Value) -> (bool) {
	if let Ok (value) = Boolean::try_as_ref (value) {
		return value.0 == false;
	} else {
		return false;
	}
}




pub fn is_not_false (value : &Value) -> (bool) {
	return !is_false (value);
}

pub fn is_true_or_equivalent (value : &Value) -> (bool) {
	!is_false_or_equivalent (value)
}

pub fn is_false_or_equivalent (value : &Value) -> (bool) {
	match value.class () {
		ValueClass::Null | ValueClass::Void | ValueClass::Undefined =>
			return true,
		ValueClass::Boolean =>
			return Boolean::as_ref (value) .0 == false,
		ValueClass::Error =>
			return true,
		_ =>
			return false,
	}
}




pub fn is_null (value : &Value) -> (bool) {
	return value.is (ValueClass::Null);
}

pub fn is_void (value : &Value) -> (bool) {
	return value.is (ValueClass::Void);
}

pub fn is_undefined (value : &Value) -> (bool) {
	return value.is (ValueClass::Undefined);
}




pub fn is_null_all_2 (value_1 : &Value, value_2 : &Value) -> (bool) {
	return is_null (value_1) && is_null (value_2)
}

pub fn is_null_all_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (bool) {
	return is_null (value_1) && is_null (value_2) && is_null (value_3)
}

pub fn is_null_all_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (bool) {
	return is_null (value_1) && is_null (value_2) && is_null (value_3) && is_null (value_4)
}


pub fn is_null_any_2 (value_1 : &Value, value_2 : &Value) -> (bool) {
	return is_null (value_1) || is_null (value_2)
}

pub fn is_null_any_3 (value_1 : &Value, value_2 : &Value, value_3 : &Value) -> (bool) {
	return is_null (value_1) || is_null (value_2) || is_null (value_3)
}

pub fn is_null_any_4 (value_1 : &Value, value_2 : &Value, value_3 : &Value, value_4 : &Value) -> (bool) {
	return is_null (value_1) || is_null (value_2) || is_null (value_3) || is_null (value_4)
}




pub fn is_number (value : &Value) -> (bool) {
	return number_class (value) .is_ok ();
}




pub fn is_list (value : &Value) -> (bool) {
	return list_class (value) .is_ok ();
}

pub fn is_list_proper (value : &Value) -> (bool) {
	let class = list_class (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return class == ListClass::Proper;
	} else {
		return false;
	}
}

pub fn is_list_proper_or_empty (value : &Value) -> (bool) {
	let class = list_class (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return (class == ListClass::Proper) || (class == ListClass::Empty);
	} else {
		return false;
	}
}

pub fn is_list_dotted (value : &Value) -> (bool) {
	let class = list_class (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return class == ListClass::Dotted;
	} else {
		return false;
	}
}

pub fn is_list_dotted_or_empty (value : &Value) -> (bool) {
	let class = list_class (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return (class == ListClass::Dotted) || (class == ListClass::Empty);
	} else {
		return false;
	}
}

pub fn is_list_cyclic (value : &Value) -> (bool) {
	let class = list_class (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return class == ListClass::Cyclic;
	} else {
		return false;
	}
}

pub fn is_list_cyclic_or_empty (value : &Value) -> (bool) {
	let class = list_class (value);
	if class.is_ok () {
		let class = class.unwrap ();
		return (class == ListClass::Cyclic) || (class == ListClass::Empty);
	} else {
		return false;
	}
}




pub fn is_procedure (value : &Value) -> (bool) {
	return procedure_class (value) .is_ok ();
}

pub fn is_syntax (value : &Value) -> (bool) {
	return syntax_class (value) .is_ok ();
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum NumberClass {
	Integer,
	Real,
}


pub fn number_class (value : &Value) -> (Outcome<NumberClass>) {
	match value.class () {
		
		ValueClass::NumberInteger =>
			succeed! (NumberClass::Integer),
		
		ValueClass::NumberReal =>
			succeed! (NumberClass::Real),
		
		_ =>
			fail! (0x7a6c3f3e),
		
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ListClass {
	Empty,
	Proper,
	Dotted,
	Cyclic,
}


pub fn list_class (value : &Value) -> (Outcome<ListClass>) {
	match value.class () {
		
		ValueClass::Null =>
			succeed! (ListClass::Empty),
		
		ValueClass::Pair => {
			let mut cursor = Pair::as_ref (value) .right ();
			loop {
				match cursor.class () {
					ValueClass::Pair =>
						cursor = Pair::as_ref (cursor) .right (),
					ValueClass::Null =>
						succeed! (ListClass::Proper),
					_ =>
						succeed! (ListClass::Dotted),
				}
				if cursor == value {
					succeed! (ListClass::Cyclic);
				}
			}
		},
		
		_ =>
			fail! (0xf9bfa236),
		
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum ProcedureClass {
	Lambda,
	Primitive,
}


pub fn procedure_class (value : &Value) -> (Outcome<ProcedureClass>) {
	match value.class () {
		ValueClass::Lambda =>
			succeed! (ProcedureClass::Lambda),
		ValueClass::ProcedurePrimitive =>
			succeed! (ProcedureClass::Primitive),
		_ =>
			fail! (0xef418db1),
	}
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Hash) ]
pub enum SyntaxClass {
	Primitive,
}


pub fn syntax_class (value : &Value) -> (Outcome<SyntaxClass>) {
	match value.class () {
		ValueClass::SyntaxPrimitive =>
			succeed! (SyntaxClass::Primitive),
		_ =>
			fail! (0x97144c3b),
	}
}
