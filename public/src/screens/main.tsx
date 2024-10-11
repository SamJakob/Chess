import { useAPIDetails } from '../providers/api.hooks.ts';
import { require } from '../utilities';
import { ChessBoard } from '../components/board.tsx';
import { useGame } from '../providers/game.hooks.ts';
import { requireAsync } from '../utilities/async.ts';
import { useEffect, useState } from 'react';

import dayjs from 'dayjs';
import duration from 'dayjs/plugin/duration';

dayjs.extend(duration);

export function MainScreen() {
	const api = require(useAPIDetails());
	const game = requireAsync(useGame());

	const [offset, setOffset] = useState(0);

	useEffect(() => {
		const timeout = setInterval(() => setOffset(new Date().getTime() - game.created_at), 500);
		return () => clearInterval(timeout);
	}, [game.created_at]);

	return (
		<div className="h-full w-full flex flex-col justify-center items-center">
			<div id="header" className="flex flex-col w-full text-center my-8">
				<h1 className="text-2xl font-extrabold">Chess</h1>
				<p>API Version: {api.version}</p>
				<p>
					Duration: {dayjs.duration(offset).format('HH:mm:ss')} (Total Moves: {game.moves_count})
				</p>
				<p>Game ID: {game.id}</p>
				<p>Current Move: {game.current_move}</p>
				<p>
					In check? White: {game.is_player_in_check.W ? 'Yes' : 'No'}, Black:{' '}
					{game.is_player_in_check.B ? 'Yes' : 'No'}
				</p>
			</div>
			<main>
				<ChessBoard />
			</main>
		</div>
	);
}
