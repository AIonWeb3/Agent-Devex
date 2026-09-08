//! User-facing next-step copy after successful commands.

use crate::Lang;
use crate::output;

pub fn after_init(project_name: &str, lang: Lang) {
    output::success(format!("Created {project_name}/"));
    output::hint("  contracts/agent_pay_integration  Soroban + AgentPay/AgentGuard");
    match lang {
        Lang::Ts => {
            output::hint("  agent/                          TypeScript MCP server");
            output::hint(format!(
                "Next: cd {project_name} && agent-devex validate --project-dir ."
            ));
            output::hint(
                "      stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml",
            );
            output::hint("      cd agent && npm install && npx tsx src/index.ts");
        }
        Lang::Py => {
            output::hint("  agent/                          Python MCP server");
            output::hint(format!(
                "Next: cd {project_name} && agent-devex validate --project-dir ."
            ));
            output::hint(
                "      stellar contract build --manifest-path contracts/agent_pay_integration/Cargo.toml",
            );
            output::hint("      cd agent && uv sync && uv run python src/server.py");
        }
    }
}

pub fn after_deploy_without_account(wasm: &std::path::Path, network: &str) {
    output::warn(format!(
        "Built {}. Set STELLAR_ACCOUNT and re-run deploy, or run:",
        wasm.display()
    ));
    output::hint(format!(
        "  stellar contract deploy --network {network} --source-account <ACCOUNT> --wasm {}",
        wasm.display()
    ));
}

pub fn after_deploy_success(network: &str) {
    output::success(format!(
        "Deploy to {network} finished. Set AGENTPAY_CONTRACT_ID from the CLI output, then start the MCP server."
    ));
}
