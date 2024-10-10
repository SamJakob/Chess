import './board.scss';
import { ChessPiece } from './piece.tsx';
import { useGame } from '../providers/game.hooks.ts';
import { requireAsync } from '../utilities/async.ts';

const RANKS = ['1', '2', '3', '4', '5', '6', '7', '8'];
const FILES = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

export function ChessBoard() {
	const game = requireAsync(useGame());

	const fileInfoColumn = (
		<tr className="file-info">
			<td />
			{Array(game.board[0].length)
				.fill(undefined)
				.map((_, i) => (
					<td key={i}>{FILES[i]}</td>
				))}
			<td />
		</tr>
	);

	return (
		<div className="h-full flex flex-col justify-center items-center">
			<table className="chess-board">
				<tbody>
					{fileInfoColumn}
					{game.board.map((files, rank) => (
						<tr key={rank}>
							<td className="rank-info">{RANKS[rank]}</td>
							{files.map((piece, file) => (
								<td
									key={file}
									className="square"
									data-file={file}
									data-file-name={FILES[file]}
									data-rank={rank}
									data-rank-name={RANKS[rank]}
									data-square-name={`${FILES[file]}${RANKS[rank]}`}>
									{piece ? (
										<ChessPiece kind={piece.kind} color={piece.color} position={[rank, file]} />
									) : (
										<></>
									)}
								</td>
							))}
							<td className="rank-info">{RANKS[rank]}</td>
						</tr>
					))}
					{fileInfoColumn}
				</tbody>
			</table>
		</div>
	);
}
