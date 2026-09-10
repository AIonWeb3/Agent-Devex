import { mapStellarError } from "../src/stellar";

describe("stellar error mapping", () => {
  it("maps timeouts", () => {
    const err = mapStellarError(new Error("ETIMEDOUT"));
    expect(err.message).toMatch(/timeout/);
  });

  it("maps generic failures", () => {
    const err = mapStellarError(new Error("tx failed"));
    expect(err.message).toMatch(/transaction failed/);
  });
});
