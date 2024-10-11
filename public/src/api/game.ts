import { Color, Kind } from '../components/piece.tsx';
import axios from 'axios';

export interface Piece {
	kind: Kind;
	color: Color;
	move_count: number;
}

export interface Game {
	/**
	 * The ID of the game.
	 */
	id: string;

	/**
	 * The state of the board.
	 */
	board: (Piece | undefined)[][];

	/**
	 * When the game started.
	 */
	created_at: number;

	/**
	 * The total number of moves that have been made in the game.
	 */
	moves_count: number;

	is_player_in_check: {
		/**
		 * Whether the white player is in check.
		 */
		W: boolean;

		/**
		 * Whether the black player is in check.
		 */
		B: boolean;
	};
}

export async function createGame() {
	return (await axios.put<Game>('/game')).data;
}

export async function getGame(game: Game) {
	return (await axios.get<Game>(`/game/${game.id}`)).data;
}
