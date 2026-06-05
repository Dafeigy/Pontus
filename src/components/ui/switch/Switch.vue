<script setup lang="ts">
import { cn } from "@/lib/utils";

const model = defineModel<boolean>({ required: true });
const props = defineProps<{
  disabled?: boolean;
}>();

function toggle() {
  if (!props.disabled) {
    model.value = !model.value;
  }
}
</script>

<template>
  <button
    type="button"
    role="switch"
    :aria-checked="model"
    :disabled="disabled"
    :data-state="model ? 'checked' : 'unchecked'"
    :class="
      cn(
        'peer inline-flex h-[24px] w-[44px] shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50',
        model ? 'bg-primary' : 'bg-input',
      )
    "
    @click="toggle"
  >
    <span
      :class="
        cn(
          'pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform',
          model ? 'translate-x-5' : 'translate-x-0',
        )
      "
    />
  </button>
</template>
