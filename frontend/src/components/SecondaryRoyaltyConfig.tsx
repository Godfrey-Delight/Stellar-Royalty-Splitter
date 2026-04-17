import { useState } from "react";
import { api } from "../api";
import { signAndSubmitTransaction } from "../stellar.js";

interface Props {
  contractId: string;
  walletAddress: string;
  onSuccess: () => void;
  onRateUpdate?: (rate: number) => void;
}

export default function SecondaryRoyaltyConfig({
  contractId,
  walletAddress,
  onSuccess,
  onRateUpdate,
}: Props) {
  const [royaltyRate, setRoyaltyRate] = useState<string>("500"); // Default 5%
  const [status, setStatus] = useState<{
    type: "ok" | "error" | "info";
    msg: string;
  } | null>(null);
  const [loading, setLoading] = useState(false);

  async function submit() {
    if (!contractId) {
      return setStatus({ type: "error", msg: "Enter a contract ID first." });
    }

    const rate = parseInt(royaltyRate);
    if (isNaN(rate) || rate < 0 || rate > 10000) {
      return setStatus({
        type: "error",
        msg: "Royalty rate must be between 0 and 10000 basis points (0-100%).",
      });
    }

    setLoading(true);
    setStatus({ type: "info", msg: "Building transaction..." });

    try {
      const { xdr } = await api.setRoyaltyRate({
        contractId,
        walletAddress,
        royaltyRate: rate,
      });

      setStatus({ type: "info", msg: "Please sign the transaction..." });

      // Sign and submit transaction (this would require Freighter integration)
      const result = await signAndSubmitTransaction(xdr);

      setStatus({
        type: "ok",
        msg: `Royalty rate set to ${(rate / 100).toFixed(2)}%! TX: ${result}`,
      });

      onSuccess();
      // Update parent component with new rate
      if (onRateUpdate) {
        onRateUpdate(rate);
      }
    } catch (err) {
      setStatus({
        type: "error",
        msg: `Error: ${err instanceof Error ? err.message : "Unknown error"}`,
      });
    } finally {
      setLoading(false);
    }
  }

  const percentage = (parseInt(royaltyRate) / 100).toFixed(2);

  return (
    <div className="card">
      <h3>Set Secondary Royalty Rate</h3>
      <p className="description">
        Configure the percentage of resale proceeds to distribute to
        collaborators.
      </p>

      <div className="form-group">
        <label>Royalty Rate (basis points)</label>
        <div className="input-with-label">
          <input
            type="number"
            value={royaltyRate}
            onChange={(e) => setRoyaltyRate(e.target.value)}
            min="0"
            max="10000"
            step="100"
            disabled={loading}
          />
          <span className="rate-display">{percentage}%</span>
        </div>
        <small>1 bp = 0.01%, max 10000 bp (100%)</small>
      </div>

      {status && <div className={`message ${status.type}`}>{status.msg}</div>}

      <button onClick={submit} disabled={loading} className="btn-primary">
        {loading ? "Processing..." : "Set Royalty Rate"}
      </button>
    </div>
  );
}
