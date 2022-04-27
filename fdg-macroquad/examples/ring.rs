use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, Dimensions};

#[macroquad::main("Force Graph Ring Demo")]
async fn main() {
    pretty_env_logger::init();

    let mut graph: ForceGraph<()> = ForceGraph::default();

    let nodes = 100;

    graph.add_force_node("", ());
    for x in 1..nodes {
        graph.add_force_node("", ());
        graph.add_edge(x.into(), (x - 1).into(), ());
    }
    graph.add_edge(0.into(), (nodes - 1).into(), ());

    let center = graph.add_force_node("", ());
    for x in 0..nodes {
        graph.add_edge(x.into(), center, ());
    }

    let params = SimulationParameters {
        dimensions: Dimensions::Three,
        ..Default::default()
    };

    let mut sim = Simulation::from_graph(graph, params);

    fdg_macroquad::run_window(&mut sim).await;
}
