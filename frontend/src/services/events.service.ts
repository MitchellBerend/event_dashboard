import { baseURL } from "@/constants";


interface Event {
	reference: string,
	createdAt: Date,
}
const authHeader = {"Authorization": `Bearer fakeToken`};

class EventService {
	async getEvents(): Promise<Array<Event>> {
		const response = await fetch(`${baseURL}/events`, {headers: authHeader});
		const result: Array<Event> = await response.json();
		return result;
	}

	async getEvent(id: string): Promise<Event> {
		const response = await fetch(`${baseURL}/event/${id}`, {headers: authHeader});
		const result: Event = await response.json();
		return result;
	}
}

const eventService = new EventService();
export default eventService;
