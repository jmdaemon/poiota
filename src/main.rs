extern crate osmpbf;
extern crate osmpbfreader;

use osmpbf::{ElementReader, Element};
use std::process::exit;
use std::time::Instant;
use std::path::Path;

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
    let maps_dir = "osm".to_string();
    //let mapfp = maps_dir.push_str("/british-columbia-latest.osm.pbf");
    let mapfp = format!("{}/{}", maps_dir, "/british-columbia-latest.osm.pbf");
    let path = Path::new(&mapfp);

    //let path = std::path::Path::new(&mapfp);
    //let r = std::fs::File::open(&path).unwrap();
    
    //let mapf = std::fs::File::open(&mapfp).unwrap();

    let now = Instant::now();
    println!("Time now: {:.2?}", now.elapsed());

    let map = std::fs::File::open(&path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(map);
    
    let objs = pbf.get_objs_and_deps(|obj| {
        obj.is_way() && obj.tags().contains_key("highway")
    }).unwrap();

    for (id, obj) in &objs {
        println!("{:?}: {:?}", id, obj);
    }

    let elapsed = now.elapsed();
    println!("Time Elapsed: {:.2?}", elapsed);

    //let mut nb = 0;
    //for _obj in pbf.iter().map(Result::unwrap) {
        //nb += 1;
    //}

    //let mut pbf = osmpbfreader::OsmPbfReader::new(&mapfp);
    //for obj in pbf.iter() {
    //// error handling:
    //let obj = obj.unwrap_or_else(|e| {println!("{:?}", e); exit(1)});

    //println!("{:?}", obj);
//}
    //println!("Parsing {}", mapfp);
    //show_paths(mapfp);
}
