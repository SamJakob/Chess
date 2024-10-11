import { Color, Kind } from '../components/piece.tsx';
import axios from 'axios';

export type Player = 'W' | 'B';

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
	 * The current player to move.
	 */
	current_move: Player;

	/**
	 * The total number of moves that have been made in the game.
	 */
	moves_count: number;

	/**
	 * A map from player color (i.e., 'W' or 'B') to a boolean indicating whether they are currently in check.
	 */
	is_player_in_check: Record<Player, boolean>;
}

export async function createGame() {
	return (await axios.put<Game>('/game')).data;
}

export async function getGame(game: Game) {
	return (await axios.get<Game>(`/game/${game.id}`)).data;
}
