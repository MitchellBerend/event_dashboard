let protocol = "http"

if (import.meta.env.PROD) {
	protocol += "s"
}

export const baseURL = `${protocol}://${import.meta.env.VITE_BACKEND_DOMAIN}`;
