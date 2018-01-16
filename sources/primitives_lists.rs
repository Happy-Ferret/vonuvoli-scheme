

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::ListPrimitive0;
	pub use super::ListPrimitive1;
	pub use super::ListPrimitive2;
	pub use super::ListPrimitive3;
	pub use super::ListPrimitive4;
	pub use super::ListPrimitive5;
	pub use super::ListPrimitiveN;
	pub use super::ListPrimitiveV;
	
	pub use super::list_primitive_0_evaluate;
	pub use super::list_primitive_1_evaluate;
	pub use super::list_primitive_2_evaluate;
	pub use super::list_primitive_3_evaluate;
	pub use super::list_primitive_4_evaluate;
	pub use super::list_primitive_5_evaluate;
	pub use super::list_primitive_n_evaluate;
	
	pub use super::list_primitive_v_alternative_0;
	pub use super::list_primitive_v_alternative_1;
	pub use super::list_primitive_v_alternative_2;
	pub use super::list_primitive_v_alternative_3;
	pub use super::list_primitive_v_alternative_4;
	pub use super::list_primitive_v_alternative_5;
	pub use super::list_primitive_v_alternative_n;
	
	pub use super::list_primitive_0_attributes;
	pub use super::list_primitive_1_attributes;
	pub use super::list_primitive_2_attributes;
	pub use super::list_primitive_3_attributes;
	pub use super::list_primitive_4_attributes;
	pub use super::list_primitive_5_attributes;
	pub use super::list_primitive_n_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive0 {
	
	ListBuild,
	ListAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive1 {
	
	PairLeft,
	PairRight,
	
	ListFirstOfFirst,
	ListRestOfFirst,
	
	ListFirstAt2,
	ListRestAt2,
	
	ListLength,
	ListClone,
	ListReverse,
	
	ListMake,
	
	ListBuild,
	ListAppend,
	
	ListFill,
	
	PairToImmutable,
	PairToMutable,
	
	ListToImmutable,
	ListToMutable,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive2 {
	
	Pair,
	
	PairLeftSet,
	PairRightSet,
	
	ListPairAt,
	ListFirstAt,
	ListRestAt,
	
	ListMake,
	
	ListBuild,
	ListAppend,
	
	ListFill,
	ListCopy,
	ListRangeClone,
	
	ListMemberByIdentity,
	ListMemberByValue,
	ListMemberByValueRecursive,
	ListAssocByIdentity,
	ListAssocByValue,
	ListAssocByValueRecursive,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive3 {
	
	ListFirstAtSet,
	ListRestAtSet,
	
	ListBuild,
	ListAppend,
	
	ListRangeFill,
	ListRangeCopy,
	ListRangeClone,
	
	ListMemberByComparator,
	ListAssocByComparator,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive4 {
	
	ListBuild,
	ListAppend,
	
	ListRangeFill,
	ListRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitive5 {
	
	ListRangeCopy,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitiveN {
	
	ListBuild,
	ListAppend,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum ListPrimitiveV {
	
	ListMake,
	ListBuild,
	ListAppend,
	
	ListRangeFill,
	ListRangeCopy,
	ListRangeClone,
	
	ListMember,
	ListAssoc,
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_0_evaluate (primitive : ListPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive0::ListBuild =>
			return list_empty () .into_0 (),
		
		ListPrimitive0::ListAppend =>
			return list_empty () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_1_evaluate (primitive : ListPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive1::PairLeft =>
			return list_first (input_1),
		
		ListPrimitive1::PairRight =>
			return list_rest (input_1),
		
		ListPrimitive1::ListFirstAt2 =>
			return list_first_at (input_1, 1),
		
		ListPrimitive1::ListRestAt2 =>
			return list_rest_at (input_1, 1),
		
		ListPrimitive1::ListFirstOfFirst =>
			return list_first (try! (list_first_ref (input_1))),
		
		ListPrimitive1::ListRestOfFirst =>
			return list_rest (try! (list_first_ref (input_1))),
		
		ListPrimitive1::ListLength =>
			return list_length (input_1) .into_0 (),
		
		ListPrimitive1::ListClone =>
			return list_clone (input_1, None),
		
		ListPrimitive1::ListReverse =>
			return list_reverse (input_1, None),
		
		ListPrimitive1::ListMake =>
			return list_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), &UNDEFINED.into (), None),
		
		ListPrimitive1::ListBuild =>
			return list_build_1 (input_1, None) .into_0 (),
		
		ListPrimitive1::ListAppend =>
			return input_1.clone () .into_0 (),
		
		ListPrimitive1::ListFill =>
			return list_fill_range (input_1, None, None, None),
		
		ListPrimitive1::PairToImmutable =>
			return try_as_pair_as_ref! (input_1) .to_immutable () .into_0 (),
		
		ListPrimitive1::PairToMutable =>
			return try_as_pair_as_ref! (input_1) .to_mutable () .into_0 (),
		
		ListPrimitive1::ListToImmutable =>
			fail_unimplemented! (0xaab9fe29), // deferred
		
		ListPrimitive1::ListToMutable =>
			fail_unimplemented! (0xf0892d44), // deferred
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_2_evaluate (primitive : ListPrimitive2, input_1 : &Value, input_2 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive2::Pair =>
			return pair (input_1, input_2, None) .into_0 (),
		
		ListPrimitive2::PairLeftSet =>
			return pair_left_set (input_1, input_2),
		
		ListPrimitive2::PairRightSet =>
			return pair_right_set (input_1, input_2),
		
		ListPrimitive2::ListPairAt =>
			return list_pair_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListFirstAt =>
			return list_first_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListRestAt =>
			return list_rest_at (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ())),
		
		ListPrimitive2::ListMake =>
			return list_make (try! (try_as_number_integer_ref! (input_1) .try_to_usize ()), input_2, None),
		
		ListPrimitive2::ListBuild =>
			return list_build_2 (input_1, input_2, None). into_0 (),
		
		ListPrimitive2::ListAppend =>
			return list_append_2 (input_1, input_2, None),
		
		ListPrimitive2::ListFill =>
			return list_fill_range (input_1, Some (input_2), None, None),
		
		ListPrimitive2::ListCopy =>
			return list_copy_range (input_1, None, input_2, None, None),
		
		ListPrimitive2::ListRangeClone =>
			return list_clone_range (input_1, Some (input_2), None, None),
		
		ListPrimitive2::ListMemberByIdentity =>
			return list_member_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByIdentity, Some (false), Some (false))),
		
		ListPrimitive2::ListMemberByValue =>
			return list_member_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false))),
		
		ListPrimitive2::ListMemberByValueRecursive =>
			return list_member_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true))),
		
		ListPrimitive2::ListAssocByIdentity =>
			return list_assoc_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByIdentity, Some (false), Some (false))),
		
		ListPrimitive2::ListAssocByValue =>
			return list_assoc_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (false))),
		
		ListPrimitive2::ListAssocByValueRecursive =>
			return list_assoc_by_comparison (input_2, input_1, Comparison::Equivalence (Equivalence::ByValue, Some (false), Some (true))),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_3_evaluate (primitive : ListPrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive3::ListFirstAtSet =>
			return list_first_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		ListPrimitive3::ListRestAtSet =>
			return list_rest_at_set (input_1, try! (try_as_number_integer_ref! (input_2) .try_to_usize ()), input_3),
		
		ListPrimitive3::ListBuild =>
			return list_build_3 (input_1, input_2, input_3, None) .into_0 (),
		
		ListPrimitive3::ListAppend =>
			return list_append_3 (input_1, input_2, input_3, None),
		
		ListPrimitive3::ListRangeFill =>
			return list_fill_range (input_1, Some (input_2), Some (input_3), None),
		
		ListPrimitive3::ListRangeCopy =>
			return list_copy_range (input_1, Some (input_2), input_3, None, None),
		
		ListPrimitive3::ListRangeClone =>
			return list_clone_range (input_1, Some (input_2), Some (input_3), None),
		
		ListPrimitive3::ListMemberByComparator =>
			return list_member_by_comparator (input_2, input_1, input_3, evaluator),
		
		ListPrimitive3::ListAssocByComparator =>
			return list_assoc_by_comparator (input_2, input_1, input_3, evaluator),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_4_evaluate (primitive : ListPrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive4::ListBuild =>
			return list_build_4 (input_1, input_2, input_3, input_4, None) .into_0 (),
		
		ListPrimitive4::ListAppend =>
			return list_append_4 (input_1, input_2, input_3, input_4, None),
		
		ListPrimitive4::ListRangeFill =>
			return list_fill_range (input_1, Some (input_2), Some (input_3), Some (input_4)),
		
		ListPrimitive4::ListRangeCopy =>
			return list_copy_range (input_1, Some (input_2), input_3, Some (input_4), None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_5_evaluate (primitive : ListPrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitive5::ListRangeCopy =>
			return list_copy_range (input_1, Some (input_2), input_3, Some (input_4), Some (input_5)),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_n_evaluate (primitive : ListPrimitiveN, inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		ListPrimitiveN::ListBuild =>
			return list_build_n (inputs, None) .into_0 (),
		
		ListPrimitiveN::ListAppend =>
			return list_append_n (inputs, None),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_0 (primitive : ListPrimitiveV) -> (Option<ListPrimitive0>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			None,
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitive0::ListBuild),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitive0::ListAppend),
		ListPrimitiveV::ListRangeFill =>
			None,
		ListPrimitiveV::ListRangeCopy =>
			None,
		ListPrimitiveV::ListRangeClone =>
			None,
		ListPrimitiveV::ListMember =>
			None,
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_1 (primitive : ListPrimitiveV) -> (Option<ListPrimitive1>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			Some (ListPrimitive1::ListMake),
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitive1::ListBuild),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitive1::ListAppend),
		ListPrimitiveV::ListRangeFill =>
			Some (ListPrimitive1::ListFill),
		ListPrimitiveV::ListRangeCopy =>
			None,
		ListPrimitiveV::ListRangeClone =>
			Some (ListPrimitive1::ListClone),
		ListPrimitiveV::ListMember =>
			None,
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_2 (primitive : ListPrimitiveV) -> (Option<ListPrimitive2>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			Some (ListPrimitive2::ListMake),
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitive2::ListBuild),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitive2::ListAppend),
		ListPrimitiveV::ListRangeFill =>
			Some (ListPrimitive2::ListFill),
		ListPrimitiveV::ListRangeCopy =>
			Some (ListPrimitive2::ListCopy),
		ListPrimitiveV::ListRangeClone =>
			Some (ListPrimitive2::ListRangeClone),
		ListPrimitiveV::ListMember =>
			Some (ListPrimitive2::ListMemberByValueRecursive),
		ListPrimitiveV::ListAssoc =>
			Some (ListPrimitive2::ListAssocByValueRecursive),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_3 (primitive : ListPrimitiveV) -> (Option<ListPrimitive3>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			None,
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitive3::ListBuild),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitive3::ListAppend),
		ListPrimitiveV::ListRangeFill =>
			Some (ListPrimitive3::ListRangeFill),
		ListPrimitiveV::ListRangeCopy =>
			Some (ListPrimitive3::ListRangeCopy),
		ListPrimitiveV::ListRangeClone =>
			Some (ListPrimitive3::ListRangeClone),
		ListPrimitiveV::ListMember =>
			Some (ListPrimitive3::ListMemberByComparator),
		ListPrimitiveV::ListAssoc =>
			Some (ListPrimitive3::ListAssocByComparator),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_4 (primitive : ListPrimitiveV) -> (Option<ListPrimitive4>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			None,
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitive4::ListBuild),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitive4::ListAppend),
		ListPrimitiveV::ListRangeFill =>
			Some (ListPrimitive4::ListRangeFill),
		ListPrimitiveV::ListRangeCopy =>
			Some (ListPrimitive4::ListRangeCopy),
		ListPrimitiveV::ListRangeClone =>
			None,
		ListPrimitiveV::ListMember =>
			None,
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_5 (primitive : ListPrimitiveV) -> (Option<ListPrimitive5>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			None,
		ListPrimitiveV::ListBuild =>
			None,
		ListPrimitiveV::ListAppend =>
			None,
		ListPrimitiveV::ListRangeFill =>
			None,
		ListPrimitiveV::ListRangeCopy =>
			Some (ListPrimitive5::ListRangeCopy),
		ListPrimitiveV::ListRangeClone =>
			None,
		ListPrimitiveV::ListMember =>
			None,
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_v_alternative_n (primitive : ListPrimitiveV) -> (Option<ListPrimitiveN>) {
	match primitive {
		ListPrimitiveV::ListMake =>
			None,
		ListPrimitiveV::ListBuild =>
			Some (ListPrimitiveN::ListBuild),
		ListPrimitiveV::ListAppend =>
			Some (ListPrimitiveN::ListAppend),
		ListPrimitiveV::ListRangeFill =>
			None,
		ListPrimitiveV::ListRangeCopy =>
			None,
		ListPrimitiveV::ListRangeClone =>
			None,
		ListPrimitiveV::ListMember =>
			None,
		ListPrimitiveV::ListAssoc =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_0_attributes (_primitive : ListPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_1_attributes (_primitive : ListPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_2_attributes (_primitive : ListPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_3_attributes (_primitive : ListPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_4_attributes (_primitive : ListPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_5_attributes (_primitive : ListPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn list_primitive_n_attributes (_primitive : ListPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

