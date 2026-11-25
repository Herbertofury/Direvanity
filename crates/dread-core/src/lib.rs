//! Reusable fixed-point geometry, deterministic navigation and broad-phase queries.
//! This crate has no dependency on a renderer, game-specific unit, browser or clock.

use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

/// Millimetres per world/grid unit. Simulation math never uses floating point.
pub const CELL_SIZE: i32 = 1_000;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub z: i32,
}

impl Point {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// Squared distance, saturating only beyond the representable u64 range.
    pub fn distance_squared(self, other: Self) -> u64 {
        let dx = i128::from(self.x) - i128::from(other.x);
        let dz = i128::from(self.z) - i128::from(other.z);
        (dx * dx + dz * dz).min(i128::from(u64::MAX)) as u64
    }

    pub fn step_towards(self, target: Self, speed: i32) -> Self {
        if speed <= 0 {
            return self;
        }
        let dx = i128::from(target.x) - i128::from(self.x);
        let dz = i128::from(target.z) - i128::from(self.z);
        let distance_squared = (dx * dx + dz * dz) as u128;
        if distance_squared <= (speed as u128).pow(2) {
            return target;
        }
        let root = distance_squared.isqrt();
        let distance = root + u128::from(root * root < distance_squared);
        let mut result = Self {
            x: (i128::from(self.x) + dx * i128::from(speed) / distance as i128) as i32,
            z: (i128::from(self.z) + dz * i128::from(speed) / distance as i128) as i32,
        };
        // Rounding a sub-cell diagonal must never stall a speed-one actor.
        if result == self {
            if dx.abs() >= dz.abs() {
                result.x += dx.signum() as i32;
            } else {
                result.z += dz.signum() as i32;
            }
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavGrid {
    pub width: u16,
    pub height: u16,
    /// Row-major, indexed as z * width + x.
    pub blocked: Vec<bool>,
}

impl NavGrid {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            blocked: vec![false; usize::from(width) * usize::from(height)],
        }
    }

    pub fn validate(&self) -> bool {
        self.width > 0
            && self.height > 0
            && self.blocked.len() == usize::from(self.width) * usize::from(self.height)
    }

    pub fn is_walkable(&self, x: i32, z: i32) -> bool {
        x >= 0
            && z >= 0
            && x < i32::from(self.width)
            && z < i32::from(self.height)
            && self
                .blocked
                .get(z as usize * usize::from(self.width) + x as usize)
                .is_some_and(|b| !b)
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.is_walkable(point.x.div_euclid(CELL_SIZE), point.z.div_euclid(CELL_SIZE))
    }

    pub fn block(&mut self, x: i32, z: i32) {
        if x >= 0 && z >= 0 && x < i32::from(self.width) && z < i32::from(self.height) {
            self.blocked[z as usize * usize::from(self.width) + x as usize] = true;
        }
    }

    pub fn cell_center(x: i32, z: i32) -> Point {
        Point::new(x * CELL_SIZE + CELL_SIZE / 2, z * CELL_SIZE + CELL_SIZE / 2)
    }

    /// Resolve a click on a wall or outside the world to the closest open cell.
    /// A blocked destination uses an exhaustive closest-cell search without a radius cap.
    pub fn nearest_walkable(&self, point: Point) -> Option<Point> {
        if !self.validate() {
            return None;
        }
        if self.contains_point(point) {
            return Some(point);
        }
        let mut best = None;
        for z in 0..i32::from(self.height) {
            for x in 0..i32::from(self.width) {
                if !self.is_walkable(x, z) {
                    continue;
                }
                let candidate = Self::cell_center(x, z);
                let key = (candidate.distance_squared(point), z, x);
                if best.as_ref().is_none_or(|(old, _)| key < *old) {
                    best = Some((key, candidate));
                }
            }
        }
        best.map(|(_, result)| result)
    }

    /// Exact integer grid traversal. A line touching a blocked corner is occluded.
    pub fn line_of_sight(&self, from: Point, to: Point) -> bool {
        if !self.contains_point(from) || !self.contains_point(to) {
            return false;
        }
        let (mut x, mut z) = (from.x.div_euclid(CELL_SIZE), from.z.div_euclid(CELL_SIZE));
        let (end_x, end_z) = (to.x.div_euclid(CELL_SIZE), to.z.div_euclid(CELL_SIZE));
        let dx = i64::from(to.x) - i64::from(from.x);
        let dz = i64::from(to.z) - i64::from(from.z);
        let (sx, sz) = (dx.signum() as i32, dz.signum() as i32);
        let (adx, adz) = (i128::from(dx.abs()), i128::from(dz.abs()));
        let mut distance_x = i128::from(if sx > 0 {
            (x + 1) * CELL_SIZE - from.x
        } else {
            from.x - x * CELL_SIZE
        });
        let mut distance_z = i128::from(if sz > 0 {
            (z + 1) * CELL_SIZE - from.z
        } else {
            from.z - z * CELL_SIZE
        });
        while x != end_x || z != end_z {
            let cross_x = distance_x * adz;
            let cross_z = distance_z * adx;
            if dx == 0 || (dz != 0 && cross_z < cross_x) {
                z += sz;
                distance_z += i128::from(CELL_SIZE);
            } else if dz == 0 || cross_x < cross_z {
                x += sx;
                distance_x += i128::from(CELL_SIZE);
            } else {
                if !self.is_walkable(x + sx, z) || !self.is_walkable(x, z + sz) {
                    return false;
                }
                x += sx;
                z += sz;
                distance_x += i128::from(CELL_SIZE);
                distance_z += i128::from(CELL_SIZE);
            }
            if !self.is_walkable(x, z) {
                return false;
            }
        }
        true
    }

    /// Deterministic eight-neighbour A*, excluding diagonal corner cutting.
    /// Returns an empty path for an unreachable destination. The first point is
    /// the next cell, and the last point preserves the walkable exact destination.
    pub fn path(&self, start: Point, destination: Point) -> Vec<Point> {
        if !self.validate() || !self.contains_point(start) {
            return Vec::new();
        }
        let Some(goal) = self.nearest_walkable(destination) else {
            return Vec::new();
        };
        let start_cell = (start.x.div_euclid(CELL_SIZE), start.z.div_euclid(CELL_SIZE));
        let goal_cell = (goal.x.div_euclid(CELL_SIZE), goal.z.div_euclid(CELL_SIZE));
        if start_cell == goal_cell {
            return vec![goal];
        }
        let index = |x: i32, z: i32| z as usize * usize::from(self.width) + x as usize;
        let start_idx = index(start_cell.0, start_cell.1);
        let goal_idx = index(goal_cell.0, goal_cell.1);
        let mut costs = vec![u64::MAX; self.blocked.len()];
        let mut parents = vec![usize::MAX; self.blocked.len()];
        let mut open = BinaryHeap::new();
        costs[start_idx] = 0;
        open.push(Reverse((heuristic(start_cell, goal_cell), 0u64, start_idx)));
        while let Some(Reverse((_, cost, current))) = open.pop() {
            if cost != costs[current] {
                continue;
            }
            if current == goal_idx {
                let mut result = vec![goal];
                let mut cursor = current;
                while parents[cursor] != start_idx {
                    cursor = parents[cursor];
                    if cursor == usize::MAX {
                        return Vec::new();
                    }
                    result.push(Self::cell_center(
                        (cursor % usize::from(self.width)) as i32,
                        (cursor / usize::from(self.width)) as i32,
                    ));
                }
                result.reverse();
                return result;
            }
            let x = (current % usize::from(self.width)) as i32;
            let z = (current / usize::from(self.width)) as i32;
            for (dx, dz) in [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ] {
                let nx = x + dx;
                let nz = z + dz;
                if !self.is_walkable(nx, nz)
                    || (dx != 0
                        && dz != 0
                        && (!self.is_walkable(x + dx, z) || !self.is_walkable(x, z + dz)))
                {
                    continue;
                }
                let next = index(nx, nz);
                let next_cost = cost + if dx == 0 || dz == 0 { 1_000 } else { 1_414 };
                if next_cost < costs[next] {
                    costs[next] = next_cost;
                    parents[next] = current;
                    open.push(Reverse((
                        next_cost + heuristic((nx, nz), goal_cell),
                        next_cost,
                        next,
                    )));
                }
            }
        }
        Vec::new()
    }
}

fn heuristic((x, z): (i32, i32), (gx, gz): (i32, i32)) -> u64 {
    let dx = (x - gx).unsigned_abs() as u64;
    let dz = (z - gz).unsigned_abs() as u64;
    1_000 * dx.max(dz) + 414 * dx.min(dz)
}

/// Broad-phase spatial hash with stable insertion order and no unit-count limit.
/// Buckets are retained between rebuilds to reuse allocation. Queries enumerate
/// every intersecting bucket; callers apply exact distance/team filters.
#[derive(Debug, Clone)]
pub struct SpatialIndex {
    bucket_size: i32,
    buckets: BTreeMap<(i32, i32), Vec<usize>>,
}

impl Default for SpatialIndex {
    fn default() -> Self {
        Self::new(4 * CELL_SIZE)
    }
}

impl SpatialIndex {
    pub fn new(bucket_size: i32) -> Self {
        assert!(bucket_size > 0, "spatial bucket size must be positive");
        Self {
            bucket_size,
            buckets: BTreeMap::new(),
        }
    }

    pub fn rebuild(&mut self, positions: impl IntoIterator<Item = (usize, Point)>) {
        for entries in self.buckets.values_mut() {
            entries.clear();
        }
        for (index, point) in positions {
            let cell = (
                point.x.div_euclid(self.bucket_size),
                point.z.div_euclid(self.bucket_size),
            );
            self.buckets.entry(cell).or_default().push(index);
        }
    }

    pub fn for_each_candidate(&self, center: Point, radius: i32, mut visit: impl FnMut(usize)) {
        if radius < 0 {
            return;
        }
        let xmin = center.x.saturating_sub(radius).div_euclid(self.bucket_size);
        let xmax = center.x.saturating_add(radius).div_euclid(self.bucket_size);
        let zmin = center.z.saturating_sub(radius).div_euclid(self.bucket_size);
        let zmax = center.z.saturating_add(radius).div_euclid(self.bucket_size);
        for z in zmin..=zmax {
            for x in xmin..=xmax {
                if let Some(entries) = self.buckets.get(&(x, z)) {
                    for &index in entries {
                        visit(index);
                    }
                }
            }
        }
    }
}

/// Stable FNV-1a digest for replay diagnostics, not a cryptographic integrity check.
pub fn stable_hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_routes_around_wall_without_cutting_corners() {
        let mut grid = NavGrid::new(9, 9);
        for z in 0..7 {
            grid.block(4, z);
        }
        let path = grid.path(NavGrid::cell_center(2, 2), NavGrid::cell_center(6, 2));
        assert!(!path.is_empty());
        assert!(path.iter().any(|p| p.z >= 7_000));
        assert!(path.iter().all(|p| grid.contains_point(*p)));
        assert_eq!(path.last(), Some(&NavGrid::cell_center(6, 2)));
    }

    #[test]
    fn trapped_start_does_not_cut_diagonal_corner() {
        let mut grid = NavGrid::new(3, 3);
        grid.block(1, 0);
        grid.block(0, 1);
        assert!(
            grid.path(Point::new(500, 500), Point::new(2_500, 2_500))
                .is_empty()
        );
    }

    #[test]
    fn nearest_handles_negative_and_fully_blocked_map() {
        let mut grid = NavGrid::new(3, 3);
        assert_eq!(
            grid.nearest_walkable(Point::new(-4_000, -2_000)),
            Some(Point::new(500, 500))
        );
        grid.blocked.fill(true);
        assert_eq!(grid.nearest_walkable(Point::new(500, 500)), None);
    }

    #[test]
    fn spatial_candidates_preserve_exact_radius_results() {
        let points: Vec<_> = (0..2_000)
            .map(|i| Point::new((i * 7919 % 100_000) - 50_000, (i * 1223 % 70_000) - 35_000))
            .collect();
        let mut index = SpatialIndex::default();
        index.rebuild(points.iter().copied().enumerate());
        for center in [
            Point::new(0, 0),
            Point::new(-41_000, 23_000),
            Point::new(49_000, -34_000),
        ] {
            let mut fast = Vec::new();
            index.for_each_candidate(center, 5_500, |i| {
                if center.distance_squared(points[i]) <= 5_500u64.pow(2) {
                    fast.push(i);
                }
            });
            fast.sort_unstable();
            let slow: Vec<_> = points
                .iter()
                .enumerate()
                .filter_map(|(i, p)| (center.distance_squared(*p) <= 5_500u64.pow(2)).then_some(i))
                .collect();
            assert_eq!(fast, slow);
        }
    }

    #[test]
    fn movement_is_integer_and_does_not_overshoot() {
        let origin = Point::new(0, 0);
        let target = Point::new(3_000, 4_000);
        assert_eq!(origin.step_towards(target, 1_000), Point::new(600, 800));
        assert_eq!(origin.step_towards(target, 6_000), target);
        assert_eq!(origin.step_towards(target, 0), origin);
        let diagonal = Point::new(1, 1);
        let first = origin.step_towards(diagonal, 1);
        assert_eq!(first.distance_squared(origin), 1);
        assert_eq!(first.step_towards(diagonal, 1), diagonal);
        let huge =
            Point::new(i32::MIN, i32::MIN).step_towards(Point::new(i32::MAX, i32::MAX), 1_000);
        assert!(huge.distance_squared(Point::new(i32::MIN, i32::MIN)) <= 1_000_000);
    }

    #[test]
    fn line_of_sight_blocks_walls_and_diagonal_cracks() {
        let mut grid = NavGrid::new(8, 8);
        assert!(grid.line_of_sight(Point::new(500, 500), Point::new(6_500, 3_500)));
        grid.block(1, 0);
        assert!(!grid.line_of_sight(Point::new(500, 500), Point::new(3_500, 3_500)));
        assert!(grid.line_of_sight(Point::new(500, 500), Point::new(500, 7_500)));
        grid.block(0, 4);
        assert!(!grid.line_of_sight(Point::new(500, 500), Point::new(500, 7_500)));
    }
}
