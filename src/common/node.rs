use common::types::FnType;
use common::types::StructType;
use common::link::*;

use common::entity::FromEntity;
use common::entity::Entity;

#[derive(Debug, Clone)]
pub struct Node {
    id: i32,
    name: String,
    url: String,
    fn_type: FnType,
    link: Vec<Link>,
    nodes: Vec<Node>,
}

impl FromEntity for Node {
    fn from_entity(entity: Entity) -> Self {
        Node {
            id: entity.id,
            name: entity.name,
            url: entity.url,
            fn_type: entity.fn_type,
            link: Vec::new(),
            nodes: Vec::new()
        }
    }

    //TODO entity matcher for nodes
    fn from_entities(entites: Vec<Entity>) -> Vec<Self> {
        let mut node_vec = Vec::new();
        let mut link_vec = Vec::new();

        for entity in entites {
            match entity.struct_type {
                StructType::Link => {
                    link_vec.push(Link::from_entity(entity));
                },
                StructType::Node => {
                    node_vec.push(Node::from_entity(entity));
                }
            }
        }

        node_vec
    }
}