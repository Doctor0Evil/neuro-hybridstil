use serde::{Deserialize, Serialize};
use crate::knotenlexikon::lexicon::NodeId;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClusterType {
    NodeCluster,
    CoreCluster,
    LayerCluster,
    ChannelCluster,
    EventCluster,
    LanguageCluster,
    MeaningCluster,
    RoutingCluster,
    InterfaceCluster,
    AdaptationCluster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub id: NodeId,
    pub cluster_type: ClusterType,
    pub german_name: String,
    pub code_label: String,
    pub member_nodes: Vec<NodeId>,
    pub metadata: HashMap<String, String>,
}

impl Cluster {
    pub fn new(
        id: NodeId,
        cluster_type: ClusterType,
        german_name: String,
        code_label: String,
    ) -> Self {
        Self {
            id,
            cluster_type,
            german_name,
            code_label,
            member_nodes: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_member(&mut self, node_id: NodeId) {
        if !self.member_nodes.contains(&node_id) {
            self.member_nodes.push(node_id);
        }
    }

    pub fn remove_member(&mut self, node_id: &NodeId) {
        self.member_nodes.retain(|n| n != node_id);
    }

    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

impl ToString for ClusterType {
    fn to_string(&self) -> String {
        match self {
            ClusterType::NodeCluster => "NodeCluster".to_string(),
            ClusterType::CoreCluster => "CoreCluster".to_string(),
            ClusterType::LayerCluster => "LayerCluster".to_string(),
            ClusterType::ChannelCluster => "ChannelCluster".to_string(),
            ClusterType::EventCluster => "EventCluster".to_string(),
            ClusterType::LanguageCluster => "LanguageCluster".to_string(),
            ClusterType::MeaningCluster => "MeaningCluster".to_string(),
            ClusterType::RoutingCluster => "RoutingCluster".to_string(),
            ClusterType::InterfaceCluster => "InterfaceCluster".to_string(),
            ClusterType::AdaptationCluster => "AdaptationCluster".to_string(),
        }
    }
}
