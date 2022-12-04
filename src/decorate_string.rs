use csl::*;

/// Let's enjoy doing some decorations
/// THis is App-dev defined
#[atom(name = "decorate_string_atom1")]
#[boot_electron]
fn enjoy_decorating() {
	let input = String::from("Tarun likes doing this");
	let decorated = csl::csl_spawn_and_wait!(decorate_string(input));
	csl::print(decorated.as_str());
}

/// A developer-defined function that takes a String and "decorates" it
#[atom(name = "decorate_string_atom2")]
#[electron(callers("decorate_string_atom1"))]
fn decorate_string(input: String) -> String {
	format!("( ͡° ͜ʖ ͡°)_/¯ {input} ( ͡° ͜ʖ ͡°)_/¯")
}
