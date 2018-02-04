

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::TypePrimitive0;
	pub use super::TypePrimitive1;
	pub use super::TypePrimitive2;
	pub use super::TypePrimitive3;
	pub use super::TypePrimitive4;
	pub use super::TypePrimitive5;
	pub use super::TypePrimitiveN;
	pub use super::TypePrimitiveV;
	
	pub use super::type_primitive_0_evaluate;
	pub use super::type_primitive_1_evaluate;
	pub use super::type_primitive_1_evaluate_0;
	pub use super::type_primitive_2_evaluate;
	pub use super::type_primitive_3_evaluate;
	pub use super::type_primitive_4_evaluate;
	pub use super::type_primitive_5_evaluate;
	pub use super::type_primitive_n_evaluate;
	
	pub use super::type_primitive_v_alternative_0;
	pub use super::type_primitive_v_alternative_1;
	pub use super::type_primitive_v_alternative_2;
	pub use super::type_primitive_v_alternative_3;
	pub use super::type_primitive_v_alternative_4;
	pub use super::type_primitive_v_alternative_5;
	pub use super::type_primitive_v_alternative_n;
	
	pub use super::type_primitive_0_attributes;
	pub use super::type_primitive_1_attributes;
	pub use super::type_primitive_2_attributes;
	pub use super::type_primitive_3_attributes;
	pub use super::type_primitive_4_attributes;
	pub use super::type_primitive_5_attributes;
	pub use super::type_primitive_n_attributes;
	
}




macro_rules! def_type_primitive_enum {
	( $identifier : ident ) => (
		
		#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
		pub enum $identifier {
			
			IsNull,
			IsNullNot,
			IsVoid,
			IsVoidNot,
			IsUndefined,
			IsUndefinedNot,
			
			IsBoolean,
			IsTrue,
			IsTrueNot,
			IsFalse,
			IsFalseNot,
			IsTrueOrEquivalent,
			IsFalseOrEquivalent,
			
			IsNumber,
			IsNumberInteger,
			IsNumberRational,
			IsNumberReal,
			IsNumberComplex,
			IsNumberExact,
			IsNumberExactInteger,
			IsNumberInexact,
			
			IsCharacter,
			
			IsSymbol,
			IsKeyword,
			IsUnique,
			
			IsStringRegex,
			
			IsString,
			IsStringImmutable,
			IsStringMutable,
			IsStringEmpty,
			IsStringImmutableEmpty,
			IsStringMutableEmpty,
			IsStringEmptyNot,
			IsStringImmutableEmptyNot,
			IsStringMutableEmptyNot,
			
			IsBytes,
			IsBytesImmutable,
			IsBytesMutable,
			IsBytesEmpty,
			IsBytesImmutableEmpty,
			IsBytesMutableEmpty,
			IsBytesEmptyNot,
			IsBytesImmutableEmptyNot,
			IsBytesMutableEmptyNot,
			
			IsPair,
			IsPairMutable,
			IsPairImmutable,
			
			IsArray,
			IsArrayMutable,
			IsArrayImmutable,
			IsArrayEmpty,
			IsArrayMutableEmpty,
			IsArrayImmutableEmpty,
			IsArrayEmptyNot,
			IsArrayMutableEmptyNot,
			IsArrayImmutableEmptyNot,
			
			IsValues,
			IsValuesEmpty,
			IsValuesEmptyNot,
			
			IsRecordKind,
			IsRecord,
			IsRecordImmutable,
			IsRecordMutable,
			
			IsError,
			IsErrorSyntax,
			IsErrorFile,
			IsErrorPort,
			IsErrorPortInput,
			IsErrorPortOutput,
			
			IsList,
			IsListProper,
			IsListProperOrEmpty,
			IsListDotted,
			IsListDottedOrEmpty,
			IsListCyclic,
			IsListCyclicOrEmpty,
			
			IsProcedure,
			IsSyntax,
			
			IsPort,
			IsPortInput,
			IsPortOutput,
			IsPortBinary,
			IsPortTextual,
			IsPortEof,
			
			IsProcess,
			IsContext,
			IsBinding,
			IsParameters,
			IsParameter,
			IsPromise,
			
			IsResource,
			IsInternal,
			IsOpaque,
			
			IsNumberZero,
			IsNumberPositive,
			IsNumberNegative,
			IsNumberFinite,
			IsNumberInfinite,
			IsNumberNan,
			IsNumberEven,
			IsNumberOdd,
			
			IsCharacterNumeric,
			IsCharacterAlphabetic,
			IsCharacterAlphabeticUpperCase,
			IsCharacterAlphabeticLowerCase,
			IsCharacterAlphabeticOrNumeric,
			IsCharacterWhitespace,
			IsCharacterControl,
			IsCharacterAscii,
			IsCharacterAsciiNumeric,
			IsCharacterAsciiNumericBase8,
			IsCharacterAsciiNumericBase16,
			IsCharacterAsciiAlphabetic,
			IsCharacterAsciiAlphabeticUpperCase,
			IsCharacterAsciiAlphabeticLowerCase,
			IsCharacterAsciiAlphabeticOrNumeric,
			IsCharacterAsciiWhitespace,
			IsCharacterAsciiControl,
			IsCharacterAsciiPunctuation,
			IsCharacterAsciiGraphic,
			
		}
		
	);
}




def_type_primitive_enum! (TypePrimitive1);
def_type_primitive_enum! (TypePrimitive2);
def_type_primitive_enum! (TypePrimitive3);
def_type_primitive_enum! (TypePrimitive4);
def_type_primitive_enum! (TypePrimitiveN);
def_type_primitive_enum! (TypePrimitiveV);


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum TypePrimitive0 {}

#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum TypePrimitive5 {}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_0_evaluate (primitive : TypePrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_1_evaluate (primitive : TypePrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	return type_primitive_1_evaluate_0 (primitive, input_1) .into_0 ();
}


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_1_evaluate_0 (primitive : TypePrimitive1, input_1 : &Value) -> (Outcome<bool>) {
	match primitive {
		
		TypePrimitive1::IsNull =>
			return is_null (input_1) .into_0 (),
		
		TypePrimitive1::IsNullNot =>
			return is_not_null (input_1) .into_0 (),
		
		TypePrimitive1::IsVoid =>
			return is_void (input_1) .into_0 (),
		
		TypePrimitive1::IsVoidNot =>
			return is_not_void (input_1) .into_0 (),
		
		TypePrimitive1::IsUndefined =>
			return is_undefined (input_1) .into_0 (),
		
		TypePrimitive1::IsUndefinedNot =>
			return is_not_undefined (input_1) .into_0 (),
		
		TypePrimitive1::IsBoolean =>
			return is_boolean (input_1) .into_0 (),
		
		TypePrimitive1::IsTrue =>
			return is_true (input_1) .into_0 (),
		
		TypePrimitive1::IsTrueNot =>
			return is_not_true (input_1) .into_0 (),
		
		TypePrimitive1::IsFalse =>
			return is_false (input_1) .into_0 (),
		
		TypePrimitive1::IsFalseNot =>
			return is_not_false (input_1) .into_0 (),
		
		TypePrimitive1::IsTrueOrEquivalent =>
			return is_true_or_equivalent (input_1) .into_0 (),
		
		TypePrimitive1::IsFalseOrEquivalent =>
			return is_false_or_equivalent (input_1) .into_0 (),
		
		TypePrimitive1::IsNumber =>
			return is_number (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberInteger =>
			return is_number_integer (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberRational =>
			return is_number_rational (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberReal =>
			return is_number_real (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberComplex =>
			return is_number_complex (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberExact =>
			return is_number_exact (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberExactInteger =>
			return is_number_exact_integer (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberInexact =>
			return is_number_inexact (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacter =>
			return is_character (input_1) .into_0 (),
		
		TypePrimitive1::IsSymbol =>
			return is_symbol (input_1) .into_0 (),
		
		TypePrimitive1::IsKeyword =>
			return is_keyword (input_1) .into_0 (),
		
		TypePrimitive1::IsUnique =>
			return is_unique (input_1) .into_0 (),
		
		TypePrimitive1::IsStringRegex =>
			return is_string_regex (input_1) .into_0 (),
		
		TypePrimitive1::IsString =>
			return is_string (input_1) .into_0 (),
		
		TypePrimitive1::IsStringImmutable =>
			return is_string_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsStringMutable =>
			return is_string_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsStringEmpty =>
			return is_string_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsStringImmutableEmpty =>
			return is_string_immutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsStringMutableEmpty =>
			return is_string_mutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsStringEmptyNot =>
			return is_string_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsStringImmutableEmptyNot =>
			return is_string_immutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsStringMutableEmptyNot =>
			return is_string_mutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytes =>
			return is_bytes (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesImmutable =>
			return is_bytes_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesMutable =>
			return is_bytes_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesEmpty =>
			return is_bytes_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesImmutableEmpty =>
			return is_bytes_immutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesMutableEmpty =>
			return is_bytes_mutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesEmptyNot =>
			return is_bytes_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesImmutableEmptyNot =>
			return is_bytes_immutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsBytesMutableEmptyNot =>
			return is_bytes_mutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsPair =>
			return is_pair (input_1) .into_0 (),
		
		TypePrimitive1::IsPairImmutable =>
			return is_pair_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsPairMutable =>
			return is_pair_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsArray =>
			return is_array (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayImmutable =>
			return is_array_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayMutable =>
			return is_array_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayEmpty =>
			return is_array_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayImmutableEmpty =>
			return is_array_immutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayMutableEmpty =>
			return is_array_mutable_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayEmptyNot =>
			return is_array_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayImmutableEmptyNot =>
			return is_array_immutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsArrayMutableEmptyNot =>
			return is_array_mutable_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsValues =>
			return is_values (input_1) .into_0 (),
		
		TypePrimitive1::IsValuesEmpty =>
			return is_values_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsValuesEmptyNot =>
			return is_values_not_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsRecordKind =>
			return is_record_kind (input_1) .into_0 (),
		
		TypePrimitive1::IsRecord =>
			return is_record (input_1) .into_0 (),
		
		TypePrimitive1::IsRecordImmutable =>
			return is_record_immutable (input_1) .into_0 (),
		
		TypePrimitive1::IsRecordMutable =>
			return is_record_mutable (input_1) .into_0 (),
		
		TypePrimitive1::IsError =>
			return is_error (input_1) .into_0 (),
		
		TypePrimitive1::IsErrorSyntax =>
			return is_error_syntax (input_1) .into_0 (),
		
		TypePrimitive1::IsErrorFile =>
			return is_error_file (input_1) .into_0 (),
		
		TypePrimitive1::IsErrorPort =>
			return is_error_port (input_1) .into_0 (),
		
		TypePrimitive1::IsErrorPortInput =>
			return is_error_port_input (input_1) .into_0 (),
		
		TypePrimitive1::IsErrorPortOutput =>
			return is_error_port_output (input_1) .into_0 (),
		
		TypePrimitive1::IsList =>
			return is_list (input_1) .into_0 (),
		
		TypePrimitive1::IsListProper =>
			return is_list_proper (input_1) .into_0 (),
		
		TypePrimitive1::IsListProperOrEmpty =>
			return is_list_proper_or_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsListDotted =>
			return is_list_dotted (input_1) .into_0 (),
		
		TypePrimitive1::IsListDottedOrEmpty =>
			return is_list_dotted_or_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsListCyclic =>
			return is_list_cyclic (input_1) .into_0 (),
		
		TypePrimitive1::IsListCyclicOrEmpty =>
			return is_list_cyclic_or_empty (input_1) .into_0 (),
		
		TypePrimitive1::IsProcedure =>
			return is_procedure (input_1) .into_0 (),
		
		TypePrimitive1::IsSyntax =>
			return is_syntax (input_1) .into_0 (),
		
		TypePrimitive1::IsPort =>
			return is_port (input_1) .into_0 (),
		
		TypePrimitive1::IsPortInput =>
			return is_port_input (input_1) .into_0 (),
		
		TypePrimitive1::IsPortOutput =>
			return is_port_output (input_1) .into_0 (),
		
		TypePrimitive1::IsPortBinary =>
			return is_port_binary (input_1) .into_0 (),
		
		TypePrimitive1::IsPortTextual =>
			return is_port_textual (input_1) .into_0 (),
		
		TypePrimitive1::IsPortEof =>
			return is_port_eof (input_1) .into_0 (),
		
		TypePrimitive1::IsProcess =>
			return is_process (input_1) .into_0 (),
		
		TypePrimitive1::IsContext =>
			return is_context (input_1) .into_0 (),
		
		TypePrimitive1::IsBinding =>
			return is_binding (input_1) .into_0 (),
		
		TypePrimitive1::IsParameters =>
			return is_parameters (input_1) .into_0 (),
		
		TypePrimitive1::IsParameter =>
			return is_parameter (input_1) .into_0 (),
		
		TypePrimitive1::IsPromise =>
			return is_promise (input_1) .into_0 (),
		
		TypePrimitive1::IsResource =>
			return is_resource (input_1) .into_0 (),
		
		TypePrimitive1::IsInternal =>
			return is_internal (input_1) .into_0 (),
		
		TypePrimitive1::IsOpaque =>
			return is_opaque (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberZero =>
			return is_number_zero (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberPositive =>
			return is_number_positive (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberNegative =>
			return is_number_negative (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberFinite =>
			return is_number_finite (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberInfinite =>
			return is_number_infinite (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberNan =>
			return is_number_nan (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberEven =>
			return is_number_even (input_1) .into_0 (),
		
		TypePrimitive1::IsNumberOdd =>
			return is_number_odd (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterNumeric =>
			return character_is_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabetic =>
			return character_is_alphabetic (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabeticUpperCase =>
			return character_is_alphabetic_upper_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabeticLowerCase =>
			return character_is_alphabetic_lower_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAlphabeticOrNumeric =>
			return character_is_alphabetic_or_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterWhitespace =>
			return character_is_whitespace (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterControl =>
			return character_is_control (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAscii =>
			return character_is_ascii (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiNumeric =>
			return character_is_ascii_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiNumericBase8 =>
			return character_is_ascii_numeric_base_8 (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiNumericBase16 =>
			return character_is_ascii_numeric_base_16 (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabetic =>
			return character_is_ascii_alphabetic (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticUpperCase =>
			return character_is_ascii_alphabetic_upper_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticLowerCase =>
			return character_is_ascii_alphabetic_lower_case (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiAlphabeticOrNumeric =>
			return character_is_ascii_alphabetic_or_numeric (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiWhitespace =>
			return character_is_ascii_whitespace (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiControl =>
			return character_is_ascii_control (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiPunctuation =>
			return character_is_ascii_punctuation (input_1) .into_0 (),
		
		TypePrimitive1::IsCharacterAsciiGraphic =>
			return character_is_ascii_graphic (input_1) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_2_evaluate (_primitive : TypePrimitive2, _input_1 : &Value, _input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	fail_unimplemented! (0xcce4932c);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_3_evaluate (_primitive : TypePrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	fail_unimplemented! (0x0d047c81);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_4_evaluate (_primitive : TypePrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	fail_unimplemented! (0x5936bf08);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_5_evaluate (primitive : TypePrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_n_evaluate (_primitive : TypePrimitiveN, _inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	fail_unimplemented! (0x1a44de68);
}




macro_rules! def_type_primitive_v_alternative_fn {
	( $identifier : ident, $alternative : ident ) => (
		
		#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
		pub fn $identifier (_primitive : TypePrimitiveV) -> (Option<$alternative>) {
			return None;
		}
		
	);
}


def_type_primitive_v_alternative_fn! (type_primitive_v_alternative_1, TypePrimitive1);
def_type_primitive_v_alternative_fn! (type_primitive_v_alternative_2, TypePrimitive2);
def_type_primitive_v_alternative_fn! (type_primitive_v_alternative_3, TypePrimitive3);
def_type_primitive_v_alternative_fn! (type_primitive_v_alternative_4, TypePrimitive4);
def_type_primitive_v_alternative_fn! (type_primitive_v_alternative_n, TypePrimitiveN);


#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_v_alternative_0 (_primitive : TypePrimitiveV) -> (Option<TypePrimitive0>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_v_alternative_5 (_primitive : TypePrimitiveV) -> (Option<TypePrimitive5>) {
	return None;
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_0_attributes (_primitive : TypePrimitive0) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_0);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_1_attributes (_primitive : TypePrimitive1) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_1);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_2_attributes (_primitive : TypePrimitive2) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_2);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_3_attributes (_primitive : TypePrimitive3) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_3);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_4_attributes (_primitive : TypePrimitive4) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_4);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_5_attributes (_primitive : TypePrimitive5) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_5);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn type_primitive_n_attributes (_primitive : TypePrimitiveN) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_N);
}

