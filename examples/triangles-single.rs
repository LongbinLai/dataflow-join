// extern crate mmap;
// extern crate time;
// extern crate dataflow_join;

extern crate graph_map;

use std::cmp::Ordering::*;

use graph_map::GraphMMap;

// use dataflow_join::graph::{GraphTrait, GraphMMap, gallop};

fn main () {
    if let Some(source) = std::env::args().skip(1).next() {
        println!("triangles: {:?}", raw_triangles(&GraphMMap::new(&source)));
    }
    else {
        println!("usage: <source>");
    }
}

fn raw_triangles(graph: &GraphMMap) -> u64 {

    let mut count = 0;
    for a in 0..graph.nodes() {
        if graph.edges(a).len() > 0 {
            count += 1;
        }
    }

    println!("count: {}", count);

    let mut count = 0;
    for a in 0..graph.nodes() {
        let aaa = graph.edges(a);
        // println!("degr[{}]: {}", a, aaa.len());
        for &b in aaa {
            let bbb = graph.edges(b as usize);
            count += if aaa.len() < bbb.len() { intersect(aaa, bbb) }
                     else                     { intersect(bbb, aaa) };
        }
    }
    count
}

fn intersect(mut aaa: &[u32], mut bbb: &[u32]) -> u64 {
    let mut count = 0;
    // magic gallop overhead # is 4
    if aaa.len() < bbb.len() / 4 {
        for a in aaa {
            bbb = gallop(bbb, a);
            if bbb.len() > 0 && &bbb[0] == a { count += 1; }
        }
    }
    else {
        while aaa.len() > 0 && bbb.len() > 0 {
            match aaa[0].cmp(&bbb[0]) {
                Greater => { bbb = &bbb[1..]; },
                Less    => { aaa = &aaa[1..]; },
                Equal   => { aaa = &aaa[1..];
                             bbb = &bbb[1..];
                             count += 1;
                           },
            }
        }
    }
    count
}


#[inline(always)]
pub fn gallop<'a, T: Ord>(mut slice: &'a [T], value: &T) -> &'a [T] {
    // if empty slice, or already >= element, return
    if slice.len() > 0 && &slice[0] < value {
        let mut step = 1;
        while step < slice.len() && &slice[step] < value {
            slice = &slice[step..];
            step = step << 1;
        }

        step = step >> 1;
        while step > 0 {
            if step < slice.len() && &slice[step] < value {
                slice = &slice[step..];
            }
            step = step >> 1;
        }

        slice = &slice[1..]; // advance one, as we always stayed < value
    }

    return slice;
}