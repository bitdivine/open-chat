import type { Level } from "./structure";

export type AccessGate =
    | NoGate
    | NeuronGate
    | PaymentGate
    | DiamondGate
    | NftGate
    | CredentialGate;

export type NoGate = { kind: "no_gate" };

export type NftGate = { kind: "nft_gate" };

// TODO - this might end up being more complex e.g. a credential might not simply be a string it might be more complex like e.g. age > 18
export type CredentialGate = {
    kind: "credential_gate";
    issuerOrigin: string;
    credentialId: string;
};

export type NeuronGate = {
    kind: "neuron_gate";
    governanceCanister: string;
    minStakeE8s?: number;
    minDissolveDelay?: number;
};

export type PaymentGate = {
    kind: "payment_gate";
    ledgerCanister: string;
    amount: bigint;
    fee: bigint;
};

export function isNeuronGate(gate: AccessGate): gate is NeuronGate {
    return gate.kind === "neuron_gate";
}

export function isPaymentGate(gate: AccessGate): gate is PaymentGate {
    return gate.kind === "payment_gate";
}

export type DiamondGate = { kind: "diamond_gate" };

export type AccessControlled = {
    gate: AccessGate;
    public: boolean;
    frozen: boolean;
    historyVisible: boolean;
};

export type VersionedRules = Rules & {
    version: number;
};

export type Rules = {
    text: string;
    enabled: boolean;
};

export type UpdatedRules = Rules & {
    newVersion: boolean;
};

export function defaultChatRules(level: Level): VersionedRules {
    let text = "";

    if (level !== "channel") {
        text = `- Do not impersonate others in a deceptive or misleading manner
- Do not intentionally share false or misleading information
- Keep messages relevant to the ${level === "community" ? "channel" : "group"}

If you break the rules you might be blocked and/or have your message(s) deleted.`;
    }

    return {
        text,
        enabled: false,
        version: 0,
    };
}