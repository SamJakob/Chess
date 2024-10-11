import './board.scss';
import {ChessPiece} from './piece.tsx';
import {useGame} from '../providers/game.hooks.ts';
import {requireAsync} from '../utilities/async.ts';

const RANKS = ['1', '2', '3', '4', '5', '6', '7', '8'];
const FILES = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

export function ChessBoard() {
    const game = requireAsync(useGame());

    const fileInfoColumn = (
        <div className="file-info">
            <div/>
            {Array(game.board[0].length)
                .fill(undefined)
                .map((_, i) => (
                    <div key={i}>{FILES[i]}</div>
                ))}
            <div/>
        </div>
    );

    return (
        <div className="h-full flex flex-col justify-center items-center">
            <div className="chess-board">
                <div>
                    {fileInfoColumn}
                    {game.board.map((files, rank) => (
                        <div className="row" key={rank}>
                            <div className="rank-info">{RANKS[rank]}</div>
                            {files.map((piece, file) => (
                                <div
                                    key={file}
                                    className="square"
                                    data-file={file}
                                    data-file-name={FILES[file]}
                                    data-rank={rank}
                                    data-rank-name={RANKS[rank]}
                                    data-square-name={`${FILES[file]}${RANKS[rank]}`}>
                                    {piece ? (
                                        <ChessPiece kind={piece.kind} color={piece.color} position={[rank, file]}/>
                                    ) : (
                                        <span className="chess-piece-placeholder"></span>
                                    )}
                                </div>
                            ))}
                            <div className="rank-info">{RANKS[rank]}</div>
                        </div>
                    ))}
                    {fileInfoColumn}
                </div>
            </div>
        </div>
    );
}
