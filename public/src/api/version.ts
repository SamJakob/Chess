import axios from 'axios';

export async function getAPIVersion() {
	return (await axios.get<string>('/version')).data;
}
