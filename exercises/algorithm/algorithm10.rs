/*
    graph
    This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    /// 创建一个新的无向图实例
    /// 
    /// # Returns
    /// 
    /// 一个空的无向图
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    
    /// 获取邻接表的可变引用
    /// 
    /// # Returns
    /// 
    /// 邻接表的可变引用
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    
    /// 获取邻接表的不可变引用
    /// 
    /// # Returns
    /// 
    /// 邻接表的不可变引用
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    
    /// 在无向图中添加边
    /// 
    /// # Parameters
    /// 
    /// * `edge` - 边的元组，包含起始节点、目标节点和权重
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from_node, to_node, weight) = edge;
        
        // 确保节点存在
        self.add_node(from_node);
        self.add_node(to_node);
        
        // 添加从 from_node 到 to_node 的边
        if let Some(edges) = self.adjacency_table_mutable().get_mut(from_node) {
            edges.push((String::from(to_node), weight));
        }
        
        // 添加从 to_node 到 from_node 的边（因为是无向图）
        if let Some(edges) = self.adjacency_table_mutable().get_mut(to_node) {
            edges.push((String::from(from_node), weight));
        }
    }
}
pub trait Graph {
    /// 创建一个新的图实例
    /// 
    /// # Returns
    /// 
    /// 一个新的空图
    fn new() -> Self;
    
    /// 获取邻接表的可变引用
    /// 
    /// # Returns
    /// 
    /// 邻接表的可变引用
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    
    /// 获取邻接表的不可变引用
    /// 
    /// # Returns
    /// 
    /// 邻接表的不可变引用
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    
    /// 向图中添加节点
    /// 
    /// # Parameters
    /// 
    /// * `node` - 要添加的节点名称
    /// 
    /// # Returns
    /// 
    /// 节点添加是否成功
    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            false // 节点已存在，添加失败
        } else {
            self.adjacency_table_mutable().insert(String::from(node), Vec::new());
            true // 成功添加节点
        }
    }
    
    /// 在图中添加边
    /// 
    /// # Parameters
    /// 
    /// * `edge` - 边的元组，包含起始节点、目标节点和权重
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
    }
    
    /// 检查图中是否包含指定节点
    /// 
    /// # Parameters
    /// 
    /// * `node` - 要检查的节点名称
    /// 
    /// # Returns
    /// 
    /// 节点是否存在于图中
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    
    /// 获取图中所有节点的集合
    /// 
    /// # Returns
    /// 
    /// 图中所有节点的引用集合
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    
    /// 获取图中所有边的列表
    /// 
    /// # Returns
    /// 
    /// 图中所有边的列表，每个元素是一个元组，包含起始节点、目标节点和权重
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}