const form = document.getElementById("demo-form");
const status = document.getElementById("form-status");

form?.addEventListener("submit", (event) => {
  event.preventDefault();
  const data = new FormData(form);
  const amount = data.get("amount");
  const agent = String(data.get("agent_address") || "");
  if (!/^-?\d+$/.test(String(amount)) || Number(amount) <= 0) {
    status.textContent = "Amount must be a positive integer string (i128).";
    status.dataset.state = "error";
    return;
  }
  if (!(agent.startsWith("G") || agent.startsWith("C")) || agent.length < 56) {
    status.textContent = "Agent address should look like a Stellar G/C account (56 chars).";
    status.dataset.state = "error";
    return;
  }
  status.textContent =
    "Payload looks valid. In production the MCP server signs execute_agent_action with STELLAR_SECRET_KEY.";
  status.dataset.state = "ok";
});
