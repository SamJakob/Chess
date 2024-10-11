import axios from 'axios';
import {APIDetails} from '../providers/api.hooks.ts';

export async function getAPIDetails() {
    return (await axios.get<APIDetails>('/details')).data;
}
