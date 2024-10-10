import axios from 'axios';
import { Game } from './game.ts';

export async function move(game: Game, position: [number, number], newPosition: [number, number]) {
	await axios.post(`/game/${game.id}/${JSON.stringify(position)}/move`, newPosition);
}
