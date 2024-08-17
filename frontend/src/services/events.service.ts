import { baseURL } from "@/constants";


interface Event {
	reference: string,
	createdAt: Date,
}

class EventService {
	async getEvents(): Promise<Array<Event>> {
		const response = await fetch(`${baseURL}/events`);
		const result: Array<Event> = await response.json();
		return result;
	}

	async getEvent(id: string): Promise<Event> {
		const response = await fetch(`${baseURL}/event/${id}`);
		const result: Event = await response.json();
		return result;
	}
}

const eventService = new EventService();
export default eventService;
