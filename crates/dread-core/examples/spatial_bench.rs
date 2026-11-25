//! Equivalent-work broad-phase comparison. Includes every unit and query in both
//! paths, charges index construction to the indexed path, and checks exact result
//! fingerprints. This measures spatial queries, not renderer frame rate.

use dread_core::{Point, SpatialIndex};
use std::hint::black_box;
use std::time::Instant;

fn fingerprint(query: usize, index: usize) -> u64 {
    (index as u64 + 1)
        .wrapping_mul(0x9e3779b97f4a7c15)
        .rotate_left((query % 64) as u32)
        .wrapping_add(query as u64)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let unit_count = args
        .get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(20_000);
    let query_count = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1_000);
    let mut state = 0x852f91e47a0d3c65u64;
    let mut coordinate = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state % 500_000) as i32 - 250_000
    };
    let points: Vec<_> = (0..unit_count)
        .map(|_| Point::new(coordinate(), coordinate()))
        .collect();
    let queries: Vec<_> = (0..query_count)
        .map(|_| Point::new(coordinate(), coordinate()))
        .collect();
    let radius = 7_500;
    let distance_limit = (radius as u64).pow(2);
    let mut naive_times = Vec::new();
    let mut indexed_times = Vec::new();
    let mut last_result = (0usize, 0u64);
    for _ in 0..5 {
        let start = Instant::now();
        let mut slow = (0usize, 0u64);
        for (query, center) in queries.iter().enumerate() {
            for (index, point) in points.iter().enumerate() {
                if center.distance_squared(*point) <= distance_limit {
                    slow.0 += 1;
                    slow.1 = slow.1.wrapping_add(fingerprint(query, index));
                }
            }
        }
        black_box(slow);
        naive_times.push(start.elapsed().as_secs_f64() * 1_000.0);
        let start = Instant::now();
        let mut spatial = SpatialIndex::new(5_000);
        spatial.rebuild(points.iter().copied().enumerate());
        let mut fast = (0usize, 0u64);
        for (query, center) in queries.iter().enumerate() {
            spatial.for_each_candidate(*center, radius, |index| {
                if center.distance_squared(points[index]) <= distance_limit {
                    fast.0 += 1;
                    fast.1 = fast.1.wrapping_add(fingerprint(query, index));
                }
            });
        }
        black_box(fast);
        indexed_times.push(start.elapsed().as_secs_f64() * 1_000.0);
        assert_eq!(slow, fast, "query outputs differ");
        last_result = fast;
    }
    naive_times.sort_by(f64::total_cmp);
    indexed_times.sort_by(f64::total_cmp);
    println!(
        "{{\"units\":{unit_count},\"queries\":{query_count},\"rounds\":5,\"includes_index_build\":true,\"naive_median_ms\":{:.4},\"indexed_median_ms\":{:.4},\"speedup\":{:.3},\"matched_pairs\":{},\"result_checksum\":\"{:016x}\",\"equivalent\":true}}",
        naive_times[2],
        indexed_times[2],
        naive_times[2] / indexed_times[2],
        last_result.0,
        last_result.1
    );
}
