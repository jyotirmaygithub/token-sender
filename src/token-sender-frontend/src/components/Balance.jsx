import React, { useState } from "react";
import { Principal } from "@dfinity/principal";
import { token_sender_backend } from "../../../declarations/token-sender-backend";

function Balance() {
  const [principalId, setPrincipalId] = useState("");
  const [userBalance, setUserBalance] = useState(null);
  const [tokenSymbol, setTokenSymbol] = useState(null);

  async function handleClick() {
    try {
      const principal = Principal.fromText(principalId);

      const balance = await token_sender_backend.get_user_balance(principal);
      const symbol = await token_sender_backend.get_token_name();

      // setUserBalance(balance);
      setTokenSymbol(symbol);
      setUserBalance(balance.toLocaleString())

    } catch (error) {
      console.error("Error fetching balance or token symbol:", error);
    }
  }

  return (
    <div className="window white">
      <label>Check account token balance:</label>
      <p>
        <input
          id="balance-principal-id"
          type="text"
          placeholder="Enter a Principal ID"
          value={principalId}
          onChange={(e) => setPrincipalId(e.target.value)}
        />
      </p>
      <p className="trade-buttons">
        <button
          id="btn-request-balance"
          onClick={handleClick}
        >
          Check Balance
        </button>
      </p>
      <p>
        This account has a balance of {tokenSymbol ? tokenSymbol : 'Token symbol'} - {userBalance !== null ? userBalance : 'Balance not available'}.
      </p>
    </div>
  );
}

export default Balance;
