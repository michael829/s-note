<script setup lang="ts">
defineProps<{
  message: string | null;
}>();
</script>

<template>
  <div v-if="message" class="error-banner" role="alert">
    {{ message }}
  </div>
</template>

<style scoped>
.error-banner {
  margin: 8px 0 0;
  padding: 8px 10px;
  border-radius: var(--radius-md);
  background: var(--color-danger-subtle, rgba(231, 76, 60, 0.08));
  color: var(--color-danger, #e74c3c);
  font-size: var(--font-size-xs, 12px);
  line-height: 1.4;
}
</style>
