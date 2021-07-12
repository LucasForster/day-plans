mod capacities;
mod categories;
mod districts;
mod filters;
mod graph;
mod io;
mod levels;
mod modes;
mod purposes;
mod sankey;
mod search;
mod time_bins;
mod trips;

fn main() {
    sankey::main();
    search::search();
}
