use super::ast::{
	argument::AstArgument,
	directive::AstDirective,
	type_::{
		directive::AstDirectiveDefinition,
		document::{AstTypeSystemDefinition, AstTypeSystemDocument},
		enum_::AstEnumTypeDefinition,
		input_object::AstInputObjectTypeDefinition,
		interface::AstInterfaceTypeDefinition,
		object::{AstFieldDefinition, AstObjectTypeDefinition},
		reference::{AstName, AstNamedType},
		scalar::AstScalarTypeDefinition,
		schema::{AstOperationType, AstRootOperationTypeDefinition, AstSchemaDefinition},
		union::AstUnionTypeDefinition,
	},
	value::{
		boolean::AstBooleanValue, enum_::AstEnumValue, float::AstFloatValue, int::AstIntValue,
		null::AstNullValue, string::AstStringValue, AstValue,
	},
};

pub trait AstVisitor {
	///
	fn visit_document(&mut self, document: &mut AstTypeSystemDocument) {
		document.visit(self);
	}

	///
	fn visit_type_definition(&mut self, type_: &mut AstTypeSystemDefinition) {
		type_.visit(self);
	}

	///
	fn visit_schema_definition(&mut self, schema: &mut AstSchemaDefinition) {
		schema.visit(self);
	}

	///
	fn visit_root_operation_type_definition(
		&mut self,
		definition: &mut AstRootOperationTypeDefinition,
	) {
		definition.visit(self);
	}

	///
	fn visit_operation_type(&mut self, operation: &mut AstOperationType) {
		_ = operation;
	}

	///
	fn visit_field_definition(&mut self, field: &mut AstFieldDefinition) {
		_ = field;
	}

	///
	fn visit_scalar_type_definition(&mut self, scalar: &mut AstScalarTypeDefinition) {
		scalar.visit(self);
	}

	///
	fn visit_object_type_definition(&mut self, _: &mut AstObjectTypeDefinition) {}

	///
	fn visit_interface_type_definition(&mut self, _: &mut AstInterfaceTypeDefinition) {}

	///
	fn visit_union_type_definition(&mut self, _: &mut AstUnionTypeDefinition) {}

	///
	fn visit_enum_type_definition(&mut self, _: &mut AstEnumTypeDefinition) {}

	///
	fn visit_input_object_type_definition(&mut self, _: &mut AstInputObjectTypeDefinition) {}

	///
	fn visit_directive_definition(&mut self, _: &mut AstDirectiveDefinition) {}

	///
	fn visit_directive(&mut self, _: &mut AstDirective) {}

	///
	fn visit_int_value(&mut self, _: &mut AstIntValue) {}

	///
	fn visit_float_value(&mut self, _: &mut AstFloatValue) {}

	///
	fn visit_string_value(&mut self, _: &mut AstStringValue) {}

	///
	fn visit_boolean_value(&mut self, _: &mut AstBooleanValue) {}

	///
	fn visit_null_value(&mut self, _: &mut AstNullValue) {}

	///
	fn visit_enum_value(&mut self, _: &mut AstEnumValue) {}

	///
	fn visit_named_type(&mut self, _: &mut AstNamedType) {}

	///
	fn visit_name(&mut self, _: &mut AstName) {}

	///
	fn visit_argument(&mut self, _: &mut AstArgument) {}

	///
	fn visit_value(&mut self, _: &mut AstValue) {}
}
