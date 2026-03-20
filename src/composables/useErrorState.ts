import { ref } from "vue";

const errorMessage = ref<string | null>(null);

export function useErrorState() {
  function clearError() {
    errorMessage.value = null;
  }

  function setError(error: unknown, fallback = "操作失败") {
    if (error instanceof Error) {
      errorMessage.value = error.message;
      return;
    }

    if (typeof error === "string") {
      errorMessage.value = error;
      return;
    }

    errorMessage.value = fallback;
  }

  return {
    errorMessage,
    clearError,
    setError,
  };
}
