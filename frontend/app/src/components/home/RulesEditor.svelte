<script lang="ts">
    import { _ } from "svelte-i18n";
    import TextArea from "../TextArea.svelte";
    import Legend from "../Legend.svelte";
    import Toggle from "../Toggle.svelte";
    import type { UpdatedRules, Level } from "openchat-client";
    import { afterUpdate } from "svelte";
    import { i18nKey, interpolate, type ResourceKey } from "../../i18n/i18n";
    import Translatable from "../Translatable.svelte";

    const MAX_RULES_LENGTH = 1024;

    export let rules: UpdatedRules;
    export let level: Level;
    export let valid: boolean;
    export let editing: boolean;

    let originalRules: UpdatedRules = { ...rules };

    $: isValid = !rules.enabled || (rules.text.length > 0 && rules.text.length <= MAX_RULES_LENGTH);
    $: rulesDirty = rules.text !== originalRules.text || rules.enabled !== originalRules.enabled;

    function buildRulesExplanation(level: Level): ResourceKey | undefined {
        switch (level) {
            case "community":
                return i18nKey("rules.communityRulesExplanation");
            case "channel":
                return i18nKey("rules.channelRulesExplanation");
            case "group":
                return undefined;
        }
    }

    function toggleRules() {
        rules.enabled = !rules.enabled;
    }

    function toggleNewVersion() {
        rules.newVersion = !rules.newVersion;
    }

    afterUpdate(() => {
        valid = isValid;
    });
</script>

<div class="rules" class:disabled={!rules.enabled}>
    <Toggle
        small
        id="enable-rules"
        on:change={toggleRules}
        label={i18nKey("rules.enable")}
        checked={rules.enabled} />
    <div class="instructions">
        <Translatable resourceKey={i18nKey("rules.instructions", undefined, level, true)} />
    </div>

    <Legend
        label={i18nKey("rules.levelRules", undefined, level)}
        rules={buildRulesExplanation(level)} />
    <TextArea
        bind:value={rules.text}
        minlength={0}
        maxlength={MAX_RULES_LENGTH}
        rows={8}
        placeholder={interpolate($_, i18nKey("rules.placeholder", undefined, level, true))} />
    {#if editing && rules.enabled}
        <Toggle
            id="new-version"
            on:change={toggleNewVersion}
            checked={rules.newVersion && rulesDirty}
            label={i18nKey("rules.promptExistingUsers")}
            disabled={!rulesDirty}
            small />

        <div class="instructions">
            <Translatable
                resourceKey={i18nKey(
                    "rules.promptExistingUsersInstructions",
                    undefined,
                    level,
                    true,
                )} />
        </div>
    {/if}
</div>

<style lang="scss">
    :global(.rules.disabled textarea) {
        color: var(--disabledTxt);
    }
    .rules {
        .instructions {
            @include font(book, normal, fs-80, 28);
            color: var(--txt-light);
            margin-bottom: $sp4;
        }
    }
</style>
