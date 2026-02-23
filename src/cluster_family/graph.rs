use crate::cluster_family::primitives::{Cluster, ClusterType};
use crate::error::Result;
use crate::knotenlexikon::lexicon::NodeId;
use dashmap::DashMap;
use std::sync::Arc;

pub struct ClusterGraph {
    clusters: Arc<DashMap<NodeId, Cluster>>,
    adjacency: Arc<DashMap<NodeId, Vec<NodeId>>>,
}

impl ClusterGraph {
    pub fn new() -> Self {
        Self {
            clusters: Arc::new(DashMap::new()),
            adjacency: Arc::new(DashMap::new()),
        }
    }

    pub fn add_cluster(&self, cluster: Cluster) -> Result<()> {
        self.clusters.insert(cluster.id.clone(), cluster);
        Ok(())
    }

    pub fn connect_clusters(&self, from: NodeId, to: NodeId) -> Result<()> {
        self.adjacency
            .entry(from)
            .or_insert_with(Vec::new)
            .push(to);
        Ok(())
    }

    pub fn get_cluster(&self, id: &NodeId) -> Result<Cluster> {
        self.clusters
            .get(id)
            .map(|c| c.clone())
            .ok_or_else(|| crate::error::NeoError::MissingNode {
                node_id: id.0.clone(),
            })
    }

    pub fn get_neighbors(&self, id: &NodeId) -> Result<Vec<NodeId>> {
        Ok(self
            .adjacency
            .get(id)
            .map(|n| n.clone())
            .unwrap_or_default())
    }

    pub fn list_clusters_by_type(&self, cluster_type: ClusterType) -> Vec<Cluster> {
        self.clusters
            .iter()
            .filter(|ref_multi| ref_multi.cluster_type == cluster_type)
            .map(|ref_multi| ref_multi.clone())
            .collect()
    }

    pub fn cluster_count(&self) -> usize {
        self.clusters.len()
    }
}

impl Default for ClusterGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ClusterGraph {
    fn clone(&self) -> Self {
        Self {
            clusters: Arc::clone(&self.clusters),
            adjacency: Arc::clone(&self.adjacency),
        }
    }
}
