<script lang="ts">
    import Refresh from "svelte-material-icons/Refresh.svelte";
    import Plus from "svelte-material-icons/Plus.svelte";
    import { createEventDispatcher, getContext } from "svelte";
    import { _ } from "svelte-i18n";
    import type { OpenChat } from "openchat-client";
    import type { ResourceKey } from "../../i18n/i18n";
    import Translatable from "../Translatable.svelte";

    const client = getContext<OpenChat>("client");
    const dispatch = createEventDispatcher();

    export let ledger: string;
    export let value: bigint;
    export let label: ResourceKey | undefined = undefined;
    export let bold = false;
    export let toppingUp = false;
    export let showTopUp = false;
    export let showRefresh = true;
    export let refreshing = false;
    export let conversion: "none" | "icp" | "usd" = "none";

    $: cryptoLookup = client.enhancedCryptoLookup;
    $: tokenDetails = $cryptoLookup[ledger];
    $: symbol = tokenDetails.symbol;
    $: formattedValue =
        conversion === "none"
            ? client.formatTokens(value, tokenDetails.decimals)
            : conversion === "icp"
              ? tokenDetails.icpBalance.toFixed(3)
              : tokenDetails.dollarBalance.toFixed(2);

    $: {
        if (ledger) {
            refresh();
        }
    }

    export function refresh() {
        dispatch("click");
        refreshing = true;

        return client
            .refreshAccountBalance(ledger)
            .then((val) => {
                dispatch("refreshed", val);
            })
            .catch((err) => {
                const errorMessage = $_("unableToRefreshAccountBalance", {
                    values: { token: symbol },
                });
                client.logError(`Failed to refresh ${symbol} account balance`, err);
                dispatch("error", errorMessage);
            })
            .finally(() => (refreshing = false));
    }

    function topUp() {
        toppingUp = !toppingUp;
    }
</script>

<div class="container">
    {#if label !== undefined}
        <div class="label"><Translatable resourceKey={label} /></div>
    {/if}
    <div class="amount" class:bold>
        {formattedValue}
    </div>
    {#if showRefresh}
        <div class="refresh" class:refreshing on:click={refresh}>
            <Refresh size={"1em"} color={"var(--icon-txt)"} />
        </div>
    {/if}
    {#if showTopUp}
        <div class="top-up" on:click={topUp} title={$_("cryptoAccount.topUp")}>
            <Plus size={"1em"} color={toppingUp ? "var(--icon-selected)" : "var(--icon-txt)"} />
        </div>
    {/if}
</div>

<style lang="scss">
    .container {
        display: flex;
        justify-content: flex-end;
        align-items: center;
        gap: 6px;
    }

    .top-up,
    .refresh {
        // We want the size of the refresh icon (1em) to be 24px
        // but we can't use rem units in SVGs
        @include font-size(fs-140);
        height: $sp5;
        width: $sp5;
        cursor: pointer;
        @include mobile() {
            height: 21.59px;
            width: 21.59px;
        }
    }

    .refresh {
        &.refreshing {
            @include spin();
        }
    }

    .label {
        @include font(bold, normal, fs-100, 22);
        color: var(--txt-light);
        font-weight: 400;
    }

    .amount {
        @include font(bold, normal, fs-100, 22);
    }
</style>
