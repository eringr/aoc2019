
use petgraph::graphmap::UnGraphMap;
use petgraph::algo::astar;
use arrayvec::ArrayString;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;

fn populate_graph(g: &mut UnGraphMap::<ArrayString::<[u8; 3]>, i32>)
    -> Result<(), io::Error> 
{
    let f = File::open("input")?;
    let br = BufReader::new(f);
    for line in br.lines() {
        let input = line?;
        g.add_edge(
            ArrayString::from(&input[4..7]).expect(""),
            ArrayString::from(&input[0..3]).expect(""),
            1
        );
    }
    Ok(())
}

fn main() {
    let mut g = UnGraphMap::new();
    populate_graph(&mut g).expect("");
    let directs = g.node_count() - 1;
    println!("{} direct orbits", directs);
    let mut indirects = 0;
    for n in g.nodes() {
        // Searching undirected increases time by order of magnitude (a few
        // seconds) vs directed but only have to run once and saves dev time
        // for next part
        if let Some((d, _))  = 
            astar(&g, n, |s| {s.as_str() == "COM"}, |_| {1}, |_| {1})
        {
            indirects += std::cmp::max(d, 1) - 1;
        }
    }
    println!("{} indirect orbits", indirects);
    println!("{} total orbits", directs + indirects);

    if let Some((d, _))  = 
        astar(
            &g,
            ArrayString::from("YOU").expect(""),
            |s| {s.as_str() == "SAN"},
            |_| {1}, |_| {1}
        )
    {
        println!("YOU are {} bodies away from SAN", d - 2);
    } else {
        println!("Couldn't find SAN ‾\\_(ツ)_/‾");
    }
}
