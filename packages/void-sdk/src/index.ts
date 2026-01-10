/**
 * Void SDK - Confidential DeFi Layer for Solana
 * 
 * TypeScript SDK for interacting with the Void Protocol's
 * encrypted on-chain state via Arcium MPC.
 * 
 * @packageDocumentation
 */

// Placeholder - Full implementation pending
// See PRD.md for planned exports:
// - encryptState()
// - swapConfidential()
// - transferPrivate()

export const VERSION = "0.1.0";

export interface VoidClientConfig {
    programId: string;
    cluster: "devnet" | "mainnet-beta" | "localnet";
}

/**
 * Main client for Void Protocol interactions.
 * Implementation pending Anchor program deployment.
 */
export class VoidClient {
    constructor(public config: VoidClientConfig) {
        console.log("[VoidSDK] Initialized with config:", config);
    }

    /**
     * Initialize a new private account.
     * @placeholder - Implementation pending
     */
    async initPrivateAccount(): Promise<string> {
        throw new Error("Not yet implemented - awaiting Anchor program");
    }
}

export default VoidClient;
