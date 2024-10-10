import { useAPIContext } from '../providers/api.hooks.ts';
import { require } from '../utilities';
import { ChessBoard } from '../components/board.tsx';

export function MainScreen() {
	const api = require(useAPIContext());

	return (
		<div className="h-full w-full flex flex-col justify-center items-center">
			<div id="header" className="flex flex-col w-full text-center my-8">
				<h1 className="text-2xl font-extrabold">Chess</h1>
				<p>API Version: {api.version}</p>
			</div>
			<main>
				<ChessBoard />
			</main>
		</div>
	);
}
