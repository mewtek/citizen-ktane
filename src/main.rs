mod solvers;

fn main() {
    let wires = vec!["black", "white", "black"];
    let bomb = solvers::Bomb {
        serial: String::from("Z8CP06"),
        car: false,
        frq: false,
        battery_count: 3,
        strikes: 0,
    };

    solvers::defuse_wires(wires, bomb);
}