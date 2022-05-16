use fdg_sim::{
    petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper, Simulation,
    SimulationParameters, force::Resize,
};

#[macroquad::main("Force Graph Lattice Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();
    let mut indices: Vec<NodeIndex> = Vec::new();

    let size = 25;

    for x in 0..size {
        for y in 0..size {
            indices.push(graph.add_force_node(format!("{x},{y}"), ()));
        }
    }

    for y in 0..size {
        for x in 0..size {
            if x != 0 {
                graph.add_edge(indices[(size * y) + x], indices[((size * y) + x) - 1], ());
            }

            if y != 0 {
                graph.add_edge(indices[(size * y) + x], indices[(size * (y - 1)) + x], ());
            }
        }
    }

    fdg_macroquad::run_window(&mut Simulation::from_graph(
        &graph,
        SimulationParameters::from_force(Resize::default()),
    ))
    .await;
}