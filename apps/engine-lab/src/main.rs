//! Independent engine consumer: a warehouse courier route, with no Dreadmarch game dependency.
use dread_core::{NavGrid, Point};

fn warehouse_route() -> Vec<Point> {
    let mut floor = NavGrid::new(12, 8);
    for z in 0..8 {
        if z != 5 {
            floor.block(5, z);
        }
    }
    let route = floor.path(Point::new(1_500, 1_500), Point::new(10_500, 6_500));
    assert!(!route.is_empty());
    assert!(route.iter().all(|point| floor.contains_point(*point)));
    assert!(
        route
            .iter()
            .any(|point| point.x / 1_000 == 5 && point.z / 1_000 == 5)
    );
    route
}

fn main() {
    let route = warehouse_route();
    println!(
        "{}",
        serde_json::json!({
            "sample":"warehouse courier", "engine":"dread-core",
            "imports_dreadmarch_rules":false, "route":route,
            "verified":"Every waypoint is walkable and the route passes the only shelf opening."
        })
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn core_routes_a_distinct_domain_without_game_rules() {
        assert!(!super::warehouse_route().is_empty());
    }
}
