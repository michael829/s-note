import { ref } from "vue";

const visible = ref(false);
const message = ref("");
let timer: ReturnType<typeof setTimeout> | null = null;

export function useToast() {
  function show(nextMessage: string, duration = 1500) {
    message.value = nextMessage;
    visible.value = true;

    if (timer) {
      clearTimeout(timer);
    }

    timer = setTimeout(() => {
      visible.value = false;
    }, duration);
  }

  function hide() {
    visible.value = false;
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
  }

  return {
    visible,
    message,
    show,
    hide,
  };
}
