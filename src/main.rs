extern crate osmpbf;
use osmpbf::{ElementReader, Element};
use std::time::Instant;

//pub fn show_paths(mapfp: &str) -> Result<(), osmpbf::Error> {
pub fn show_paths(mapfp: &str) {
    println!("Creating Reader");
    let reader = ElementReader::from_path(mapfp).unwrap();
    let mut ways = 0_u64;

    println!("Calculating Number of Ways");
    // Increment the counter by one for each way.
    //reader.for_each(|element| {
        //if let Element::Way(_) = element {
            //ways += 1;
        //}
    //}).unwrap();
    let now = Instant::now();
    println!("Time now: {:.2?}", now.elapsed());

    // Count the ways
    let ways = reader.par_map_reduce(
        |element| {
            match element {
                Element::Way(_) => 1,
                _ => 0,
            }
        },
        || 0_u64,      // Zero is the identity value for addition
        |a, b| a + b   // Sum the partial results
    ).unwrap();

    let elapsed = now.elapsed();
    println!("Time Elapsed: {:.2?}", elapsed);

    println!("Number of ways: {}", ways);
}

fn main () {
    let mapfp = "british-columbia-latest.osm.pbf";
    println!("Parsing {}", mapfp);
    show_paths(mapfp);
}
