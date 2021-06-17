erl_nif::init!(
  name: "Elixir.Add",
  funcs: [add],
);

#[erl_nif::nif]
fn add(env: erl_nif::Env, a: u64, b: u64) -> Result<u64, String> {
	Ok(a + b)
}
