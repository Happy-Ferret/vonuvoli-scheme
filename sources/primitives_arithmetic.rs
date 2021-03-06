

use super::constants::exports::*;
use super::conversions::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::ArithmeticPrimitive0;
	pub use super::ArithmeticPrimitive1;
	pub use super::ArithmeticPrimitive2;
	pub use super::ArithmeticPrimitive3;
	pub use super::ArithmeticPrimitive4;
	pub use super::ArithmeticPrimitive5;
	pub use super::ArithmeticPrimitiveN;
	pub use super::ArithmeticPrimitiveV;
	
	pub use super::arithmetic_primitive_0_evaluate;
	pub use super::arithmetic_primitive_1_evaluate;
	pub use super::arithmetic_primitive_2_evaluate;
	pub use super::arithmetic_primitive_3_evaluate;
	pub use super::arithmetic_primitive_4_evaluate;
	pub use super::arithmetic_primitive_5_evaluate;
	pub use super::arithmetic_primitive_n_evaluate;
	
	pub use super::arithmetic_primitive_v_alternative_0;
	pub use super::arithmetic_primitive_v_alternative_1;
	pub use super::arithmetic_primitive_v_alternative_2;
	pub use super::arithmetic_primitive_v_alternative_3;
	pub use super::arithmetic_primitive_v_alternative_4;
	pub use super::arithmetic_primitive_v_alternative_5;
	pub use super::arithmetic_primitive_v_alternative_n;
	
	pub use super::arithmetic_primitive_0_attributes;
	pub use super::arithmetic_primitive_1_attributes;
	pub use super::arithmetic_primitive_2_attributes;
	pub use super::arithmetic_primitive_3_attributes;
	pub use super::arithmetic_primitive_4_attributes;
	pub use super::arithmetic_primitive_5_attributes;
	pub use super::arithmetic_primitive_n_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive0 {
	
	Addition,
	Multiplication,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive1 {
	
	Negate,
	Absolute,
	Signum,
	
	Floor,
	Ceiling,
	Round,
	Truncate,
	Fractional,
	
	CoerceToExact,
	CoerceToInexact,
	
	Square,
	SquareRoot,
	SquareRootWithRemainder,
	Exponential,
	Logarithm,
	
	Sin,
	Cos,
	Tan,
	Asin,
	Acos,
	Atan,
	
	Addition,
	Subtraction,
	Multiplication,
	Division,
	
	/*
	AdditionWithInteger ( Constant<i16> ),
	SubtractionWithInteger ( Constant<i16> ),
	MultiplicationWithInteger ( Constant<i16> ),
	DivisionWithInteger ( Constant<i16> ),
	*/
	
	/*
	AdditionWithReal ( Constant<f32> ),
	SubtractionWithReal ( Constant<f32> ),
	MultiplicationWithReal ( Constant<f32> ),
	DivisionWithReal ( Constant<f32> ),
	*/
	
	GreatestCommonDivisor,
	LeastCommonMultiple,
	
	Minimum,
	Maximum,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive2 {
	
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Remainder,
	
	DivisionFloor,
	DivisionFloorQuotient,
	DivisionFloorRemainder,
	
	DivisionTruncate,
	DivisionTruncateQuotient,
	DivisionTruncateRemainder,
	
	GreatestCommonDivisor,
	LeastCommonMultiple,
	
	Minimum,
	Maximum,
	
	Power,
	
}



#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive3 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive4 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitiveN {
	
	Addition,
	Subtraction,
	Multiplication,
	Division,
	
	GreatestCommonDivisor,
	LeastCommonMultiple,
	
	Minimum,
	Maximum,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ArithmeticPrimitiveV {
	
	Addition,
	Subtraction,
	Multiplication,
	Division,
	
	GreatestCommonDivisor,
	LeastCommonMultiple,
	
	Minimum,
	Maximum,
	
}




// TODO:  Eliminate creation of temporary `Number*` values!
macro_rules! arithmetic_primitive_1_delegate_call {
	( $input : expr, $value_integer : ident, $for_integer : expr, $value_real : ident, $for_real : expr ) => (
		match try! (number_coerce_1a ($input)) {
			NumberCoercion1::Integer (value) =>
				{ let $value_integer = & NumberInteger (value); $for_integer.into () },
			NumberCoercion1::Real (value) =>
				{ let $value_real = & NumberReal (value); $for_real.into () },
		}
	);
	( $delegate : ident, $input : expr ) => (
		arithmetic_primitive_1_delegate_call! (
				$input,
				value, NumberInteger::$delegate (value),
				value, NumberReal::$delegate (value)
			);
	);
}


// TODO:  Eliminate creation of temporary `Number*` values!
macro_rules! arithmetic_primitive_2_delegate_call {
	( ($input_1 : expr, $input_2 : expr), ($value_1_integer : ident, $value_2_integer : ident), $for_integer : expr, ($value_1_real : ident, $value_2_real : ident), $for_real : expr ) => (
		match try! (number_coerce_2a ($input_1, $input_2)) {
			NumberCoercion2::Integer (value_1, value_2) =>
				{ let $value_1_integer = & NumberInteger (value_1); let $value_2_integer = & NumberInteger (value_2); $for_integer.into () },
			NumberCoercion2::Real (value_1, value_2) =>
				{ let $value_1_integer = & NumberReal (value_1); let $value_2_integer = & NumberReal (value_2); $for_real.into () },
		}
	);
	( $delegate : ident, ($input_1 : expr, $input_2 : expr) ) => (
		arithmetic_primitive_2_delegate_call! (
				($input_1, $input_2),
				(value_1, value_2), NumberInteger::$delegate (value_1, value_2),
				(value_1, value_2), NumberReal::$delegate (value_1, value_2)
			);
	);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_0_evaluate (primitive : ArithmeticPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	let output : Value = match primitive {
		
		ArithmeticPrimitive0::Addition =>
			ZERO.into (),
		
		ArithmeticPrimitive0::Multiplication =>
			ONE.into (),
		
	};
	
	succeed! (output);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_1_evaluate (primitive : ArithmeticPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	let output : Value = match primitive {
		
		ArithmeticPrimitive1::Negate =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, try! (value.neg ()),
					value, value.neg ()),
		
		ArithmeticPrimitive1::Absolute =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, try! (value.abs ()),
					value, value.abs ()),
		
		ArithmeticPrimitive1::Signum =>
			arithmetic_primitive_1_delegate_call! (signum, input_1),
		
		ArithmeticPrimitive1::Floor =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.clone (),
					value, value.floor ()),
		
		ArithmeticPrimitive1::Ceiling =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.clone (),
					value, value.ceil ()),
		
		ArithmeticPrimitive1::Round =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.clone (),
					value, value.round ()),
		
		ArithmeticPrimitive1::Truncate =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.clone (),
					value, value.trunc ()),
		
		ArithmeticPrimitive1::Fractional =>
			arithmetic_primitive_1_delegate_call! (input_1,
					_value, ZERO,
					value, value.fract ()),
		
		ArithmeticPrimitive1::CoerceToExact =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.clone (),
					value, try! (value.trunc () .try_to_integer ())),
		
		ArithmeticPrimitive1::CoerceToInexact =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, try! (value.try_to_real ()),
					value, value.clone ()),
		
		ArithmeticPrimitive1::Square =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, value.power (&2.into ()),
					value, value.power (&2.into ())),
		
		ArithmeticPrimitive1::SquareRoot =>
			arithmetic_primitive_1_delegate_call! (sqrt, input_1),
		
		ArithmeticPrimitive1::SquareRootWithRemainder =>
			fail_unimplemented! (0x0aed9e07), // deferred
		
		ArithmeticPrimitive1::Exponential =>
			arithmetic_primitive_1_delegate_call! (exp, input_1),
		
		ArithmeticPrimitive1::Logarithm =>
			arithmetic_primitive_1_delegate_call! (ln, input_1),
		
		ArithmeticPrimitive1::Sin =>
			arithmetic_primitive_1_delegate_call! (sin, input_1),
		
		ArithmeticPrimitive1::Cos =>
			arithmetic_primitive_1_delegate_call! (cos, input_1),
		
		ArithmeticPrimitive1::Tan =>
			arithmetic_primitive_1_delegate_call! (tan, input_1),
		
		ArithmeticPrimitive1::Asin =>
			arithmetic_primitive_1_delegate_call! (asin, input_1),
		
		ArithmeticPrimitive1::Acos =>
			arithmetic_primitive_1_delegate_call! (acos, input_1),
		
		ArithmeticPrimitive1::Atan =>
			arithmetic_primitive_1_delegate_call! (atan, input_1),
		
		ArithmeticPrimitive1::Addition =>
			try! (number_coerce_1a (input_1)) .into_value (),
		
		ArithmeticPrimitive1::Subtraction =>
			arithmetic_primitive_2_delegate_call! (
					(&ZERO.into (), input_1),
					(value_1, value_2), try! (NumberInteger::sub (value_1, value_2)),
					(value_1, value_2), NumberReal::sub (value_1, value_2)),
		
		ArithmeticPrimitive1::Multiplication =>
			try! (number_coerce_1a (input_1)) .into_value (),
		
		ArithmeticPrimitive1::Division =>
			arithmetic_primitive_2_delegate_call! (
					(&ONE.into (), input_1),
					(value_1, value_2), try! (NumberInteger::div (value_1, value_2)),
					(value_1, value_2), NumberReal::div (value_1, value_2)),
		
		
		/*
		ArithmeticPrimitive1::AdditionWithInteger (constant) =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, try! (NumberInteger::add (value, &constant.value () .into ())),
					value, NumberReal::add (value, &constant.value () .into ())),
		
		ArithmeticPrimitive1::SubtractionWithInteger (constant) =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, try! (NumberInteger::sub (value, &constant.value () .into ())),
					value, NumberReal::sub (value, &constant.value () .into ())),
		
		ArithmeticPrimitive1::MultiplicationWithInteger (constant) =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, try! (NumberInteger::mul (value, &constant.value () .into ())),
					value, NumberReal::mul (value, &constant.value () .into ())),
		
		ArithmeticPrimitive1::DivisionWithInteger (constant) =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, try! (NumberInteger::div (value, &constant.value () .into ())),
					value, NumberReal::div (value, &constant.value () .into ())),
		*/
		
		
		/*
		ArithmeticPrimitive1::AdditionWithReal (constant) =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, NumberReal::add (&value.value () .into (), &constant.value () .into ()),
					value, NumberReal::add (value, &constant.value () .into ())),
		
		ArithmeticPrimitive1::SubtractionWithReal (constant) =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, NumberReal::sub (&value.value () .into (), &constant.value () .into ()),
					value, NumberReal::sub (value, &constant.value () .into ())),
		
		ArithmeticPrimitive1::MultiplicationWithReal (constant) =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, NumberReal::mul (&value.value () .into (), &constant.value () .into ()),
					value, NumberReal::mul (value, &constant.value () .into ())),
		
		ArithmeticPrimitive1::DivisionWithReal (constant) =>
			arithmetic_primitive_1_delegate_call! (input_1,
					value, NumberReal::div (&value.value () .into (), &constant.value () .into ()),
					value, NumberReal::div (value, &constant.value () .into ())),
		*/
		
		
		ArithmeticPrimitive1::GreatestCommonDivisor =>
			try! (number_coerce_1a (input_1)) .into_value (),
		
		ArithmeticPrimitive1::LeastCommonMultiple =>
			try! (number_coerce_1a (input_1)) .into_value (),
		
		ArithmeticPrimitive1::Minimum =>
			try! (number_coerce_1a (input_1)) .into_value (),
		
		ArithmeticPrimitive1::Maximum =>
			try! (number_coerce_1a (input_1)) .into_value (),
		
	};
	
	succeed! (output);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_2_evaluate (primitive : ArithmeticPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	let output : Value = match primitive {
		
		ArithmeticPrimitive2::Addition =>
			arithmetic_primitive_2_delegate_call! (
					(input_1, input_2),
					(value_1, value_2), try! (NumberInteger::add (value_1, value_2)),
					(value_1, value_2), NumberReal::add (value_1, value_2)),
		
		ArithmeticPrimitive2::Subtraction =>
			arithmetic_primitive_2_delegate_call! (
					(input_1, input_2),
					(value_1, value_2), try! (NumberInteger::sub (value_1, value_2)),
					(value_1, value_2), NumberReal::sub (value_1, value_2)),
		
		ArithmeticPrimitive2::Multiplication =>
			arithmetic_primitive_2_delegate_call! (
					(input_1, input_2),
					(value_1, value_2), try! (NumberInteger::mul (value_1, value_2)),
					(value_1, value_2), NumberReal::mul (value_1, value_2)),
		
		ArithmeticPrimitive2::Division =>
			arithmetic_primitive_2_delegate_call! (
					(input_1, input_2),
					(value_1, value_2), try! (NumberInteger::div (value_1, value_2)),
					(value_1, value_2), NumberReal::div (value_1, value_2)),
		
		ArithmeticPrimitive2::Remainder =>
			arithmetic_primitive_2_delegate_call! (
					(input_1, input_2),
					(value_1, value_2), try! (NumberInteger::rem (value_1, value_2)),
					(value_1, value_2), NumberReal::rem (value_1, value_2)),
		
		ArithmeticPrimitive2::DivisionFloor =>
			fail_unimplemented! (0x738acdd6), // deferred
		
		ArithmeticPrimitive2::DivisionFloorQuotient =>
			fail_unimplemented! (0x2f425d22), // deferred
		
		ArithmeticPrimitive2::DivisionFloorRemainder =>
			fail_unimplemented! (0x8b709e6a), // deferred
		
		ArithmeticPrimitive2::DivisionTruncate =>
			fail_unimplemented! (0xbbf7f471), // deferred
			
		ArithmeticPrimitive2::DivisionTruncateQuotient =>
			fail_unimplemented! (0xd6bb8165), // deferred
		
		ArithmeticPrimitive2::DivisionTruncateRemainder =>
			fail_unimplemented! (0xfba74cd9), // deferred
		
		ArithmeticPrimitive2::Power =>
			arithmetic_primitive_2_delegate_call! ((input_1, input_2),
					(value_1, value_2), NumberReal::power (&<NumberReal>::from (value_1.value ()), &<NumberReal>::from (value_2.value ())),
					(value_1, value_2), NumberReal::power (value_1, value_2)),
		
		ArithmeticPrimitive2::GreatestCommonDivisor =>
			fail_unimplemented! (0x21f7773e), // deferred
		
		ArithmeticPrimitive2::LeastCommonMultiple =>
			fail_unimplemented! (0x79f53d20), // deferred
		
		ArithmeticPrimitive2::Minimum =>
			arithmetic_primitive_2_delegate_call! (min, (input_1, input_2)),
		
		ArithmeticPrimitive2::Maximum =>
			arithmetic_primitive_2_delegate_call! (max, (input_1, input_2)),
		
	};
	
	succeed! (output);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_3_evaluate (primitive : ArithmeticPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_4_evaluate (primitive : ArithmeticPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_5_evaluate (primitive : ArithmeticPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_n_evaluate (primitive : ArithmeticPrimitiveN, inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	
	match primitive {
		
		ArithmeticPrimitiveN::GreatestCommonDivisor =>
			fail_unimplemented! (0xeefd593c), // deferred
		
		ArithmeticPrimitiveN::LeastCommonMultiple =>
			fail_unimplemented! (0x4bc0a9ad), // deferred
		
		_ =>
			(),
		
	}
	
	let inputs_count = inputs.len ();
	
	if inputs_count == 0 {
		match primitive {
			
			ArithmeticPrimitiveN::Addition =>
				succeed! (ZERO.into ()),
			
			ArithmeticPrimitiveN::Multiplication =>
				succeed! (ONE.into ()),
			
			_ =>
				fail! (0x69d3b6cc),
		}
	}
	
	let mut output : Value = try! (number_coerce_1a (inputs[0])) .into_value ();
	
	if inputs_count == 1 {
		output = match primitive {
			
			ArithmeticPrimitiveN::Subtraction =>
				arithmetic_primitive_2_delegate_call! (
						(&ZERO.into (), &output),
						(value_1, value_2), try! (NumberInteger::sub (value_1, value_2)),
						(value_1, value_2), NumberReal::sub (value_1, value_2)),
			
			ArithmeticPrimitiveN::Division =>
				arithmetic_primitive_2_delegate_call! (
						(&ONE.into (), &output),
						(value_1, value_2), try! (NumberInteger::div (value_1, value_2)),
						(value_1, value_2), NumberReal::div (value_1, value_2)),
			
			_ =>
				output,
			
		};
		succeed! (output);
	}
	
	for input in inputs[1..].iter () {
		output = match primitive {
			
			ArithmeticPrimitiveN::Addition =>
				arithmetic_primitive_2_delegate_call! (
						(&output, input),
						(value_1, value_2), try! (NumberInteger::add (value_1, value_2)),
						(value_1, value_2), NumberReal::add (value_1, value_2)),
			
			ArithmeticPrimitiveN::Subtraction =>
				arithmetic_primitive_2_delegate_call! (
						(&output, input),
						(value_1, value_2), try! (NumberInteger::sub (value_1, value_2)),
						(value_1, value_2), NumberReal::sub (value_1, value_2)),
			
			ArithmeticPrimitiveN::Multiplication =>
				arithmetic_primitive_2_delegate_call! (
						(&output, input),
						(value_1, value_2), try! (NumberInteger::mul (value_1, value_2)),
						(value_1, value_2), NumberReal::mul (value_1, value_2)),
			
			ArithmeticPrimitiveN::Division =>
				arithmetic_primitive_2_delegate_call! (
						(&output, input),
						(value_1, value_2), try! (NumberInteger::div (value_1, value_2)),
						(value_1, value_2), NumberReal::div (value_1, value_2)),
			
			ArithmeticPrimitiveN::GreatestCommonDivisor =>
				fail_unreachable! (0x38fce646),
			
			ArithmeticPrimitiveN::LeastCommonMultiple =>
				fail_unreachable! (0x5c07f7c2),
			
			ArithmeticPrimitiveN::Minimum =>
				arithmetic_primitive_2_delegate_call! (min, (&output, input)),
			
			ArithmeticPrimitiveN::Maximum =>
				arithmetic_primitive_2_delegate_call! (max, (&output, input)),
			
		};
	}
	
	succeed! (output);
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_v_alternative_0 (primitive : ArithmeticPrimitiveV) -> (Option<ArithmeticPrimitive0>) {
	match primitive {
		ArithmeticPrimitiveV::Addition =>
			Some (ArithmeticPrimitive0::Addition),
		ArithmeticPrimitiveV::Subtraction =>
			None,
		ArithmeticPrimitiveV::Multiplication =>
			Some (ArithmeticPrimitive0::Multiplication),
		ArithmeticPrimitiveV::Division =>
			None,
		ArithmeticPrimitiveV::GreatestCommonDivisor =>
			None,
		ArithmeticPrimitiveV::LeastCommonMultiple =>
			None,
		ArithmeticPrimitiveV::Minimum =>
			None,
		ArithmeticPrimitiveV::Maximum =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_v_alternative_1 (primitive : ArithmeticPrimitiveV) -> (Option<ArithmeticPrimitive1>) {
	match primitive {
		ArithmeticPrimitiveV::Addition =>
			Some (ArithmeticPrimitive1::Addition),
		ArithmeticPrimitiveV::Subtraction =>
			Some (ArithmeticPrimitive1::Subtraction),
		ArithmeticPrimitiveV::Multiplication =>
			Some (ArithmeticPrimitive1::Multiplication),
		ArithmeticPrimitiveV::Division =>
			Some (ArithmeticPrimitive1::Division),
		ArithmeticPrimitiveV::GreatestCommonDivisor =>
			Some (ArithmeticPrimitive1::GreatestCommonDivisor),
		ArithmeticPrimitiveV::LeastCommonMultiple =>
			Some (ArithmeticPrimitive1::LeastCommonMultiple),
		ArithmeticPrimitiveV::Minimum =>
			Some (ArithmeticPrimitive1::Minimum),
		ArithmeticPrimitiveV::Maximum =>
			Some (ArithmeticPrimitive1::Maximum),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_v_alternative_2 (primitive : ArithmeticPrimitiveV) -> (Option<ArithmeticPrimitive2>) {
	match primitive {
		ArithmeticPrimitiveV::Addition =>
			Some (ArithmeticPrimitive2::Addition),
		ArithmeticPrimitiveV::Subtraction =>
			Some (ArithmeticPrimitive2::Subtraction),
		ArithmeticPrimitiveV::Multiplication =>
			Some (ArithmeticPrimitive2::Multiplication),
		ArithmeticPrimitiveV::Division =>
			Some (ArithmeticPrimitive2::Division),
		ArithmeticPrimitiveV::GreatestCommonDivisor =>
			Some (ArithmeticPrimitive2::GreatestCommonDivisor),
		ArithmeticPrimitiveV::LeastCommonMultiple =>
			Some (ArithmeticPrimitive2::LeastCommonMultiple),
		ArithmeticPrimitiveV::Minimum =>
			Some (ArithmeticPrimitive2::Minimum),
		ArithmeticPrimitiveV::Maximum =>
			Some (ArithmeticPrimitive2::Maximum),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_v_alternative_3 (primitive : ArithmeticPrimitiveV) -> (Option<ArithmeticPrimitive3>) {
	match primitive {
		ArithmeticPrimitiveV::Addition =>
			None,
		ArithmeticPrimitiveV::Subtraction =>
			None,
		ArithmeticPrimitiveV::Multiplication =>
			None,
		ArithmeticPrimitiveV::Division =>
			None,
		ArithmeticPrimitiveV::GreatestCommonDivisor =>
			None,
		ArithmeticPrimitiveV::LeastCommonMultiple =>
			None,
		ArithmeticPrimitiveV::Minimum =>
			None,
		ArithmeticPrimitiveV::Maximum =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_v_alternative_4 (primitive : ArithmeticPrimitiveV) -> (Option<ArithmeticPrimitive4>) {
	match primitive {
		ArithmeticPrimitiveV::Addition =>
			None,
		ArithmeticPrimitiveV::Subtraction =>
			None,
		ArithmeticPrimitiveV::Multiplication =>
			None,
		ArithmeticPrimitiveV::Division =>
			None,
		ArithmeticPrimitiveV::GreatestCommonDivisor =>
			None,
		ArithmeticPrimitiveV::LeastCommonMultiple =>
			None,
		ArithmeticPrimitiveV::Minimum =>
			None,
		ArithmeticPrimitiveV::Maximum =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_v_alternative_5 (primitive : ArithmeticPrimitiveV) -> (Option<ArithmeticPrimitive5>) {
	match primitive {
		ArithmeticPrimitiveV::Addition =>
			None,
		ArithmeticPrimitiveV::Subtraction =>
			None,
		ArithmeticPrimitiveV::Multiplication =>
			None,
		ArithmeticPrimitiveV::Division =>
			None,
		ArithmeticPrimitiveV::GreatestCommonDivisor =>
			None,
		ArithmeticPrimitiveV::LeastCommonMultiple =>
			None,
		ArithmeticPrimitiveV::Minimum =>
			None,
		ArithmeticPrimitiveV::Maximum =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_v_alternative_n (primitive : ArithmeticPrimitiveV) -> (Option<ArithmeticPrimitiveN>) {
	match primitive {
		ArithmeticPrimitiveV::Addition =>
			Some (ArithmeticPrimitiveN::Addition),
		ArithmeticPrimitiveV::Subtraction =>
			Some (ArithmeticPrimitiveN::Subtraction),
		ArithmeticPrimitiveV::Multiplication =>
			Some (ArithmeticPrimitiveN::Multiplication),
		ArithmeticPrimitiveV::Division =>
			Some (ArithmeticPrimitiveN::Division),
		ArithmeticPrimitiveV::GreatestCommonDivisor =>
			Some (ArithmeticPrimitiveN::GreatestCommonDivisor),
		ArithmeticPrimitiveV::LeastCommonMultiple =>
			Some (ArithmeticPrimitiveN::LeastCommonMultiple),
		ArithmeticPrimitiveV::Minimum =>
			Some (ArithmeticPrimitiveN::Minimum),
		ArithmeticPrimitiveV::Maximum =>
			Some (ArithmeticPrimitiveN::Maximum),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_0_attributes (_primitive : ArithmeticPrimitive0) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_0);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_1_attributes (_primitive : ArithmeticPrimitive1) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_1);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_2_attributes (_primitive : ArithmeticPrimitive2) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_2);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_3_attributes (_primitive : ArithmeticPrimitive3) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_2);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_4_attributes (_primitive : ArithmeticPrimitive4) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_4);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_5_attributes (_primitive : ArithmeticPrimitive5) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_5);
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn arithmetic_primitive_n_attributes (_primitive : ArithmeticPrimitiveN) -> (Option<ProcedureAttributes>) {
	return Some (CONSTANT_PROCEDURE_ATTRIBUTES_N);
}

