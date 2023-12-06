// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use deno_core::op2;
use deno_core::OpState;

use crate::bootstrap::BootstrapOptions;

deno_core::extension!(
    swarmd_bootstrap,
    ops = [
        op_bootstrap_numcpus,
        op_bootstrap_user_agent,
        op_bootstrap_language,
    ],
);

#[op2(fast)]
#[smi]
pub fn op_bootstrap_numcpus(state: &mut OpState) -> u32 {
    state.borrow::<BootstrapOptions>().cpu_count as u32
}

#[op2]
#[string]
pub fn op_bootstrap_user_agent(state: &mut OpState) -> String {
    state.borrow::<BootstrapOptions>().user_agent.clone()
}

#[op2]
#[string]
pub fn op_bootstrap_language(state: &mut OpState) -> String {
    state.borrow::<BootstrapOptions>().locale.clone()
}
