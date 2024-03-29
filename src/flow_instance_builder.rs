use uuid::Uuid;

use crate::model::EntityInstance;
use crate::model::FlowInstance;
use crate::model::RelationInstance;

#[allow(dead_code)]
pub struct FlowInstanceBuilder {
    id: Uuid,
    name: String,
    description: String,
    wrapper: EntityInstance,
    entity_instances: Vec<EntityInstance>,
    relation_instances: Vec<RelationInstance>,
}

#[allow(dead_code)]
impl FlowInstanceBuilder {
    pub fn new(wrapper_entity_instance: EntityInstance) -> FlowInstanceBuilder {
        let entity_instances = vec![wrapper_entity_instance.clone()];
        FlowInstanceBuilder {
            id: wrapper_entity_instance.id,
            name: String::from(""),
            description: String::from(""),
            wrapper: wrapper_entity_instance,
            entity_instances,
            relation_instances: Vec::new(),
        }
    }

    pub fn name<S: Into<String>>(&mut self, name: S) -> &mut FlowInstanceBuilder {
        self.name = name.into();
        self
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut FlowInstanceBuilder {
        self.description = description.into();
        self
    }

    pub fn entity(&mut self, entity_instance: EntityInstance) -> &mut FlowInstanceBuilder {
        self.entity_instances.push(entity_instance);
        self
    }

    pub fn relation(&mut self, relation_instance: RelationInstance) -> &mut FlowInstanceBuilder {
        self.relation_instances.push(relation_instance);
        self
    }

    pub fn build(&self) -> FlowInstance {
        let mut flow_instance = FlowInstance::from(self.wrapper.clone());
        flow_instance.name = self.name.clone();
        flow_instance.description = self.description.clone();
        flow_instance.entity_instances = self.entity_instances.clone();
        flow_instance.relation_instances = self.relation_instances.clone();
        flow_instance
    }
}
