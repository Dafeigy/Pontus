<script setup lang="ts">
import { Toaster } from "vue-sonner";
</script>

<template>
  <Toaster
    rich-colors
    close-button
    :toast-options="{
      classes: {
        toast: 'group-[.toaster]:bg-background group-[.toaster]:text-foreground group-[.toaster]:border-border group-[.toaster]:shadow-lg',
        description: 'group-[.toast]:text-muted-foreground',
      },
    }"
  />
</template>
