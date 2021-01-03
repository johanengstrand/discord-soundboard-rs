import { currentNotification } from "./store";

export function createNotification(message, error = false) {
    currentNotification.set({
        message,
        error,
    });
}
