use serde_json::json;

use crate::model::DataType;
use crate::model::EntityTypeId;
use crate::model::ExtensionTypeId;
use crate::model::FlowTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::RelationInstanceTypeId;
use crate::model::RelationTypeId;
use crate::tests::utils::r_string;
use crate::EntityInstanceBuilder;
use crate::FlowTypeBuilder;
use crate::RelationInstanceBuilder;

#[test]
fn flow_type_builder_test() {
    let entity_type_1_namespace = r_string();
    let entity_type_1_name = r_string();
    let entity_type_1_ty = EntityTypeId::new_from_type(&entity_type_1_namespace, &entity_type_1_name);
    let wrapper_entity_instance = EntityInstanceBuilder::new(entity_type_1_ty).build();

    let entity_type_2_namespace = r_string();
    let entity_type_2_name = r_string();
    let entity_instance_2 = EntityInstanceBuilder::new_from_type(&entity_type_2_namespace, &entity_type_2_name).build();

    let relation_type_namespace = r_string();
    let relation_type_name = r_string();
    let relation_type_ty = RelationTypeId::new_from_type(&relation_type_namespace, &relation_type_name);
    let ri_ty = RelationInstanceTypeId::new_unique_id(relation_type_ty);
    let relation_instance = RelationInstanceBuilder::new(wrapper_entity_instance.id, ri_ty.clone(), entity_instance_2.id).build();

    let flow_type_namespace = r_string();
    let flow_type_name = r_string();
    let flow_type_ty = FlowTypeId::new_from_type(&flow_type_namespace, &flow_type_name);
    let description = r_string();
    let variable_1_name = r_string();
    let variable_2_name = r_string();
    let extension_namespace = r_string();
    let extension_1_name = r_string();
    let extension_2_name = r_string();
    let flow_type = FlowTypeBuilder::new(&flow_type_ty, wrapper_entity_instance.clone())
        .description(&description)
        .entity_instance(entity_instance_2)
        .relation_instance(relation_instance)
        .variable(variable_1_name.clone(), DataType::Bool)
        .variable(variable_2_name.clone(), DataType::String)
        .extension(&extension_namespace, &extension_1_name, json!(true))
        .extension(&extension_namespace, &extension_2_name, json!(true))
        .build();
    assert_eq!(flow_type_namespace, flow_type.namespace());
    assert_eq!(flow_type_name, flow_type.type_name());
    assert_eq!(flow_type_ty, flow_type.ty);
    assert_eq!(wrapper_entity_instance.ty, flow_type.wrapper_type());
    assert_eq!(description, flow_type.description);
    assert_eq!(2, flow_type.entity_instances().len());
    assert_eq!(1, flow_type.relation_instances().len());
    let extension_1_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_1_name);
    assert!(flow_type.has_extension(&extension_1_ty));
    let extension_2_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_2_name);
    assert!(flow_type.has_extension(&extension_2_ty));
    let non_existing_extension = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
    assert!(!flow_type.has_extension(&non_existing_extension));
    assert!(flow_type.has_variable(variable_1_name.clone()));
    assert!(flow_type.has_variable(variable_2_name.clone()));
    assert!(!flow_type.has_variable(r_string()));
}
