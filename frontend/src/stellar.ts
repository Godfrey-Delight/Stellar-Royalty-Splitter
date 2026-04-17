/**
 * Stellar SDK utilities for Freighter wallet integration
 */

/**
 * Sign and submit a transaction XDR with Freighter wallet
 */
export async function signAndSubmitTransaction(
  xdrString: string,
): Promise<string> {
  // @ts-ignore — Freighter injects window.freighter
  if (!window.freighter) {
    throw new Error("Freighter wallet not found");
  }

  try {
    // @ts-ignore
    const signedXdr = await window.freighter.signTransaction(xdrString, {
      network: "TESTNET", // or "PUBLIC" for mainnet
    });

    // For now, return a mock transaction hash
    // In production, you'd submit this to the Stellar network via RPC
    return "TX_" + Math.random().toString(36).substring(7);
  } catch (error) {
    throw new Error(
      `Failed to sign transaction: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
}

/**
 * Format an address for display (short form)
 */
export function formatAddress(address: string, length: number = 8): string {
  if (!address) return "";
  return address.substring(0, length) + "...";
}

/**
 * Format amount with decimals
 */
export function formatAmount(
  amount: string | number,
  decimals: number = 2,
): string {
  const num = typeof amount === "string" ? parseFloat(amount) : amount;
  if (isNaN(num)) return "0";
  return num.toLocaleString("en-US", {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  });
}
