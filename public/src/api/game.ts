import { Color, Kind } from '../components/piece.tsx';
import axios from 'axios';

export interface Piece {
	kind: Kind;
	color: Color;
	move_count: number;
}

export interface Game {
	id: string;
	board: (Piece | undefined)[][];
	created_at: number;
}

export async function createGame() {
	return (await axios.put<Game>('/game')).data;
}

export async function getGame(game: Game) {
	return (await axios.get<Game>(`/game/${game.id}`)).data;
}
