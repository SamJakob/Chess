import { useAsync } from '../utilities/async.ts';
import { createGame, getGame } from '../api/game.ts';
import { GameContext, RefreshGameContext } from './game.hooks.ts';
import { PropsWithChildren } from 'react';

export function GameProvider({ children }: Readonly<PropsWithChildren>) {
	const [game, refreshGame] = useAsync(createGame, { refreshAction: getGame });

	return (
		<RefreshGameContext.Provider value={refreshGame}>
			<GameContext.Provider value={game}>{children}</GameContext.Provider>
		</RefreshGameContext.Provider>
	);
}
