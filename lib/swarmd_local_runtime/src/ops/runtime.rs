use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::ModuleSpecifier;
use deno_core::OpState;

deno_core::extension!(
  swarmd_runtime,
  ops = [op_main_module],
  options = { main_module: ModuleSpecifier },
  state = |state, options| {
    state.put::<ModuleSpecifier>(options.main_module);
  },
);

#[op2]
#[string]
fn op_main_module(state: &mut OpState) -> Result<String, AnyError> {
  let main_url = state.borrow::<ModuleSpecifier>();
  let main_path = main_url.to_string();
  Ok(main_path)
}
